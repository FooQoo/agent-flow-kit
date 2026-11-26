# Agent Flow Kit をインストールする

AFK は Claude Code と Codex のプラグインを Git マーケットプレイスで配布します。プラグインには macOS と Linux 用の AFK 実行ファイルが含まれます。`YOUR_GIT_MARKETPLACE_URL` は、配布元の Git リポジトリ URL に置き換えてください。

## Claude Code

```sh
claude plugin marketplace add "YOUR_GIT_MARKETPLACE_URL"
claude plugin install agent-flow-kit@afk
```

プロジェクトで新しい Claude Code セッションを開き、`/agent-flow-kit:afk` に続けて依頼を書きます。

## Codex

```sh
codex plugin marketplace add "YOUR_GIT_MARKETPLACE_URL"
codex plugin add agent-flow-kit@afk
```

プロジェクトで新しい会話を開き、`$agent-flow-kit:afk` を選びます。Hook の確認が表示されたら、内容を確認してください。

## Antigravity・その他の stdio MCP クライアント

同じ Git マーケットプレイスをクローンします。MCP の起動コマンドにクローン先の `plugins/codex/run` の絶対パスを指定し、サーバーの環境に `AFK_NEEDS_WORKSPACE=1` を設定します。`plugins/codex/skills/afk/SKILL.md` をクライアントのスキルに追加し、最初の AFK 呼び出しでプロジェクトのパスを渡します。

[クイックスタートに進む](./quickstart.md)
