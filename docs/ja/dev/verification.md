# 検証とベンチマーク

AFK では、コア状態遷移・マルチスレッド競合・stdio JSON-RPC 通信・ネイティブ Hook の境界条件を網羅する決定論的な自動テストと、実モデル接続時のワークフロー遵守率・トークン消費を比較測定するクライアント実験プロトコルを組み合わせて品質を検証しています。

## 検証済み環境サマリ

`2026-11-26` 時点のベースライン検証は **macOS `26.6.2` (Apple Silicon)**、**Rust `1.96.0`**、**Claude Code `2.1.331`**、**Codex `0.171.0`** 上で実施されました。

| 検証レイヤー | 検証方法 | 観測された結果 |
| --- | --- | --- |
| **コア & 永続化** | `cargo test --locked --workspace` | 全単体テストおよびワークフロー結合テストがパス（各テストの検証項目は下表参照）。 |
| **Stdio MCP & `--hook`** | `python3 tests/integration/stdio_workflow.py target/release/afk --trials 5` | リリースバイナリによる5回連続試行すべてで、成果物・依存順序・ルート解決・セッション/ターン分離・重複 `Stop` 抑止・SQLite 排他ロック時フェイルオープンをパス。 |
| **Claude Code (`2.1.331`)** | 空白と日本語を含むパスに展開したプラグインを `--plugin-dir` でロード | MCP サーバーがリクエストを受信し、AFK ターンの末尾で `Stop` が1件の情報レポートを表示。続く無関係なターンでは無音を維持。 |
| **Codex (`0.171.0`)** | ローカルマーケットプレイス（`afk-local`）から導入し、一時プロファイルで2つの Hook を信頼登録 | スキルが有効化され6つの MCP ツールを公開。対話型ターミナルと `app-server`（`hook/completed`）で `Stop` レポートを確認し、無関係な次ターンは無音を維持。 |
| **モデルコンテキスト非干渉** | 両実クライアントをスクリプト制御のループバック HTTP モデルエンドポイントへ接続 | AFK ターンはツール呼び出し1回＋完了応答1回のみで完結。`Stop` による追加モデルリクエストは **0回** であり、次ターンのモデル入力にもレポート文字列が含まれないことを確認。 |
| **パッケージング** | Apple Silicon 配布アーカイブ（`dist/SHA256SUMS`）を展開し `otool -L` で検査 | `5,614,016` bytes のバイナリに SQLite が静的バンドルされ、macOS 標準システムライブラリのみに動的リンクしていることを確認。 |

## Rust 単体テスト・ワークフローテストスイート

ワークスペース全体の Rust テストは1秒未満で完了します。

```sh
cargo test --locked --workspace
```

現在のコードベースには2つのターゲットに計 **10 個の決定論的テスト**が含まれています。

### 1. タイトル抽出単体テスト（`crates/core/src/plain_title.rs` — 5件）

| テスト関数 | 検証している不変条件 |
| --- | --- |
| `strips_emphasis_code_and_strikethrough` | インラインの `**`、`__`、`*`、`_`、`` ` ``、`~~` 記法を正確に除去すること。 |
| `strips_block_markers` | ネストした行頭の `#` 見出し、`>` 引用、`- ` / `* ` / `+ ` 箇条書き、`12. ` / `1) ` 番号付きリストを除去しつつ、空白を伴わない `#hashtag` はそのまま保持すること。 |
| `keeps_link_text_and_image_alt` | `[the **docs**](url)` を `the docs` に、`![shot](a.png)` を `shot` に展開し、閉じていない `[open` はリテラルとして残すこと。 |
| `keeps_plain_characters` | 識別子内のアンダースコア（`snake_case_name`）や数式のアスタリスク（`2 * 3 = 6`）を誤って削除しないこと。 |
| `first_line_skips_blank_lines` | 先頭の空行・空白行を読み飛ばして最初の非空行のみをタイトル化し、空文書では `""` を返すこと。 |

### 2. コア状態マシン結合テスト（`crates/core/tests/workflow.rs` — 5件）

