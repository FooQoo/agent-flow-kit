# Agent Flow Kit をインストールする

AFK は `FooQoo/agent-flow-kit` リポジトリから Claude Code と Codex 向けに直接配布しています。初回 MCP 起動時にプラグインの `run` スクリプトが [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases) から対象 OS（macOS / Linux）のバイナリを自動取得し、`SHA256SUMS` で検証してローカルにキャッシュします。ネットワーク接続が必要なのは各バージョンの初回起動時のみです。

## Claude Code

```sh
claude plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
claude plugin install agent-flow-kit@afk
```

プロジェクトで新しい Claude Code セッションを開き、`/agent-flow-kit:afk` に続けて依頼を書きます。

## Codex

```sh
codex plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
codex plugin add agent-flow-kit@afk
```

プロジェクトで新しい会話を開き、`$agent-flow-kit:afk` を選びます。Hook の確認が表示されたら、内容を確認してください。

## Antigravity・その他の stdio MCP クライアント

`https://github.com/FooQoo/agent-flow-kit.git` をクローンします。MCP の起動コマンドにクローン先の `plugins/codex/run` の絶対パスを指定し、サーバーの環境に `AFK_NEEDS_WORKSPACE=1` を設定します。`plugins/codex/skills/afk/SKILL.md` をクライアントのスキルに追加し、最初の AFK 呼び出しでプロジェクトのパスを渡します。

## オフライン環境での利用

初回起動時に GitHub Releases へ接続できない環境では、[GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases) からバイナリ同梱アーカイブ（`afk-claude-code.tar.gz` または `afk-codex.tar.gz`）と `SHA256SUMS` を取得してください。`run` と同じ階層に `bin/afk-*` が配置されている場合、ネットワーク通信なしで同梱バイナリをそのまま実行します。

[クイックスタートに進む](./quickstart.md)
