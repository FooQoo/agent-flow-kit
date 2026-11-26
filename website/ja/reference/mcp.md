---
title: MCP リファレンス
description: Agent Flow Kit が提供する 6 つの MCP ツール、Progress / NextAction の構造、主なエラーメッセージのリファレンス。
---

# MCP リファレンス

*AFK の MCP ツール呼び出し仕様・状態遷移・エラーメッセージを正確に確認したい上級利用者・開発者向けのリファレンスです。*

Agent Flow Kit（AFK）がエージェントに提供する MCP ツールは **全部で 6 つ** です。エージェント標準のファイル読み書きツール（`Read` / `Write` / `Edit`）と組み合わせることで、依頼受付から完了検証までを扱います。

通常の利用で人間が JSON を手書きする必要はありません。

## 6 つのツール一覧

| ツール名 | 役割 | DB 更新 |
| --- | --- | --- |
| [`record_request`](#record-request) | 新規の依頼や途中の追加要望を登録し、`.afk/requests/NNN.md` を用意する。 | あり |
| [`open_plan`](#open-plan) | 最近のプラン一覧を取得する、または特定プランの状態・タスク・次の手順を確認する。 | 読み取り専用 |
| [`update_plan`](#update-plan) | プランの準備・欠損復元・確定・開始承認・完了と、引き継ぎメモの保存を行う。 | あり（`recover` は不足ファイルの復元のみ） |
| [`update_question`](#update-question) | 確認質問の作成・質問文の確定・回答の確定を行う（3 種類の `op`）。 | あり |
| [`start_next`](#start-next) | 次に進めるタスクを開始する（または前のセッションが止まった `Doing` タスクを引き継ぐ）。 | あり |
| [`finish_task`](#finish-task) | 受け入れ条件の番号と `## Result` を確認し、タスクを `Done` にする。 | あり |

すべてのツールは定義外のフィールドを受け付けません（`deny_unknown_fields`）。特定のプランに対する応答 JSON には最上位に必ず `"plan_id"` と `"afk_root"` が付与されます。

## `record_request` {#record-request}

最初の依頼（新しいプランの作成）や作業途中の追加・変更依頼を登録します。呼び出し後、エージェントは返されたパスの Markdown に人間の指示を書き込みます。

### パラメータ

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `workspace` | `string` | 条件付き | プロジェクトの絶対パス。`AFK_NEEDS_WORKSPACE=1` 環境で初回呼び出し時に指定します。 |
| `plan_id` | `integer` | 任意 | 省略すると新しいプランを作成。既存プランへの追加・変更時はその `plan_id` を渡します。 |
| `receipt` | `string` | 任意 | 再送防止キー（1〜100 バイト）。同じ依頼の再送時に同じキーを渡すと二重登録を防ぎます。 |

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

## `open_plan` {#open-plan}

`plan_id` を指定するとそのプランの詳細を返し、省略すると最近のプランを新しい順に最大 10 件返します。

### パラメータ

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `workspace` | `string` | 条件付き | プロジェクトの絶対パス（未バインドのセッションのみ必要）。 |
| `plan_id` | `integer` | 任意 | 確認したいプラン ID。省略すると最近のプラン一覧を返します。 |
| `tasks` | `boolean` | 任意 | 既定値 `false`。`true` で有効タスク一覧・前提関係（`after`）・引き継ぎメモ（`handoff`）を含めます。 |
| `before` | `integer` | 任意 | 一覧のページ送り用（`id < before` を最大 10 件）。 |
| `check_files` | `boolean` | 任意 | 既定値 `false`。`true` で `afk.db` 未参照の `.md` を最大 32 件 `orphan_documents` に返します。 |

## `update_plan` {#update-plan}

`change.op` でプランのライフサイクル操作と引き継ぎメモの保存を行います。

| `change.op` | 必須フィールド | 任意フィールド | 動作 |
| --- | --- | --- | --- |
| `"prepare"` | `revision`, `add`（`integer[][]`, 最大 32 タスク・各 32 前提） | `remove`（未完了タスク ID のみ）, `replace_draft` | 新規プラン文書とタスク文書を予約します。`add` 内の正の ID は残す既存タスク、負の 1 始まり番号（`-1`, `-2`）は同じ `add` 内の前の新タスクを指します。 |
| `"recover"` | `draft_id` | — | 公開中ドラフトの不足テンプレートファイルを再作成します（既存ファイルは上書きしません）。 |
| `"commit"` | `draft_id`, `requests`, `answers` | `execute`（既定 `false`） | タイトルと `## Acceptance` を検証して SHA-256 ハッシュを固定し、`revision` を 1 進めます。`Backlog` で `execute: true` なら即 `Todo` へ移行します。 |
| `"authorize"` | — | — | 確定済み（`revision > 0`）の `Backlog` プランを `Todo` へ進めます。 |
| `"finish"` | `revision`, `verified` | — | 全有効タスクが `Done` のとき、プラン自身の `## Acceptance` 全項目と空でない `## Result` を確認して `Done` にします。 |
| `"save_handoff"` | `task_id`, `attempt`, `note`（最大 16 KiB） | — | 現在所有している `Doing` タスクの引き継ぎメモを保存・更新します。 |

## `update_question` {#update-question}

確認質問（`.afk/questions/NNN.md`）を 3 種類の `change.op` で管理します。

| `change.op` | 追加フィールド | 動作 |
| --- | --- | --- |
| `"prepare"` | `task_id?: integer`, `draft_id?: integer` | 質問ファイルを作成します。未完了の `task_id`・公開中の `draft_id` のいずれか一方に紐付けるか、両方省略できます。 |
| `"publish"` | `question_id: integer` | タイトルを確認し、`## Answer` より上の質問文の SHA-256 ハッシュを固定します。 |
| `"answer"` | `question_id: integer` | 質問文が未改変であることを確認し、`## Answer` 下の回答の SHA-256 ハッシュを固定します。 |

## `start_next` {#start-next}

次に進めるべきタスクを 1 件選んで開始します。`Doing` タスクがあればそれを返し、なければ前提がすべて `Done` の最小 ID `Todo` タスクを開始します。

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `plan_id` | `integer` | 必須 | 対象プラン ID。 |
| `take_over` | `boolean` | 任意 | 既定値 `false`。前セッションが停止した `Doing` タスクを引き継ぐときだけ `true` にします。 |
| `model` | `string` | 任意 | 実行モデル名（分かる場合に指定するとタスク実行履歴に記録されます）。 |

## `finish_task` {#finish-task}

タスクの `## Acceptance` 全項目を確認し、`## Result` に証跡を書き込んだあと `Done` にします。

| フィールド | 型 | 必須 | 説明 |
| --- | --- | --- | --- |
| `plan_id` | `integer` | 必須 | 対象プラン ID。 |
| `task_id` | `integer` | 必須 | 実行中タスクの ID。 |
| `attempt` | `integer` | 必須 | `start_next` が返した `attempt` 番号。 |
| `verified` | `integer[]` | 必須 | `## Acceptance` 全項目に対応する 1 始まりの番号リスト（`[1, 2, ..., N]`）。 |

## `Progress` と `NextAction` の構造 {#progress-と-nextaction-の構造}

| フィールド | 型 | 意味 |
| --- | --- | --- |
| `plan_id` | `integer` | プラン番号。 |
| `title` | `string` | プレーンテキストのタイトル（初回確定前は `"Plan #<id>"`）。 |
| `revision` | `integer` | 確定済みリビジョン番号（初回確定前は `0`）。 |
| `status` | `string` | `"backlog"`, `"todo"`, `"doing"`, `"done"` のいずれか。 |
| `total` / `done` / `doing` | `integer` | 現行リビジョンの有効タスク件数。 |
| `next` | `NextAction` | 次に取るべき最優先アクション（`"action"` タグ付き）。 |

### `NextAction` 全 10 種類（優先順位順）

| `next.action` | 追加フィールド | 状態の意味 |
| --- | --- | --- |
| `"wait_for_answer"` | `id`, `title`, `count` | 公開済みの質問に対する人間の回答待ち。 |
| `"prepare_question"` | `id` | 作成済みで未公開の質問がある。 |
| `"replan"` | `requests`, `answers` | 未反映の依頼または回答を計画へ取り込む必要がある。 |
| `"confirm_draft"` | `id` | 準備中ドラフトの `commit` が必要。 |
| `"complete"` | — | プラン全体が `Done` で保留中の変更もない。 |
| `"prepare_plan"` | — | 新規プラン（`revision == 0`）の初回ドラフト作成が必要。 |
| `"await_start"` | — | 確定済み `Backlog` で `authorize` 待ち。 |
| `"resume_task"` | `id`, `title` | `Doing` 中のタスクがある。 |
| `"start_task"` | `id`, `title` | 着手可能な `Todo` タスクがある。 |
| `"verify_plan"` | — | 全タスク完了済み。プラン自身の完了条件を検証して `finish` できる。 |

## よくあるエラーと対処法

| エラーメッセージ | 原因 | 対処法 |
| --- | --- | --- |
| `provide the current project directory as workspace to record_request or open_plan` | `AFK_NEEDS_WORKSPACE=1` 有効時に初回 `workspace` が未指定。 | `"workspace": "/プロジェクトの絶対パス"` を渡します。 |
| `AFK is already bound to ...; start a new MCP session for another workspace` | 別プロジェクトにすでに紐付いている。 | 別プロジェクトでは新しいセッションを開きます。 |
| `open draft changed; inspect it, recover its files or replace it by ID` | 公開中ドラフトがあるのに `replace_draft` なしで `prepare` した。 | `open_plan` で確認し、確定・`recover`・または `replace_draft` 指定で進めます。 |
| `request set changed; inspect and reflect all pending requests` | 未反映の依頼・回答 ID が `commit` に漏れがある。 | `open_plan` で全件確認し、本文へ反映したうえで全 ID を渡します。 |
| `a different session owns this task; take over only after its worker stops` | 別セッション所有の `Doing` タスクに `take_over: true` なしで `start_next` した。 | 前セッションの停止を確認して `"take_over": true` で呼びます。 |
| `verify each of the N acceptance bullets by its one-based position` | `verified` が `1..=N` と一致しない。 | `## Acceptance` の全項目を確認し `[1, ..., N]` を渡します。 |
| `confirmed text changed at ...` | 確定済み Markdown がハッシュ固定後に直接変更された。 | 変更内容を別へ控え、確定済みテキストへ戻してから新ドラフトで改訂します。 |

運用時の復旧手順は [トラブルシューティング](/ja/troubleshooting) を参照してください。
