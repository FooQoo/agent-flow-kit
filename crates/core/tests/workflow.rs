use std::{fs, path::Path, sync::Barrier};

use afk_core::{Column, NextAction, Store};
use tempfile::TempDir;

fn write(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
}

const SPEC: &str = "# Example\n\nWork details.\n\n## Acceptance\n- Verify output.\n";
const RESULT: &str = "\n## Result\nVerified by test.\n";

#[test]
fn clarification_replan_dependency_and_resume() {
    let root = TempDir::new().unwrap();
    let store = Store::initialize(root.path()).unwrap();
    let intake = store.record_request(None, Some("request-1")).unwrap();
    let plan = intake.plan_id;
    assert_eq!(
        store
            .record_request(None, Some("request-1"))
            .unwrap()
            .request
            .id,
        intake.request.id
    );
    assert!(store.authorize_execution(plan).is_err());
    write(&intake.request.path, "Plan the example.");
    let draft = store
        .prepare_plan(plan, 0, &[vec![], vec![-1]], &[], None)
        .unwrap();
    let first = draft.tasks[0].clone();
    let second = draft.tasks[1].clone();
    assert!(
        store
            .commit_plan(plan, draft.draft_id, &[intake.request.id], &[], false)
            .is_err()
    );
    assert_eq!(store.progress(plan).unwrap().revision, 0);
    write(&draft.plan_path, SPEC);
    write(&first.path, SPEC);
    write(&second.path, SPEC);

    let question = store
        .prepare_question(plan, None, Some(draft.draft_id))
        .unwrap();
    write(&question.path, "# Output format?\n\nChoose JSON or text.\n");
    store.publish_question(plan, question.id).unwrap();
    let snapshot = store.progress(plan).unwrap();
    assert!(matches!(
        snapshot.next,
        NextAction::WaitForAnswer { count: 1, .. }
    ));
    assert!(
        store
            .commit_plan(
                plan,
                draft.draft_id,
                &[intake.request.id],
                &[question.id],
                false
            )
            .is_err()
    );
    write(
        &question.path,
        "# Output format?\n\nChoose JSON or text.\n\n## Answer\nJSON.\n",
    );
    store.answer_question(plan, question.id).unwrap();
    let committed = store
        .commit_plan(
            plan,
            draft.draft_id,
            &[intake.request.id],
            &[question.id],
            false,
        )
        .unwrap();
    assert_eq!(committed.status, Column::Backlog);
    assert!(store.start_next(plan, "worker", false).is_err());
    store.authorize_execution(plan).unwrap();
    let started = store.start_next(plan, "worker", false).unwrap();
    assert_eq!(started.task.id, first.id);
    assert!(store.start_next(plan, "other", false).is_err());
    assert!(
        store
            .finish_task(plan, first.id, "worker", started.attempt, &[1])
            .is_err()
    );
    write(&first.path, &format!("{SPEC}{RESULT}"));
    let complete = store
        .finish_task(plan, first.id, "worker", started.attempt, &[1])
        .unwrap();
    assert!(
        fs::read_to_string(&first.path)
            .unwrap()
            .contains("Attempt 1:")
    );
    assert_eq!(complete.done, 1);
    assert!(matches!(complete.next, NextAction::StartTask { id, .. } if id == second.id));
    let started = store.start_next(plan, "worker", false).unwrap();
    let extra = store.record_request(Some(plan), None).unwrap();
    write(&extra.request.path, "Add a second output format.");
    assert!(
        store
            .finish_task(plan, second.id, "worker", started.attempt, &[1])
            .is_err()
    );
    let update = store
        .prepare_plan(plan, 1, &[vec![first.id]], &[second.id], None)
        .unwrap();
    assert!(store.prepare_plan(plan, 1, &[], &[], None).is_err());
    write(&update.plan_path, SPEC);
    write(&update.tasks[0].path, SPEC);
    let revised = store
        .commit_plan(plan, update.draft_id, &[extra.request.id], &[], true)
        .unwrap();
    assert_eq!((revised.done, revised.total, revised.revision), (1, 2, 2));
    assert!(
        store
            .finish_task(plan, second.id, "worker", started.attempt, &[1])
            .is_err()
    );
    assert!(first.path.exists() && second.path.exists() && draft.plan_path.exists());

    let moved = TempDir::new().unwrap();
    let new_root = moved.path().join("new path");
    fs::rename(root.path(), &new_root).unwrap();
    let resumed = Store::initialize(&new_root).unwrap();
    let active = resumed.start_next(plan, "new-worker", false).unwrap();
    assert!(
        active
            .task
            .path
            .starts_with(new_root.canonicalize().unwrap())
    );
    assert!(resumed.finish_plan(plan, 2, &[1]).is_err());
    write(&active.task.path, &format!("{SPEC}{RESULT}"));
    assert!(
        resumed
            .finish_task(plan, active.task.id, "new-worker", active.attempt, &[])
            .is_err()
    );
    resumed
        .finish_task(plan, active.task.id, "new-worker", active.attempt, &[1])
        .unwrap();
    let active_plan = resumed.inspect(plan, false).unwrap().plan.unwrap();
    write(&active_plan.path, &format!("{SPEC}{RESULT}"));
    assert_eq!(
        resumed.finish_plan(plan, 2, &[1]).unwrap().status,
        Column::Done
    );
    assert_eq!(resumed.finish_plan(plan, 2, &[1]).unwrap().done, 2);
}

