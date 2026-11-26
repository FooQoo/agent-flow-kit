# Agent Flow Kit

[English](README.md)

Agent Flow Kit（AFK）は、Claude Code、Codex などの MCP クライアントに任せた作業を計画し、進捗を確認して続きから再開するためのツールです。計画、質問、検証結果はプロジェクトの `.afk/` に残ります。

- [Git からインストール](website/ja/guide/index.md)
- [クイックスタート](website/ja/guide/quickstart.md)

## AFK で作業を任せる

着手前に計画を頼みます。タスクを確認し、質問に答えたら、エージェントに開始を伝えます。途中で条件を追加でき、検証結果は `.afk/tasks/` で読めます。

別のセッションに引き継ぐときは、Claude Code なら `/agent-flow-kit:afk handoff` を実行します。Codex では `$agent-flow-kit:afk` を選んで `handoff` と送ります。

## リポジトリ内の資料

- [公開サイトのソース](website/ja/index.md)
- [開発・参照資料](docs/README.md)
- [ベンチマーク](benchmarks/README.md)

## ソースの再利用

状態の型定義と Markdown の見出し処理は、MIT ライセンスの [Agent Project](https://github.com/FooQoo/agent-project)（コミット `095bc9bdad1c10cee9fb11ed25d616416e990167`）から取り入れました。詳しくは [THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt) を参照してください。
