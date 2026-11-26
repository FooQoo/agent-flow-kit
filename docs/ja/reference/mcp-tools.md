# MCP ツールリファレンス

Agent Flow Kit（AFK）がエージェントに提供する MCP ツールは **全部で6つ** です。これに通常のファイル読み書きツール（`Read` / `Write` / `Edit`）を組み合わせることで、依頼の受付から完了検証までの一連の流れを扱います。

普段の対話で人間がこの JSON を手書きする必要はありません。ツールの動作を確かめたいときや、独自クライアントへ組み込むとき、エラーの原因を調べたいときの辞書として使ってください。

---

## 6つのツール一覧

| ツール名 | 役割 | DB 更新 |
| --- | --- | --- |
| [`record_request`](#record-request) | 新規の依頼や途中の追加要望を登録し、`.afk/requests/NNN.md` を用意する。 | あり |
| [`open_plan`](#open-plan) | 最近のプラン一覧を取得する、または特定プランの状態・タスク・次の手順を確認する。 | 読み取り専用 |
| [`update_plan`](#update-plan) | プランの準備・欠損復元・確定・開始承認・完了と、引き継ぎメモの保存を行う。 | あり（`recover` は不足ファイルの復元のみ） |
| [`update_question`](#update-question) | 確認質問の作成・質問文の確定・回答の確定を行う（3種類の `op`）。 | あり |
| [`start_next`](#start-next) | 次に進めるタスクを開始する（または前のセッションが止まった `Doing` タスクを引き継ぐ）。 | あり |
| [`finish_task`](#finish-task) | 受け入れ条件の番号と `## Result` を確認し、タスクを `Done` にする。 | あり |

すべてのツールは定義外のフィールドを受け付けません（`deny_unknown_fields`）。また、特定のプランに対する応答 JSON には、最上位に必ず `"plan_id"` と `"afk_root"` が付与されます。

---

<a name="record-request"></a>

## `record_request`

最初の依頼（新しいプランの作成）や、作業途中の追加・変更依頼を登録します。呼び出し後、エージェントは返されたパスの Markdown ファイルにあなたの指示を書き込みます。

### パラメータ

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `workspace` | `string` | 条件付き | プロジェクトの絶対パス。`AFK_NEEDS_WORKSPACE=1` 環境で初回呼び出しを行うときに指定します。 |
| `plan_id` | `integer` | 任意 | 省略すると新しいプランを作ります。既存プランに要望を足すときはその `plan_id` を渡します。 |
| `receipt` | `string` | 任意 | 再送防止用のキー（1〜100バイト）。通信リトライなどで同じ依頼に同じキーを渡すと、二重登録せず既存の受付結果を返します。 |

### 呼び出し例と応答

```json
{
  "workspace": "/Users/alice/projects/payments",
  "receipt": "req-initial-01"
}
```

```json
{
  "plan_id": 1,
  "afk_root": "/Users/alice/projects/payments",
  "request": {
    "id": 1,
    "path": "/Users/alice/projects/payments/.afk/requests/001.md"
  }
}
```

---

<a name="open-plan"></a>

## `open_plan`

`plan_id` を指定するとそのプランの詳しい状態を返し、省略すると最近のプランを新しい順に最大10件返します。

### パラメータ

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `workspace` | `string` | 条件付き | プロジェクトの絶対パス（未バインドのセッションでのみ必要）。 |
| `plan_id` | `integer` | 任意 | 確認したいプラン ID。省略すると最近のプラン一覧を返します。 |
| `tasks` | `boolean` | 任意 | 既定値 `false`。`plan_id` と一緒に `true` を渡すと、有効なタスク一覧と前提関係（`after`）も返します。 |
| `before` | `integer` | 任意 | 一覧取得時のページ送り用。`id < before` のプランを最大10件返します。 |
| `check_files` | `boolean` | 任意 | 既定値 `false`。`true` にすると、`afk.db` から参照されていない `.afk/` 内の `.md` ファイルを最大32件 `orphan_documents` に返します。 |

### 呼び出し例と応答（`PlanView`）

空の配列（`requests`, `questions`）や存在しない項目（`draft`, `tasks`）は JSON から省略されます。

```json
{
  "plan_id": 1,
  "tasks": true
}
```

```json
{
  "plan_id": 1,
  "afk_root": "/Users/alice/projects/payments",
  "plan": {
    "id": 1,
    "path": "/Users/alice/projects/payments/.afk/plans/001-r001.md"
  },
  "questions": [
    {
      "id": 1,
      "path": "/Users/alice/projects/payments/.afk/questions/001.md",
      "state": "waiting"
    }
  ],
  "tasks": [
    {
      "id": 1,
      "path": "/Users/alice/projects/payments/.afk/tasks/001.md",
      "status": "done"
    },
    {
      "id": 2,
      "path": "/Users/alice/projects/payments/.afk/tasks/002.md",
      "status": "todo",
      "after": [1]
    }
  ],
  "progress": {
    "plan_id": 1,
    "title": "トークン更新ガードの追加",
    "revision": 1,
    "status": "doing",
    "total": 2,
    "done": 1,
    "doing": 0,
    "next": {
      "action": "wait_for_answer",
      "id": 1,
      "title": "空トークンにはどの HTTP ステータスを返しますか？",
      "count": 1
    }
  }
}
```

※ `questions` の `state` は、公開前が `"draft"`、回答待ちが `"waiting"`、回答記入済み（プラン未反映）が `"answered"` になります。

---

<a name="update-plan"></a>

## `update_plan`

プランの作成から完了までの操作と、タスクの引き継ぎメモの保存を `change.op` で指定します。

### `op: "prepare"` — 新しいドラフトを用意する

新しいプラン文書（`plans/NNN-rMMM.md`）と、最大32件の新規タスク文書（`tasks/NNN.md`）を予約します。

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `revision` | `integer` | 必須 | 現在の `progress.revision`（初回は `0`）。不一致の場合はエラーになります。 |
| `add` | `integer[][]` | 必須 | 追加する各タスクの前提リスト（最大32タスク・各32前提）。残す既存タスクは正の ID（`1`, `2`）、同じ `add` 内の前の新タスクは負の番号（`-1`, `-2`）で指定します。 |
| `remove` | `integer[]` | 任意 | 既定値 `[]`。今回の改訂で外す **未完了** タスクの ID リスト（最大32件）。完了済み（`Done`）タスクは外せません。 |
| `replace_draft` | `integer` | 任意 | すでに開いているドラフトを破棄して作り直すときだけ、その `draft_id` を指定します。 |

```json
{
  "plan_id": 1,
  "change": {
    "op": "prepare",
    "revision": 1,
    "remove": [2],
    "add": [[1], [-1]]
  }
}
```

応答として `Draft`（`plan_id`, `draft_id`, `base_revision`, `plan_path`, `tasks`, `afk_root`）が返ります。

---

### `op: "recover"` — ドラフトの不足ファイルを復元する

作成途中で止まったり誤って消したりした公開中ドラフトの初期テンプレートファイルを再作成します。すでに存在するファイルは上書きしません。

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `draft_id` | `integer` | 必須 | 現在開いているドラフトの ID。 |

```json
{
  "plan_id": 1,
  "change": {
    "op": "recover",
    "draft_id": 2
  }
}
```

---

### `op: "commit"` — ドラフトを確定して有効化する

プランと新タスクのタイトル・`## Acceptance` を検証して仕様の SHA-256 ハッシュを記録し、`revision` を 1 進めます。

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `draft_id` | `integer` | 必須 | 確定する公開中ドラフトの ID（確定済みの現行ドラフトに再実行してもエラーにならず現在の `Progress` を返します）。 |
| `requests` | `integer[]` | 必須 | 今回の改訂で反映した **すべての未反映リクエスト ID**。 |
| `answers` | `integer[]` | 必須 | 今回の改訂で反映した **すべての未反映・回答済み質問 ID**。 |
| `execute` | `boolean` | 任意 | 既定値 `false`。`Backlog` プランで `true` を渡すとすぐ `Todo` へ進み、`false` なら `authorize` されるまで `Backlog` で待機します。 |

```json
{
  "plan_id": 1,
  "change": {
    "op": "commit",
    "draft_id": 1,
    "requests": [1],
    "answers": [1],
    "execute": true
  }
}
```

更新後の [`Progress`](#progress-と-nextaction-の構造) を返します。

---

### `op: "authorize"` — `Backlog` のプランを開始可能にする

計画のみ確定して `Backlog` で待っていたプランに対し、人間から開始の指示が出たあと `Todo` へ進めます。

```json
{
  "plan_id": 1,
  "change": {
    "op": "authorize"
  }
}
```

---

### `op: "finish"` — プラン全体を完了にする

有効なタスクがすべて `Done` になったあと、プラン自身の `## Acceptance` 全項目と空でない `## Result` を確認してプランを `Done` にします。

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `revision` | `integer` | 必須 | 現在の `progress.revision`。 |
| `verified` | `integer[]` | 必須 | プランの `## Acceptance` の全項目に対応する 1 始まりの番号リスト（`[1, 2, ...]`）。 |

```json
{
  "plan_id": 1,
  "change": {
    "op": "finish",
    "revision": 2,
    "verified": [1, 2]
  }
}
```

### `op: "save_handoff"` で引き継ぎメモを保存する

`task_id`、`start_next` が返した現在の `attempt`、最大 16 KiB の Markdown `note` を渡します。保存できるのは進行中のタスクを担当するセッションです。保存すると最新のメモが更新され、タスクは Doing のままです。別セッションからは `open_plan` に `tasks: true` を渡すと、タスクの `handoff` から読めます。

---

<a name="update-question"></a>

## `update_question`

確認質問（`.afk/questions/NNN.md`）を3種類の `change.op` で管理します。

| `change.op` | 追加フィールド | 動作 |
| --- | --- | --- |
| `"prepare"` | `task_id?: integer`, `draft_id?: integer` | 質問テンプレートから `.afk/questions/NNN.md` を作成します。未完了の `task_id` か公開中の `draft_id` のどちらか一方に紐付けるか、両方省略できます。 |
| `"publish"` | `question_id: integer` | タイトルを確認し、`## Answer` より上の質問文の SHA-256 ハッシュを確定します。 |
| `"answer"` | `question_id: integer` | 質問文が変わっていないことを確認し、`## Answer` の下に書かれた回答の SHA-256 ハッシュを確定します。 |

```json
{
  "plan_id": 1,
  "change": { "op": "prepare", "draft_id": 1 }
}
```

```json
{
  "plan_id": 1,
  "change": { "op": "publish", "question_id": 1 }
}
```

```json
{
  "plan_id": 1,
  "change": { "op": "answer", "question_id": 1 }
}
```

---

<a name="start-next"></a>

## `start_next`

次に進めるべきタスクを1件選んで開始します。進行中（`Doing`）のタスクがあればそれを最優先し、なければ前提タスクがすべて `Done` になっている最小 ID の `Todo` タスクを選びます。

### パラメータ

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `plan_id` | `integer` | 必須 | 対象のプラン ID。 |
| `take_over` | `boolean` | 任意 | 既定値 `false`。前の MCP セッションが終了して残った `Doing` タスクを引き継ぐときだけ `true` にします。 |
| `model` | `string` | 任意 | 分かる場合に実行モデル名を指定します。タスクの実行履歴に MCP クライアント名と併記されます。 |

### 呼び出し例と応答（`TaskRun`）

```json
{
  "plan_id": 1,
  "take_over": false
}
```

```json
{
  "plan_id": 1,
  "afk_root": "/Users/alice/projects/payments",
  "id": 1,
  "path": "/Users/alice/projects/payments/.afk/tasks/001.md",
  "attempt": 1,
  "status": "doing"
}
```

---

<a name="finish-task"></a>

## `finish_task`

タスクの `## Acceptance` の全項目を確かめ、`## Result` に結果を書き込んだあと、タスクを `Done` にします。

### パラメータ

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `plan_id` | `integer` | 必須 | 対象のプラン ID。 |
| `task_id` | `integer` | 必須 | 実行中タスクの ID。 |
| `attempt` | `integer` | 必須 | `start_next` が返した `attempt` 番号。 |
| `verified` | `integer[]` | 必須 | タスクの `## Acceptance` 全項目に対応する 1 始まりの番号リスト（`[1, 2, ...]`）。 |

```json
{
  "plan_id": 1,
  "task_id": 1,
  "attempt": 1,
  "verified": [1, 2]
}
```

---

## `Progress` と `NextAction` の構造

`commit`・`authorize`・`finish_task`・`update_plan(finish)`・`open_plan` が返す `Progress` オブジェクトの内容は次のとおりです。

| フィールド | 型 | 意味 |
| --- | --- | --- |
| `plan_id` | `integer` | プラン番号。 |
| `title` | `string` | プレーンテキストのタイトル（初回確定前は `"Plan #<id>"`）。 |
| `revision` | `integer` | 確定済みのリビジョン番号（初回確定前は `0`）。 |
| `status` | `string` | `"backlog"`, `"todo"`, `"doing"`, `"done"` のいずれか。 |
| `total` | `integer` | 現在のリビジョンで有効なタスク総数。 |
| `done` | `integer` | `"done"` になった有効タスク数。 |
| `doing` | `integer` | 現在 `"doing"` の有効タスク数（`0` または `1`）。 |
| `next` | `NextAction` | 次に取るべき最優先アクション（`"action"` タグ付き）。 |

### `NextAction` 全10種類（優先順位順）

| `next.action` | 追加フィールド | 状態の意味 |
| --- | --- | --- |
| `"wait_for_answer"` | `id`, `title`, `count` | 公開済みの質問に対する人間の回答を待っています。 |
| `"prepare_question"` | `id` | 質問ファイルが作られましたが、まだ `publish` されていません。 |
| `"replan"` | `requests`, `answers` | 未反映の依頼または回答があるため、計画への取り込みが必要です。 |
| `"confirm_draft"` | `id` | 準備中のドラフトを `commit` する必要があります。 |
| `"complete"` | — | プラン全体が `Done` で、保留中の変更もありません。 |
| `"prepare_plan"` | — | 新規プラン（`revision == 0`）の初回ドラフト作成が必要です。 |
| `"await_start"` | — | 計画のみ確定した `Backlog` 状態で、開始指示（`authorize`）を待っています。 |
| `"resume_task"` | `id`, `title` | `Doing` 中のタスクがあります（作業続行または引き継ぎ）。 |
| `"start_task"` | `id`, `title` | 着手可能な `Todo` タスクがあります（`start_next` を呼べます）。 |
| `"verify_plan"` | — | 全タスクが完了しました。プラン自身の完了条件を確認して `finish` できます。 |

---

## よくあるエラーと対処法

| エラーメッセージ | 原因 | 対処法 |
| --- | --- | --- |
| `provide the current project directory as workspace to record_request or open_plan` | `AFK_NEEDS_WORKSPACE=1` が有効なのに初回呼び出しに `workspace` がない。 | `record_request` または `open_plan` に `"workspace": "/プロジェクトの絶対パス"` を渡します。 |
| `AFK is already bound to ...; start a new MCP session for another workspace` | 現在の MCP プロセスがすでに別のプロジェクトに紐付いている。 | 別プロジェクトを扱うときは新しいセッションを開きます。 |
| `open draft changed; inspect it, recover its files or replace it by ID` | すでに公開中ドラフトがある状態で `replace_draft` なしの `prepare` を呼んだ。 | `open_plan` で既存ドラフトを確認し、そのまま確定・`recover` するか、`replace_draft` にその ID を渡します。 |
| `request set changed; inspect and reflect all pending requests`（または `question set changed...`） | `commit` に渡した `requests` / `answers` が DB 上の未反映 ID と一致していない。 | `open_plan` で未反映の依頼・回答をすべて確認し、本文に反映したうえで全 ID を渡します。 |
| `replace each retained task whose prerequisite is being removed` | `remove` で外すタスクを前提にしている未完了タスクが残っている。 | 依存先の未完了タスクも一緒に `remove` に入れ、新しい前提で `add` に追加し直します。 |
| `resolve questions and reflect pending requests or answers in a confirmed plan` | 未解決の質問や未反映の依頼があるままタスク開始・完了を呼んだ。 | 質問への回答・封印と、ドラフトの `commit` を先に済ませます。 |
| `a different session owns this task; take over only after its worker stops` | 別セッションが開始した `Doing` タスクに対し `take_over: true` なしで `start_next` を呼んだ。 | 前のセッションが終わっていることを確認し、`"take_over": true` を付けて呼びます。 |
| `task attempt changed; inspect before continuing` | 古い `attempt` 番号または別セッションから `finish_task` を呼んだ。 | `open_plan` で最新状態を確認してから続行します。 |
| `verify each of the N acceptance bullets by its one-based position` | `verified` が `1..=N` の連番になっていない。 | `## Acceptance` の全項目を確認し、`[1, 2, ..., N]` を漏れなく渡します。 |
| `confirmed text changed at ...; preserve your edits elsewhere, restore the confirmed text and replan in a new draft` | 確定後のプランやタスクの `.md` が直接編集された。 | 変更内容を別ファイルへ控え、元の確定済みテキストに戻してから、新しいドラフトで改訂します。 |

次は `.afk/` 内の各ファイルの書き方やテンプレート、ハッシュ計算の範囲をまとめた **[Markdown 形式とテンプレート](file-format.md)** を確認してください。