#[test]
fn concurrent_workers_sealed_files_and_stale_attempts() {
    let root = TempDir::new().unwrap();
    let store = Store::initialize(root.path()).unwrap();
    let intake = store.record_request(None, None).unwrap();
    let plan = intake.plan_id;
    write(&intake.request.path, "Execute the example.");
    let draft = store.prepare_plan(plan, 0, &[vec![]], &[], None).unwrap();
    write(&draft.plan_path, SPEC);
    let task = &draft.tasks[0];
    write(&task.path, SPEC);
    store
        .commit_plan(plan, draft.draft_id, &[intake.request.id], &[], true)
        .unwrap();
    write(
        &task.path,
        "# Altered\n\n## Acceptance\n- Altered criteria.\n",
    );
    assert!(store.start_next(plan, "worker", false).is_err());
    write(&task.path, SPEC);

    let ready = Barrier::new(2);
    let owners = ["left", "right"];
    let winner = std::thread::scope(|scope| {
        let handles = owners.map(|owner| {
            let ready = &ready;
            let store = &store;
            scope.spawn(move || {
                ready.wait();
                store.start_next(plan, owner, false)
            })
        });
        let results = handles.map(|handle| handle.join().unwrap());
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        results
            .into_iter()
            .zip(owners)
            .find_map(|(result, owner)| result.ok().map(|run| (run, owner)))
            .unwrap()
    });
    let resumed = store.start_next(plan, "resumed", true).unwrap();
    assert!(resumed.attempt > winner.0.attempt);
    write(&task.path, &format!("{SPEC}{RESULT}"));
    assert!(
        store
            .finish_task(plan, task.id, winner.1, winner.0.attempt, &[1])
            .is_err()
    );
    store
        .finish_task(plan, task.id, "resumed", resumed.attempt, &[1])
        .unwrap();
    write(&task.path, &format!("{SPEC}\n## Result\nChanged evidence."));
    write(&draft.plan_path, &format!("{SPEC}{RESULT}"));
    assert!(store.finish_plan(plan, 1, &[1]).is_err());
}

#[cfg(unix)]
#[test]
fn rejects_symlinked_state_and_documents() {
    use std::os::unix::fs::symlink;

    let root = TempDir::new().unwrap();
    let outside = TempDir::new().unwrap();
    symlink(outside.path(), root.path().join(".afk")).unwrap();
    assert!(Store::initialize(root.path()).is_err());
    fs::remove_file(root.path().join(".afk")).unwrap();
    let store = Store::initialize(root.path()).unwrap();
    let intake = store.record_request(None, None).unwrap();
    let target = outside.path().join("request.md");
    write(&target, "Outside the project");
    fs::remove_file(&intake.request.path).unwrap();
    symlink(&target, &intake.request.path).unwrap();
    let draft = store
        .prepare_plan(intake.plan_id, 0, &[vec![]], &[], None)
        .unwrap();
    write(&draft.plan_path, SPEC);
    write(&draft.tasks[0].path, SPEC);
    assert!(
        store
            .commit_plan(
                intake.plan_id,
                draft.draft_id,
                &[intake.request.id],
                &[],
                true
            )
            .is_err()
    );
    assert_eq!(fs::read_to_string(target).unwrap(), "Outside the project");
}

#[test]
fn recovers_reserved_files_and_reports_unowned_documents() {
    let root = TempDir::new().unwrap();
    let store = Store::initialize(root.path()).unwrap();
    let intake = store.record_request(None, None).unwrap();
    let draft = store
        .prepare_plan(intake.plan_id, 0, &[vec![]], &[], None)
        .unwrap();
    write(&draft.plan_path, SPEC);
    fs::remove_file(&draft.tasks[0].path).unwrap();
    let repaired = store.recover_draft(intake.plan_id, draft.draft_id).unwrap();
    assert_eq!(fs::read_to_string(&repaired.plan_path).unwrap(), SPEC);
    assert!(repaired.tasks[0].path.exists());
    let orphan = root
        .path()
        .canonicalize()
        .unwrap()
        .join(".afk/tasks/002.md");
    write(&orphan, "Preserve for recovery.");
    assert_eq!(store.orphan_documents().unwrap(), vec![orphan.clone()]);
    assert!(
        store
            .prepare_plan(intake.plan_id, 0, &[vec![]], &[], Some(draft.draft_id))
            .is_err()
    );
    assert_eq!(
        fs::read_to_string(orphan).unwrap(),
        "Preserve for recovery."
    );
    assert_eq!(
        store
            .inspect(intake.plan_id, false)
            .unwrap()
            .draft
            .unwrap()
            .draft_id,
        draft.draft_id
    );
}

