---
title: インストール
description: Claude Code・Codex・その他の stdio MCP クライアントに Agent Flow Kit を導入する手順。
---

# インストール

*手元の AI エージェントに AFK を追加し、きちんと動くことを確認するための手順書です。*

AFK は「プラグインを追加 → いつものエージェントを起動 → 仕事を頼む」ですぐ始められます。Rust のビルド環境、Homebrew、常駐サーバー、専用管理画面は不要です。

## 動作環境

- **対応 OS / CPU**:
  - macOS（Apple Silicon / Intel 両対応の `afk-macos`）
  - Linux（`x86_64`: `afk-linux-x64`、`arm64`: `afk-linux-arm64`）
- **初回起動時のインターネット接続**: 各バージョンの初回起動時だけ、プラグイン内の起動スクリプト（`run`）が [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases) から対象 OS 用の実行ファイルを取得し、改ざんチェック（SHA-256 検証）をして手元にキャッシュします。2 回目以降はオフラインで起動します。
- **使用する標準コマンド**: `curl` または `wget`、`tar`、`shasum` または `sha256sum`（macOS / Linux に標準搭載）。
- **対応クライアント**: Claude Code、Codex、または stdio MCP に対応したクライアント。

## Claude Code への導入

AFK のリポジトリをマーケットプレイスに登録し、プラグインを追加します。

```sh
claude plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
claude plugin install agent-flow-kit@afk
```

1. 作業したいプロジェクトで新しい Claude Code セッションを開きます。
2. `/agent-flow-kit:afk` に続けて依頼を入力します。
3. Claude Code はプロジェクトのパスを自動で渡すため、起動後すぐに `.afk/` の保存と会話末尾の進捗表示が有効になります。

## Codex への導入

AFK のリポジトリをマーケットプレイスに登録し、プラグインを有効にします。

```sh
codex plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
codex plugin add agent-flow-kit@afk
```

1. 作業したいプロジェクトで新しい会話を開きます。
2. `$agent-flow-kit:afk` を選んで依頼を送ります。
3. プラグインの Hook（進捗表示に使う設定）の許可確認が表示されたら内容を確認して許可します。

## 動作確認

導入できたら、次のように動作を確かめます。

1. 変更してよいサンプルプロジェクトで新しいセッションを開きます。
2. まず計画だけを頼んでみます（例: `/agent-flow-kit:afk 小さなテスト用データの計画だけ立てて。まだ着手しないで。`）。
3. 以下を確認します。
   - プロジェクト直下に `.afk/` フォルダ（`afk.db`, `requests/`, `plans/`, `tasks/`）が作られている。
   - 会話の最後に `[AFK] ... · ██████░░░░ ...` の進捗メッセージが表示される（Hook が有効な場合）。

うまく動かない場合は [トラブルシューティング](/ja/troubleshooting) を参照してください。

## プロジェクト内のデータ（`.afk/`）

- `.afk/` は最初に依頼を登録したり計画を開いたりしたタイミングで、プロジェクト直下に自動作成されます。
- 誤って `/` やホームディレクトリ（`$HOME`）、プラグイン自身のフォルダに `.afk/` を作らない保護が入っています。
- AFK が `.gitignore` を勝手に書き換えることはありません。手元だけに留めたい場合は `/.afk/` を `.gitignore` に追加してください。

## 更新方法

利用しているクライアントのプラグイン更新コマンドで更新できます（手動クローンの場合は `git pull`）。実行ファイルはバージョン別のキャッシュに保存されるため、バージョンが上がると初回起動時に新しいファイルが自動で取得・検証されます。

念のため大きな更新前は、AFK を動かしていない状態で `.afk/` をバックアップしておくと安心です。

## アンインストール方法

- **プラグインの削除**: Claude Code / Codex のプラグイン管理から `agent-flow-kit` を削除します。
- **キャッシュの削除**（任意）: プラグインデータフォルダ、または `${XDG_CACHE_HOME:-$HOME/.cache}/agent-flow-kit/bin` を削除します。
- **プロジェクト内の記録**: プラグインを外しても各プロジェクトの `.afk/` は消えません。不要な場合のみ手動で削除してください。

## オフライン環境での導入

初回起動時に GitHub Releases へ接続できない環境では、[GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases) から実行ファイル同梱のアーカイブ（`afk-claude-code.tar.gz` または `afk-codex.tar.gz`）と `SHA256SUMS` を取得してください。`run` と同じ階層に `bin/afk-*` がある場合、通信なしでそのまま起動します。

## その他の MCP クライアント（Antigravity など）での手動設定

> **注意:** MCP サーバーだけの接続では 6 つのツールは使えますが、クライアント固有の Hook やガイド（Skill）は自動で入りません。ガイドファイルも併せて登録することをおすすめします。

1. リポジトリをクローンします。
   ```sh
   git clone https://github.com/FooQoo/agent-flow-kit.git
   ```
2. MCP サーバーの起動コマンドに `plugins/codex/run` の絶対パスを指定し、環境変数に `AFK_NEEDS_WORKSPACE=1` を設定します（特定プロジェクト専用なら `--root /プロジェクトの絶対パス` も指定できます）。
3. `plugins/codex/skills/afk/SKILL.md` をクライアントのスキルとして登録します。

## 次に読むページ

- [最初の仕事を任せる](/ja/guide/quickstart) — 依頼から計画・質問・実行・完了・引き継ぎまでを体験する。
- [トラブルシューティング](/ja/troubleshooting) — 導入や進捗表示で困ったときの対処法。
