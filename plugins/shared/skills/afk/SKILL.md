---
name: afk
description: Plan and carry out multi-step work with Agent Flow Kit, resume a plan, or save a handoff with the handoff argument.
disable-model-invocation: true
---

Follow the user's requested outcome and acceptance conditions. For the first AFK call in a session, pass the absolute current project directory as `workspace` to `record_request` or `open_plan` if the client has not bound it.

## Handoff

When the user invokes AFK with the argument `handoff`, or asks to prepare a handoff, use this command before the planning workflow. Treat it as an operation on the current task. Skip request registration and replanning.

1. Use the plan ID, task ID, and attempt returned by the latest `start_next` for the task this session is still working on. If the session has no identifiable current attempt, tell the user and stop the command. Use the existing attempt without starting or taking over a task.
2. Reach a safe stopping point. Call `update_plan` with `plan_id` and `change: {op: "save_handoff", task_id, attempt, note}`. In the note include completed work, changed files or worktree, checks performed and pending, and the next action.
3. When AFK confirms the save, tell the user which task has a checkpoint. Leave it Doing with its current owner. If AFK rejects the attempt, report the error and stop the command.

Refresh the checkpoint at meaningful milestones in long-running work so a sudden stop still leaves useful context. When resuming, use `open_plan` with `tasks: true` to read the last saved handoff.

## Plan and execute

1. Use `record_request` for the original instruction and each added or changed instruction. Use the existing plan ID for changes. Write the human's instruction at the returned path. To resume, use `open_plan`. With no plan ID it lists recent plans. Select the one the human means.
2. Prepare a revision with `update_plan` / `prepare`. Supply an `add` array with one list of prerequisites per new task. For example, `[[], [-1]]` makes the second new task wait for the first. Use positive IDs for existing prerequisites. For replanning, use `open_plan` with `tasks: true` and list superseded unfinished tasks in `remove`. Keep accepted Markdown and completed tasks as evidence. Use `recover` for missing files of an existing open draft.
3. Use standard file tools to write the returned plan and task files. Give each a `# Title` on the first line, scope or work details, and an `## Acceptance` section of ordered `- ` bullets. Describe the outcome and verification. Keep `## Result` at the end for completion evidence.
4. Ask only when intent, a human preference, irreversible action, or a scope change needs a human decision. Prepare a question, write its prompt and any options in Markdown, publish it, and ask the human in chat. Append their reply beneath `## Answer`, call `answer`, and reflect it in the draft. Ask again if an important uncertainty remains.
5. Commit the draft with the IDs of every reflected pending request and answer. Use `execute: true` if the user asked for execution. If they asked for planning only, leave it in Backlog until they explicitly tell you to start, then `authorize`. When a new request arrives, pause at a safe checkpoint and reconcile it before further execution.
6. Use `start_next`, read the selected task, and do the work with your usual tools. Supply `model` when the executing model is known. AFK maintains the task's `Execution agents` section, so preserve it when editing the task. After checking every acceptance bullet, write the evidence under `## Result` and call `finish_task` with the returned attempt and the one-based verified bullet numbers. Use `take_over` only when the former worker has stopped. Verify the plan's own acceptance bullets and record its outcome before calling `update_plan` / `finish`.
7. Give the user the requested answer or result. The Stop hook displays routine progress automatically.