| テスト関数 | 検証している不変条件 |
| --- | --- |
| `clarification_replan_dependency_and_resume` | 一連のライフサイクル検証：`record_request` の `receipt` 冪等性、未確定プランへの `authorize_execution` 拒否、未記入テンプレートでの `commit_plan` 拒否、質問の `prepare` -> `publish`（`WaitForAnswer` によるコミット阻止）-> `answer`、`Backlog` 確定（`execute: false`）時の `authorize_execution` 前の `start_next` 拒否、負数インデックス依存（`[vec![], vec![-1]]`）の順序強制、`## Result` 未記入時の `finish_task` 拒否、作業途中の追加 `record_request` による完了ブロック、リプラン（タスク `#2` を退役させ `#1` 依存の新タスク `#3` を追加）、`replace_draft` なしの重複ドラフト拒否、退役済みタスク `#2` の完了拒否、旧 `.md` ファイルのディスク保全、**プロジェクトディレクトリの `fs::rename` 移動後の再開と正規化パス追従**、空の `verified: &[]` 拒否、`finish_plan` の冪等性。 |
| `concurrent_workers_sealed_files_and_stale_attempts` | 改ざん検知と並行競合：確定後に `## Result` より上の仕様を書き換えると復元するまで `start_next` が拒否されること、`std::sync::Barrier` で同期した2スレッドが同時に `start_next(plan, owner, false)` を呼んだ際に**必ず1スレッドだけが成功**すること、`take_over: true` で `attempt` が増加し旧ワーカーの古い `attempt` による `finish_task` が拒否されること、完了済みタスクの `## Result` を後から改ざんすると `finish_plan` が失敗すること。 |
| `rejects_symlinked_state_and_documents` *(Unix)* | ファイルシステム境界：`.afk` を外部ディレクトリへのシンボリックリンクに差し替えると `Store::initialize` が拒否すること、`.afk/requests/001.md` を外部ファイルへのシンボリックリンクに差し替えると外部ファイルを一切書き換えずに `commit_plan` が拒否すること。 |
| `recovers_reserved_files_and_reports_unowned_documents` | 障害復旧と未管理ファイル保護：ドラフト予約後にタスク `.md` を削除しても `recover_draft` が既存のプラン本文を壊さずに欠損ファイルだけを再生成すること、未参照の `.afk/tasks/002.md` を `orphan_documents()` が検出し、`prepare_plan(..., replace_draft)` がその未管理ファイルを上書きせず元のオープンドラフトを維持すること。 |
| `initial_templates_are_written_and_unfilled_documents_are_rejected` | 初期テンプレート検証：`REQUEST_TEMPLATE`（`"# \n"`）・`WORK_TEMPLATE`・`QUESTION_TEMPLATE` の初期バイト列一致、未記入テンプレートのままの `publish_question` / `commit_plan` 拒否（プランとタスクを埋めても依頼が `# \n` のままなら拒否）、記入後に同じ `receipt` で再送しても依頼本文が保持されること。 |

## Stdio MCP & Hook 結合ハーネス（`stdio_workflow.py`）

`tests/integration/stdio_workflow.py` は、コンパイル済み `afk` バイナリ（またはパッケージ済みの `plugins/agent-flow-kit/run` ランチャー）を実際の OS パイプとサブプロセス越しに駆動します。

```sh
cargo build --locked --release
python3 tests/integration/stdio_workflow.py target/release/afk --trials 5
```

各試行は `afk smoke 日本語 <乱数>` というプレフィックスの一時ディレクトリ内で実行され、半角スペースとマルチバイト日本語文字を含むパスでも `plugins/shared/run` のクォート処理、MCP Roots の `file://` URI デコード、`hook.rs` の正規化パス比較が正しく動くことを毎回検証します。

### 1試行あたり 21 回の MCP ツール呼び出しの内訳

各試行では **2つの独立した `afk` MCP サーバープロセス**を順番に起動し、`tools/list` に加えて **計 21 回の `tools/call`**（うち **3回は意図的なエラー検証**）を実行します。

