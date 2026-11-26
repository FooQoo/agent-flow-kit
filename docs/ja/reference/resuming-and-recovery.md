# 再開・引き継ぎと復旧

長い仕事では、ターミナルを閉じたり、会話が長くなって新しいセッションに切り替えたりすることがよくあります。Agent Flow Kit（AFK）は状態を `.afk/afk.db` に、文章を `.afk/**/*.md` に残しているため、新しいセッションでもツール1回で続きが分かり、途中で止まった作業からも安全に復帰できます。

---

## 最近のプラン一覧を見る

新しい会話を開いてプランの番号（`plan_id`）が分からないときは、`plan_id` を省いて `open_plan` を呼びます（「最近のプランを見せて」と頼むだけで構いません）。

```json
{}
```

※ `AFK_NEEDS_WORKSPACE=1` の環境で、そのセッションの最初の呼び出しなら `"workspace": "/プロジェクトの絶対パス"` を添えます。

新しい順（`ORDER BY id DESC LIMIT 10`）に **最大10件** のプランが返り、それぞれのタイトル・版・状態・進捗・次にやること（`next`）が分かります。

さらに前のプランを見るときは、いま見えた一番小さい `plan_id` を `before` に入れます。

```json
{
  "before": 12
}
```

これで `id < 12` のプランが最大10件返ります。

---

## プランの中身を見て再開する

再開する `plan_id` が決まったら、`open_plan` で詳しい状態を読み込みます。前のチャットのやり取りを読み返す必要はありません。

```json
{
  "plan_id": 1,
  "tasks": true
}
```

返ってくる内容だけで、続きに必要な情報がすべて揃います。
- **`plan`：** いまの確定済みプランのパス（例：`.afk/plans/001-r002.md`）。まだ一度も確定していなければ `null`。
- **`draft`：** 書きかけの下書き（`open`）があるときだけ表示されます。
- **`requests`：** まだ計画に取り込んでいない依頼の一覧。
- **`questions`：** まだ計画に取り込んでいない質問の一覧（`"draft"` / `"waiting"` / `"answered"`）。
- **`tasks`：** （`tasks: true` のとき）有効なタスクの一覧と、それぞれの状態・前提タスク（`after`）。
- **`progress`：** タスク数の集計と、次にやるべき `next` アクション。

---

## 途中で止まった `Doing` タスクを引き継ぐ

たとえばセッション A がタスク `#2` を作業中（`status: "doing"`, `attempt: 1`）にしたあと、完了にする前にターミナルが閉じてしまったとします。

新しく開いたセッション B では、**別の `afk` プロセス** が新しい担当 ID を持って立ち上がります。`open_plan` を見ると、`next` はこうなっています。

```json
{
  "action": "resume_task",
  "id": 2,
  "title": "同時リフレッシュを1本に集約する"
}
```

### `take_over` と `attempt` が衝突を防ぐ仕組み

1. **同じセッションの中なら：** セッション A の中でもう一度 `start_next(plan_id: 1)` を呼んでも、そのまま `attempt: 1` が返ります。
2. **新しいセッションで `take_over` を付けずに呼ぶと：** まだ動いている別のセッションから誤って仕事を奪わないよう、既定の `take_over: false` ではエラーになります。
   > `a different session owns this task; take over only after its worker stops`
3. **前のセッションが終わっていることを確かめて引き継ぐ：** セッション A がすでに閉じていることを確認したら、`take_over: true` を付けて呼びます。
   ```json
   {
     "plan_id": 1,
     "take_over": true
   }
   ```
   これで担当がセッション B に移り、番号（`attempt`）が `1` から `2` に進んで、履歴に `task_resumed` が残ります。

```mermaid
sequenceDiagram
    participant S1 as セッション A（前の担当）
    participant DB as .afk/afk.db
    participant S2 as セッション B（新しい担当）

    S1->>DB: start_next(plan_id: 1)
    DB-->>S1: task_id: 2, attempt: 1
    Note over S1: セッション A が終了・停止
    S2->>DB: start_next(plan_id: 1, take_over: true)
    DB-->>S2: task_id: 2, attempt: 2
    opt もし止まっていたセッション A が遅れて動いた場合
        S1->>DB: finish_task(task_id: 2, attempt: 1)
        DB-->>S1: 拒否: "task attempt changed; inspect before continuing"
    end
    S2->>DB: finish_task(task_id: 2, attempt: 2, verified: [1, 2])
    DB-->>S2: タスク #2 → Done
```

`finish_task` は担当 ID と `attempt` の両方が一致しないと通らないため、引き継いだあとに古いセッションが遅れて完了を送ってきても結果を壊すことはありません。

---

## 引き継ぎメモを保存する

いまのタスクの引き継ぎメモを保存するには、作業中のセッションで次のように入力します。

- **Claude Code** — `/agent-flow-kit:afk handoff`
- **Codex** — `$agent-flow-kit:afk` を選び、`handoff` と入力します。