#[test]
fn initial_templates_are_written_and_unfilled_documents_are_rejected() {
    let root = TempDir::new().unwrap();
    let store = Store::initialize(root.path()).unwrap();

    let intake = store.record_request(None, Some("tpl-req")).unwrap();
    assert_eq!(fs::read_to_string(&intake.request.path).unwrap(), "# \n");

    let draft = store
        .prepare_plan(intake.plan_id, 0, &[vec![]], &[], None)
        .unwrap();
    let expected_work = "# \n\n## Acceptance\n\n- \n\n## Result\n";
    assert_eq!(fs::read_to_string(&draft.plan_path).unwrap(), expected_work);
    let task_template = fs::read_to_string(&draft.tasks[0].path).unwrap();
    assert!(task_template.contains("## Execution agents\n\nNo execution recorded."));
    assert!(task_template.ends_with("\n\n## Acceptance\n\n- \n\n## Result\n"));

    let question = store
        .prepare_question(intake.plan_id, None, Some(draft.draft_id))
        .unwrap();
    assert_eq!(
        fs::read_to_string(&question.path).unwrap(),
        "# \n\n## Answer\n"
    );

    // Unfilled templates must be rejected by existing validation guards.
    assert!(store.publish_question(intake.plan_id, question.id).is_err());
    assert!(
        store
            .commit_plan(
                intake.plan_id,
                draft.draft_id,
                &[intake.request.id],
                &[],
                true
            )
            .is_err()
    );

    // Filling plan and task but leaving request as unfilled template must still fail.
    write(&draft.plan_path, SPEC);
    write(&draft.tasks[0].path, SPEC);
    write(&question.path, "# Clarify?\n\n## Answer\nYes.\n");
    store.publish_question(intake.plan_id, question.id).unwrap();
    store.answer_question(intake.plan_id, question.id).unwrap();
    assert!(
        store
            .commit_plan(
                intake.plan_id,
                draft.draft_id,
                &[intake.request.id],
                &[question.id],
                true
            )
            .is_err()
    );

    // Filling the request succeeds, and retrying with the same receipt preserves it.
    write(&intake.request.path, "# Request\n\nDo the work.\n");
    let retried = store.record_request(None, Some("tpl-req")).unwrap();
    assert_eq!(
        fs::read_to_string(&retried.request.path).unwrap(),
        "# Request\n\nDo the work.\n"
    );
    store
        .commit_plan(
            intake.plan_id,
            draft.draft_id,
            &[intake.request.id],
            &[question.id],
            true,
        )
        .unwrap();
}

#[test]
fn execution_metadata_preserves_accepted_spec_and_handoff_ownership() {
    let root = TempDir::new().unwrap();
    let store = Store::initialize(root.path()).unwrap();
    let intake = store.record_request(None, None).unwrap();
    write(&intake.request.path, "Track the executing agent.");
    let plan = intake.plan_id;
    let draft = store.prepare_plan(plan, 0, &[vec![]], &[], None).unwrap();
    let task = &draft.tasks[0];
    write(&draft.plan_path, SPEC);
    write(&task.path, SPEC);
    store
        .commit_plan(plan, draft.draft_id, &[intake.request.id], &[], true)
        .unwrap();

    // Simulate opening the confirmed task from a v1 workspace.
    let connection = rusqlite::Connection::open(root.path().join(".afk/afk.db")).unwrap();
    connection
        .execute_batch("DROP TABLE task_runs; DROP TABLE handoffs; PRAGMA user_version=1;")
        .unwrap();
    drop(connection);
    let store = Store::initialize(root.path()).unwrap();

    let first = store
        .start_next_as(plan, "one", false, Some("Codex"), Some("gpt-example"))
        .unwrap();
    let original = fs::read_to_string(&task.path).unwrap();
    assert!(original.contains("Attempt 1: `Codex` (model: `gpt-example`)"));
    assert_eq!(
        store.start_next(plan, "one", false).unwrap().attempt,
        first.attempt
    );
    assert_eq!(fs::read_to_string(&task.path).unwrap(), original);
    store
        .save_handoff(
            plan,
            task.id,
            "one",
            first.attempt,
            "Next: run the fixture.",
        )
        .unwrap();
    assert_eq!(store.progress(plan).unwrap().status, Column::Doing);

    let second = store
        .start_next_as(plan, "two", true, Some("Claude Code"), None)
        .unwrap();
    assert_eq!(second.attempt, first.attempt + 1);
    let contents = fs::read_to_string(&task.path).unwrap();
    assert!(contents.contains("Attempt 1: `Codex`"));
    assert!(contents.contains("Attempt 2: `Claude Code` (model: `Unknown`)"));
    assert!(
        store
            .save_handoff(plan, task.id, "one", first.attempt, "Stale note.")
            .is_err()
    );
    write(&task.path, &format!("{contents}{RESULT}"));
    assert!(
        store
            .finish_task(plan, task.id, "one", first.attempt, &[1])
            .is_err()
    );
    store
        .finish_task(plan, task.id, "two", second.attempt, &[1])
        .unwrap();
    write(&draft.plan_path, &format!("{SPEC}{RESULT}"));
    store.finish_plan(plan, 1, &[1]).unwrap();
}