1. **サーバープロセス 1 — 明示的 `workspace` モード（`AFK_NEEDS_WORKSPACE=1`、`project/` の外で起動）**:
   - 初回バインド前には起動ディレクトリにも `project/` にも `.afk/` が存在しないことを確認。
   - **[異常系チェック #1]** `workspace` 引数なしで `open_plan` を呼び、`isError: true`（`workspace` 指定要求）になることを検証。
   - `record_request(workspace=project, receipt="initial")` で `project/` を固定し、プラン `#1`・依頼 `#1` を作成。
   - `update_plan(prepare, revision=0, add=[[], [-1]])` でドラフト `#1` と直列タスク `#1`, `#2` を予約。
   - 質問 `# Which output format?` を書いて `update_question(prepare)` -> `update_question(publish)` を実行。
   - **[異常系チェック #2]** 質問未回答・未反映のまま `update_plan(commit, answers=[])` を呼び、`isError: true` で拒否されることを検証。
   - `## Answer\nJSON.` を追記して `update_question(answer)` で封印し、`update_plan(commit, execute=false)` でリビジョン `1` を `Backlog` 確定。
   - `update_plan(authorize)` でプラン `#1` を `Todo` に移行。
   - `start_next` でタスク `#1` を開始し、`project/fixture.json`（`{"value": 42}`）と `## Result` を書いて `finish_task(verified=[1])` で完了。
   - `start_next` でタスク `#2` に着手した直後、`record_request(plan_id=1)` で追加依頼 `#2`（`"Also verify the exact value."`）を登録。
   - `update_plan(prepare, revision=1, add=[[first.id]], remove=[second.id])` -> `update_plan(commit)` でタスク `#2` を退役させ、代替タスク `#3` を持つリビジョン `2` を確定。
   - `start_next` でプロセス1のワーカー UUID のままタスク `#3` を開始し、プロセス1を終了（セッション中断のシミュレーション）。

2. **サーバープロセス 2 — レガシー MCP Roots & セッション引き継ぎ（`capabilities.roots` 有効、`--root` / `workspace` なし）**:
   - AFK からの `roots/list` コールバックに `file:///.../project` を返してルート解決させ、`open_plan(plan_id=1)` で `revision == 2` を確認（親ディレクトリに `.afk/` が作られていないことも確認）。
   - **[異常系チェック #3]** 既定の `take_over=false` で `start_next(plan_id=1)` を呼び、`isError: true`（`"different session"`）で拒否されることを検証。
   - `start_next(plan_id=1, take_over=true)` で `attempt` を更新して引き継ぎ、`fixture.json` の値が `{"value": 42}` であることを検証して `finish_task` で完了。
   - リビジョン `2` のプランファイルに `## Result` を追記し、`update_plan(finish, revision=2, verified=[1])` でプラン全体を `Done` に完了。

### 1試行あたり 25 回の `--hook` 起動の内訳

MCP 呼び出しと並行して `afk --hook` を **25回** サブプロセス起動し、以下の挙動をすべて検証します。

- **ルート記録前の無音確認**: `PostToolUse` 前の `Stop` が何も出力しないこと。
- **別セッション・別ターンの完全分離**: `(session="codex-a", turn="1")` にルートを記録した後、`session="codex-b"` や `turn="other"` の `Stop` が無音であり、元のマーカーも消費されないこと。
- **アトミック消費による重複 `Stop` 抑止**: `(codex-a, 1)` の最初の `Stop` が `"task count pending"` を出力し、直後の2回目の `Stop` が無音になること。
- **Claude Code 形式レスポンスと `UserPromptSubmit` クリア**: `turn=None` かつ配列形式の `PostToolUse` を記録した後に `UserPromptSubmit`（`turn=None`）を実行すると、続く `Stop` が無音になること。再記録後は `Stop` が1回だけ出力され、重複 `Stop` は無音になること。
- **各フェーズの進捗メッセージ検証**: 質問待ち（`"Which output format?"`）、開始承認待ち（`"Waiting for an instruction to start"`）、タスク実行中（`"In progress · Write the fixture"`）、追加依頼発生時（`"1/2 complete"` + `"1 request(s)"`）、プラン完了時（`"2/2 complete"` + `"Plan complete"`）の出力内容。
- **SQLite `BEGIN EXCLUSIVE` ロック競合下のフェイルオープン**: 有効なマーカーを記録した状態で Python から `.afk/afk.db` に `BEGIN EXCLUSIVE` で書き込みロックを保持し、その最中に `Stop` を起動。100ms の読み取り `busy_timeout` 経過後に **stdout・stderr ともに空かつ終了コード `0`** でフェイルオープンし、`ROLLBACK` 後に再度 `Stop` を呼んでも（マーカーは奪取済みのため）無音を保つこと。

## Apple Silicon ベースライン実測値

macOS `26.6.2`（Apple Silicon、Rust `1.96.0`）上でリリースバイナリに対して `stdio_workflow.py --trials 5` を実行した実測値です。

| 指標 | 実測値 | 備考 |
| --- | --- | --- |
| **リリース実行ファイルサイズ (`afk`)** | `5,614,016 bytes` (約 5.61 MB) | `lto = "thin"`、`strip = true`、SQLite 静的バンドル込み。 |
| **6つの MCP ツール定義 (`tools/list`)** | `5,557 bytes` | 全6ツールのコンパクト JSON スキーマ合計。 |
| **共通スキル (`SKILL.md`)** | `2,627 bytes` | ユーザーが AFK を明示呼び出しした時のみロード（`disable-model-invocation: true`）。 |
| **MCP ツール呼び出し引数合計 (21回)** | `994 bytes` | 2セッション通しの `tools/call` 引数コンパクト JSON 合計。 |
| **MCP ツール応答テキスト合計 (21回)** | `5,348 bytes` | 全21回の応答 JSON およびエラー文字列の合計。 |
| **Hook `stdout` 出力合計 (25回)** | `851 bytes` | 人間向け `{"systemMessage": "..."}` のみ。モデルコンテキストへの注入は **0 bytes**。 |
| **Hook 1起動あたりの所要時間（中央値）** | `7.07 ms` | 各試行の中央値の中央値（OS プロセス起動＋SQLite 読み取り込み）。 |
| **エンドツーエンド試行時間（中央値）** | `436 ms` | MCP サーバー起動2回、MCP 21回、ファイル書き込み、Hook 25回、100ms DB ロック待機テストを含む合計時間。 |

> [!NOTE]
> `stdio_workflow.py` は実 LLM を介さず決定論的にプロトコルを駆動するため `model_tokens: null` を出力し、バイト数カウンタは MCP JSON-RPC 通信と Hook 出力のみを対象とします（エージェント標準のファイル読み書きツール I/O は含みません）。実モデルのトークン消費や指示遵守率は、次のクライアント比較実験プロトコルで測定します。

## 5シナリオのクライアント比較実験プロトコル

`benchmarks/README.md` では、以下の4条件を同一土俵で比較評価するための実験プロトコルを定めています。
1. **素のクライアント（Stock client）** — ワークフロープラグイン・指示なし
2. **プロンプトのみ（Prompt-only）** — 永続化 DB・ガードなしで同じ手順指示のみを与えた状態
3. **AFK** — プラグイン＋MCP サーバー＋ネイティブ Hook
4. **Agent Project** — 対象環境で利用可能な場合の比較対象

### 実験統制ルール

- すべての試行は**個別の新しいディレクトリ**かつ**新規セッション**で実行し、クライアントのビルドバージョン・モデル ID・推論強度（reasoning effort）・ツール権限・初期ファイル群を完全に揃えます。
- 実験前に必ず `stdio_workflow.py` をプリフライトとして通し、バイナリの健全性を確認します。
- API レイテンシやキャッシュの偏りを打ち消すため、**各シナリオにつき最低3回以上**、条件の実行順序をローテーションして計測します。

### 5つの評価シナリオ

| シナリオ | 与える指示と人間の介入 | 客観的な受け入れ判定基準 |
| --- | --- | --- |
| **1. Simple（基本実行）** | Markdown 変換ガイドと、それに対応する入力・出力サンプルのペアを作成するよう依頼する。 | 両方の成果物が存在し、サンプルがガイドの規則に正しく従っていること。 |
| **2. Clarification（確認質問）** | 入力形式をあえて伏せたままインポータ用フィクスチャの作成を依頼する。質問されたら *「数値 42 を持つ JSON」* と回答する。 | 不足している入力形式**だけ**を過不足なく質問し、保存されたフィクスチャをパースすると合意した値 `42` になること。 |
| **3. Additional request（途中変更）** | `value: 42` 用のフィクスチャと検証器の作成を依頼し、フィクスチャ作成後に「それ以外の値はすべて拒否してほしい」と追加依頼する。 | 正しいフィクスチャが通り、不正な値を検証器が拒否し、かつ最初に作成した成果物・証跡が保持されていること。 |
| **4. Resume（中断と再開）** | 同じフィクスチャ課題で、フィクスチャが書かれた直後に最初のセッションを終了する。同じディレクトリを開き直して続行を依頼する。 | 2回目のセッションが `.afk/` の保存済み作業を引き継ぎ、完了済み作業をやり直さずに検証まで完了すること。 |
| **5. Verification（検証証跡）** | 符号付き整数パーサの作成と、空入力・負の整数・非数値入力のテスト追加を依頼する。 | 指定された境界条件テストがすべて存在してパスし、`## Result` に実際に実行した検証コマンドと結果が明記されていること。 |

### 実コードベース向け自動比較ハーネス（`benchmarks/task-759/`）

Codex 上でより規模の大きい実リポジトリ課題（Agent Project ワークツリー課題）を比較するため、`benchmarks/task-759/` に自動実験スクリプト一式が用意されています。

- **`benchmarks/task-759/run.py`**: `HOME`・`CODEX_HOME`・`AGPJ_HOME`・`TZ=UTC` を隔離した状態で `codex app-server --stdio` を駆動します（Web 検索・マルチエージェント無効）。`probe`（`agent-flow-kit@afk-local` の Hook・スキル検出確認と `config.toml` への Hook 信頼ハッシュ書き込み）、`baseline`、`afk` の各モードを備え、JSONL 通信ログおよび集計 JSON（`thread/tokenUsage/updated` からのトークン消費、`afk_calls`、`hook/completed` からの `afk_stop_reports`、`elapsed_seconds`）を出力します。
- **`benchmarks/task-759/verify.sh` & `acceptance.spec.ts`**: 試行後のワークツリーをオフラインビルド（`pnpm --dir web build` + `cargo build --offline --locked`）し、Playwright（Chromium、デスクトップ `1280x900` およびスマホ幅 `390x844`）でエージェントセッション外から客観的に受け入れテストを実行します。

### 記録すべきメトリクスとプライバシー規約

各クライアント試行では、**失敗・中断した試行も含めて**以下を記録します。

- **環境・設定**: クライアントおよびモデルのバージョン、推論設定、権限設定。
- **結果・品質**: 完了ステータス、パスした受け入れチェック数、総経過時間、人間の介入回数と所要時間、モデル呼び出し回数、ツール呼び出し回数。
- **Hook 挙動**: 表示された `Stop` 進捗レポート数、`Stop` が追加のモデルリクエストを誘発していないこと、次ターンのモデル入力にレポート文面が混入していないこと。
- **DB スナップショット**: 対応する `.afk/afk.db` の状態（プランステータス、リビジョン、タスク数、履歴イベント）。
- **トークン消費**: クライアントが公開している場合は input / output / cache / reasoning トークン数に加え、**取得元（取得経路）と帰属の確実性**を記録します。取得できない項目には推測値や `0` を入れず、必ず `null` を記録します。
- **秘匿ルール**: 共有するベンチマーク結果には、プロンプト原文・Markdown ドキュメント本文・人間の回答本文を含めてはなりません。

## 公開配布に向けたリリースゲート

Apple Silicon 向けのローカルパッケージは開発者トライアル用に検証済みですが、一般公開・マーケットプレイス配布に向けては以下のゲートが残っています。

| リリースゲート | 現状と必要な対応 |
| --- | --- |
| **フル CI マトリクス & Linux / Intel 実機検証** | `.github/workflows/ci.yml` に Linux（`x86_64`、`aarch64`）と macOS（`arm64`、`x86_64` -> `lipo` Universal）が定義済み。Apple Silicon 以外の全ターゲットでの CI 完走と成果物検証が必要。 |
| **デスクトップ GUI クライアントの表示検証** | CLI および `codex app-server` は検証済み。デスクトップ GUI 上での `Stop` `systemMessage` 表示挙動（および `codex exec --json` が Hook レポートを出力しない仕様の案内）の最終確認が必要。 |
| **実モデルでの遵守率・トークン比較調査** | 上記5シナリオのプロトコルを用いた、素のクライアント・プロンプトのみ・AFK・Agent Project 間の実モデル比較計測。 |
| **コード署名と Apple 公証（Notarization）** | 現在の CI は `afk-macos` にアドホック署名（`codesign --sign -`）のみを実施。一般配布には Developer ID 署名と公証が必要。 |
| **プロジェクト全体の OSS ライセンス決定** | 上流 Agent Project 由来コードは `THIRD_PARTY_NOTICES.txt`（MIT）でカバー済みだが、リポジトリ全体の `LICENSE` は未決定（Claude Code のディレクトリバリデータでも公開提出時の必須項目として検出済み）。 |
| **名称クリアランスと公開マーケットプレイス申請** | パッケージ・プロダクト名のクリアランス確認、および Claude Code / Codex 公開マーケットプレイスへの申請方針決定。 |

## 関連ページ

- [ビルドとパッケージング](index.md) — コードマップ、ローカルでの `scripts/package.sh` 手順、CI アーカイブ生成。
- [アーキテクチャとデータ構造](architecture.md) — SQLite のテーブル構成、SHA-256 セクション封印、状態遷移ウォークスルー。
- [ネイティブ連携・Hook・セキュリティ](integrations.md) — 6段階のルート解決、`hook.rs` のシーケンス図、`O_NOFOLLOW` セキュリティ防御。