作業中のエージェントに「引き継ぎを準備して」と頼むこともできます。進んだ作業、変更ファイルや作業ツリー、検証状況、残件、次の一手が AFK に保存されます。タスクは現在の担当のまま Doing に留まります。

準備中に前の担当が停止した場合、`open_plan` に `tasks: true` を渡して最後に保存した `handoff` を読めます。メモと実際の作業ファイルを照合し、前の担当の停止を確認してから `start_next` に `take_over: true` を渡して再開します。長い作業では節目ごとにメモを保存すると、急に停止した場合も続きが分かります。

## 下書きの消えたファイルを戻す（`recover`）

`update_plan(prepare)` は、先に `afk.db` で番号を確保し、その直後にひな形の `.md` ファイルを作ります。

もしその途中でプロセスが止まったり、確定前の下書きファイルを誤って消してしまったりしたときは、`op: "recover"` を呼びます。

```json
{
  "plan_id": 1,
  "change": {
    "op": "recover",
    "draft_id": 2
  }
}
```

- **足りないファイル** だけが、初期テンプレートで作り直されます。
- **すでに残っているファイル** は一切上書きされないため、途中まで書いた文章が消えることはありません。

---

## 古い下書きを捨てて、新しく書き直す（`replace_draft`）

1つのプランで同時に開ける下書きは1件だけです。前のセッションが残した下書き（たとえば `draft_id: 2`）を仕上げるのではなく、白紙から組み直したいときは `replace_draft` を使います。

1. `open_plan(plan_id: 1)` でいまの `draft.draft_id`（`2`）と `progress.revision` を見ます。
2. `update_plan(prepare)` に `replace_draft: 2` を渡します。

```json
{
  "plan_id": 1,
  "change": {
    "op": "prepare",
    "revision": 1,
    "remove": [],
    "add": [[], [-1]],
    "replace_draft": 2
  }
}
```

古い下書き `#2` は破棄（`discarded`）扱いになり、新しく `#3`（`plans/001-r003.md`）が開きます。`#2` で書いた `.md` ファイル自体は消えずに残るので、必要な文章を写すこともできます。

---

## 管理外のファイルを見つける（`check_files: true`）

手で `.afk/` の中に `.md` ファイルを作ったり、別のバックアップから一部だけファイルを戻したりしたときは、`open_plan` に `check_files: true` を付けると見つけられます。

```json
{
  "plan_id": 1,
  "check_files": true
}
```

`afk.db` に登録されていない `.md` ファイルがあれば、最大32件まで `orphan_documents` に並びます。

> [!TIP]
> **管理外のファイルが次の番号をふさいでいるとき：** AFK が新しく `.afk/tasks/005.md` を作ろうとしたとき、すでにそこに管理外のファイルがあると、勝手に上書きせず `"unreferenced file ...; preserve it elsewhere before reserving this path"` で止まります。そのファイルを `.afk/` の外へ移してからやり直してください。

---

## 確定済みの Markdown を手で書き換えてしまったとき

一度確定（`commit`）した計画やタスクは、`## Result` より上の文章のハッシュが記録されています（完了後は `## Result` も固定されます）。

確定したあとのファイルをエディタで直接書き換えると、次の操作でこう表示されて止まります。

> `confirmed text changed at /path/to/.afk/tasks/002.md; preserve your edits elsewhere, restore the confirmed text and replan in a new draft`

このときは **`afk.db` には手を触れず**、次の3ステップで直します。

1. **書き足した文章を別の場所へ控える：** 変更したかった内容を `.afk/` の外のメモなどにコピーしておきます。
2. **ファイルを確定時の文章に戻す：** エディタの元に戻す操作や、`git checkout -- .afk/tasks/002.md`、バックアップなどを使って、元の文章に戻します。
3. **計画変更として正式に入れる：** `record_request(plan_id)` で変更を記録し、`update_plan(prepare)`（変えたい未完了タスクを `remove` に入れ、新しいタスクを `add` で足す）→ `commit` で新しい版として確定します。

---

## `.afk/` をバックアップ・移動するとき

`afk.db` の中のパスはすべて **`.afk/` からの相対パス**（`plans/001-r001.md` や `tasks/001.md`）で保存されています。次の3つを守れば、フォルダごとコピーするだけでバックアップや別フォルダへの移動ができます。

1. **作業が止まっているときにコピーする：** セッションを閉じているか、ターンが終わって書き込みがない状態でコピーします。
2. **`.afk/` フォルダ丸ごと一式で扱う：** `afk.db` と `plans/`・`tasks/`・`requests/`・`questions/` を必ずセットでコピー・復元します。片方だけを戻すと中身が食い違います。
3. **シンボリックリンクを使わず普通のファイルとしてコピーする：** AFK は `.afk/` 内のシンボリックリンクをすべて拒否するため、通常のコピー（`cp -R` など）を使ってください。

続いて、6つのツールの詳しい引数やエラー一覧を **[MCP ツールリファレンス](mcp-tools.md)** で確認できます。
