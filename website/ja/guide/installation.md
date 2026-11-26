---
title: インストール
description: Claude Code・Codex・その他の stdio MCP クライアントに Agent Flow Kit を導入する手順。
---

# インストール

*手元の AI エージェントに AFK を導入し、MCP・Skills・Hooks が使える状態を確認するための手順書です。*

AFK は「Plugin を追加 → いつものエージェントを起動 → 仕事を依頼」で始められるように設計されています。Rust のビルド環境、Homebrew、常駐サーバー、専用 UI は不要です。

## Requirements（動作要件）

- **対応 OS / アーキテクチャ**:
  - macOS（`arm64` / `x86_64` — Universal バイナリ `afk-macos`）
  - Linux（`x86_64`: `afk-linux-x64`、`arm64`/`aarch64`: `afk-linux-arm64`）
- **初回起動時のネットワーク接続**: 各バージョンの初回 MCP 起動時に、プラグイン内のランチャー（`run`）が [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases) から対象 OS のアーカイブと `SHA256SUMS` を自動取得し、SHA-256 を検証してローカルにキャッシュします。2 回目以降はキャッシュからオフラインで起動します。
- **ランチャーが使用する標準コマンド**: `curl` または `wget`、`tar`、`shasum` または `sha256sum`（macOS / Linux 標準）。
- **対応クライアント**: Claude Code、Codex、または任意の stdio MCP クライアント。

## Claude Code

AFK の Git マーケットプレイスを追加して Plugin をインストールします。

```sh
claude plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
claude plugin install agent-flow-kit@afk
```

1. 作業したいプロジェクトで新しい Claude Code セッションを開きます。
2. `/agent-flow-kit:afk` に続けて依頼を入力します。
3. Claude Code は `CLAUDE_PROJECT_DIR` を自動で渡すため、起動時にプロジェクトルートが解決され、`PostToolUse`・`Stop`・`UserPromptSubmit` の各 Hook が有効になります。

## Codex

AFK の Git マーケットプレイスを追加して Plugin を有効化します。

```sh
codex plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
codex plugin add agent-flow-kit@afk
```

1. 作業したいプロジェクトで新しい会話を開きます。
2. `$agent-flow-kit:afk` を選んで依頼を送ります。
3. プラグインの Hook（`PostToolUse` と `Stop`）の信頼確認が表示されたら内容を確認して許可します。
4. Codex のプラグイン設定では `AFK_NEEDS_WORKSPACE=1` が設定されており、同梱の Skill が最初の AFK 呼び出し時にプロジェクトパスを渡すようガイドします。

## Verification（動作確認）

導入後、次のように動作を確認できます。

1. 変更してよいサンプルプロジェクトで新しいセッションを開きます。
2. 計画のみを依頼してみます（例: `/agent-flow-kit:afk 小さなテスト用フィクスチャの計画だけ立てて。まだ着手しないで。`）。
3. 以下を確認します。
   - プロジェクト直下に `.afk/afk.db`・`.afk/requests/`・`.afk/plans/`・`.afk/tasks/` が作成されている。
   - 会話ターンの末尾に `AFK · ...` の進捗報告が表示される（Claude Code または対話型 Codex で Hook が有効な場合）。

うまく認識されない場合は [トラブルシューティング](/ja/troubleshooting) を参照してください。

## Project initialization（プロジェクト初期化）

- `.afk/` は最初の依頼登録またはプラン確認時に、解決されたプロジェクトルート直下へ自動生成されます。
- 安全のため、`/` やホームディレクトリ（`$HOME`）、プラグイン自身のインストール先ディレクトリには `.afk/` を作成しない保護が入っています。
- AFK が `.gitignore` を自動で書き換えることはありません。ローカルのみに留めたい場合は `/.afk/` を `.gitignore` に追加してください。

## Update（更新）

利用しているクライアントのプラグイン更新コマンドで更新できます（手動クローンの場合は `git pull`）。バイナリはバージョン別のキャッシュディレクトリ（`v<version>/`）に保存されるため、バージョンが上がると初回起動時に対応するバイナリが自動で再取得・検証されます。

念のため大きな更新前は、AFK を実行していない状態で `.afk/` をバックアップしておくと安心です。

## Uninstall（アンインストール）

- **Plugin の削除**: Claude Code / Codex のプラグイン削除機能から `agent-flow-kit` を削除します。
- **キャッシュバイナリの削除**（任意）: プラグインデータディレクトリまたは `${XDG_CACHE_HOME:-$HOME/.cache}/agent-flow-kit/bin` を削除します。
- **プロジェクトデータ**: Plugin を削除しても各プロジェクト内の `.afk/` は消えません。不要な場合のみ手動で削除してください。

## オフライン環境での利用

初回起動時に GitHub Releases へ接続できない環境では、[GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases) からバイナリ同梱アーカイブ（`afk-claude-code.tar.gz` または `afk-codex.tar.gz`）と `SHA256SUMS` を取得してください。`run` と同じ階層に `bin/afk-*` が配置されている場合、ネットワーク通信なしで同梱バイナリを実行します。

## Manual MCP setup（Antigravity・その他の stdio MCP クライアント）

> **注意:** MCP サーバー単体の接続では 6 つのツールは利用できますが、ホスト固有の Hook や Skill は自動で入りません。Skill も併せて登録することを推奨します。

1. リポジトリをクローンします。
   ```sh
   git clone https://github.com/FooQoo/agent-flow-kit.git
   ```
2. クライアントの MCP 起動コマンドに `plugins/codex/run` の絶対パスを指定し、環境変数に `AFK_NEEDS_WORKSPACE=1` を設定します（単一プロジェクト固定なら `--root /プロジェクトの絶対パス` も指定できます）。
3. `plugins/codex/skills/afk/SKILL.md` をクライアントのスキルとして登録します。

## Next steps

- [クイックスタート](/ja/guide/quickstart) — 依頼から計画・質問・実行・完了・再開までを体験する。
- [トラブルシューティング](/ja/troubleshooting) — 導入・Hook・ワークスペース解決の問題を解決する。
