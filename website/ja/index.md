---
layout: page
title: Agent Flow Kit
description: AI の仕事の遂行を支える軽量なハーネス。Plan Mode の先にある途中変更・完了検証・セッション跨ぎ再開をローカル状態で支えます。
sidebar: false
aside: false
landing:
  copy:
    label: コマンドをコピー
    done: コピーしました
  hero:
    eyebrow: Delegate work. Stay in control.
    title: "計画を立てたあとも、\nやり遂げる仕組みを。"
    lead: Agent Flow Kit は、AI の仕事の遂行を支える軽量なハーネスです。一時的な計画メモで終わらせず、途中の仕様変更・完了条件の検証・別セッションからの再開までをプロジェクト内の .afk/ で一貫して支えます。
    primary:
      text: はじめる
      link: /ja/guide/installation
    secondary:
      text: GitHub
      link: https://github.com/FooQoo/agent-flow-kit
    meta: Claude Code / Codex · Local-first (.afk/) · Rust + SQLite · Open Source
    demo:
      header: .afk/ によるセッション跨ぎ再開と途中改訂
      sampleBadge: イメージ
      ariaLabel: Session 1 で handoff を保存し、翌日の Session 2 で追加要件を取り込んで rev 2 から再開するターミナルイメージ
      boundaryLabel: 翌日 · 新しいセッション（Claude Code / Codex）
      caption: ※ 表示イメージです。実際の出力形式は利用するクライアントや Hook 設定によって異なります。
      sessions:
        - label: "Session #1 — Claude Code"
          turns:
            - kind: prompt
              text: /agent-flow-kit:afk handoff
            - kind: saved
              text: "Saved handoff for #4 (3/5 done) → .afk/afk.db"
        - label: "Session #2 — 別セッションから再開"
          turns:
            - kind: prompt
              text: 前回の続きを進めて。検索0件の表示も直して。
            - kind: reply
              text: ".afk/ から Plan #1 (rev 1) と handoff を復元。完了済み #1–#3 を証跡として残し rev 2 へ改訂します。"
      status:
        title: 検索機能の改善
        revision: rev 2
        progressText: 3/6 (50%)
        percent: 50
        tasks:
          - state: done
            text: "#1–#3 調査・仕様・検索ロジック"
            tag: rev 1 証跡保持
          - state: doing
            text: "#4 テストの追加"
            tag: handoff から再開
          - state: todo
            text: "#6 検索0件表示の改善"
            tag: 途中追加
  problem:
    eyebrow: Problem
    title: 計画は立てられる。でも、そのあとが崩れる。
    lead: 標準の Plan Mode や内蔵 Todo は「着手前の計画」には便利です。ただ、仕事が長引いたりセッションをまたいだりすると、別の問題が起きます。
    items:
      - icon: list-x
        title: 途中変更で計画と実装がズレる
        text: 追加依頼やコンテキスト圧縮が重なると、当初の完了条件や変更点が会話の奥へ流れてしまう。
      - icon: circle-check
        title: 完了の証拠が残らない
        text: 「完了しました」と言われても、すべての受け入れ条件が本当に検証されたのか後から追えない。
      - icon: history
        title: セッションを閉じるとリセットされる
        text: 翌日や別エージェントで続きをやろうとすると、計画・決定事項・残タスクを説明し直す必要がある。
      - icon: message-circle-question
        title: 判断の経緯がチャットに埋もれる
        text: 「なぜこの仕様にしたか」という質問と回答がログに埋もれ、再計画や引き継ぎで活かせない。
    comparison:
      title: 標準の Plan Mode / 内蔵 Todo との違い
      lead: Plan Mode は 1 セッション内の一時的な計画メモ。AFK は完了まで状態と証跡を検証し続ける永続ハーネスです。
      axisHeader: 観点
      planModeHeader: 標準 Plan Mode / 内蔵 Todo
      afkHeader: Agent Flow Kit (AFK)
      rows:
        - axis: 寿命と範囲
          planMode: 1 セッション内の一時メモ。セッション終了や圧縮で薄れる。
          afk: プロジェクト内 .afk/ に永続化。別セッションや別クライアントから再開可能。
        - axis: 途中の仕様変更
          planMode: チャット上で上書きされ、元の計画や完了分とズレやすい。
          afk: 完了済みタスクを証跡として保持したまま、残タスクだけを Revision 改訂。
        - axis: 完了の検証
          planMode: AI の自己申告（「完了しました」）で終了できる。
          afk: SHA-256 仕様ロック＋全受け入れ条件の番号検証＋## Result が必須のゲート。
        - axis: 進捗の把握
          planMode: チャットログを遡るか、AI に状況を聞き直す。
          afk: Stop Hook が SQLite からターン末尾に自動報告（追加推論なし）。
    closing: 必要なのは「最初に計画を書く機能」ではなく、完了まで状態と証跡を貫くハーネスです。
  workflow:
    eyebrow: Workflow
    title: 頼む。答える。あとは任せる。
    lead: 不明点がなければ質問は省略でき、計画だけの依頼（Backlog）と実行依頼（Todo）も明確に区別します。
    steps:
      - step: "01"
        title: 依頼する
        actor: human
        actorLabel: Human
        text: いつも通り、やりたいことを伝える。
      - step: "02"
        title: 計画する
        actor: ai
        actorLabel: AI
        text: 仕事を分解し、順序と完了条件（## Acceptance）を整理する。
      - step: "03"
        title: 不明点を解消する
        actor: both
        actorLabel: Human + AI
        text: 人間の判断が必要なことだけを質問し、回答を封印する。
      - step: "04"
        title: 実行する
        actor: ai
        actorLabel: AI
        text: 依存関係どおりに進め、検証結果を ## Result に記録する。
      - step: "05"
        title: 完了する
        actor: ai
        actorLabel: AI
        text: 全完了条件をゲート検証し、結果を報告する。
    adaptTitle: いつでも方向を変えられる。
    adaptText: 実行途中でも依頼を追加し、完了済みタスクは証跡として残したまま新リビジョンで再計画できます。変更の反映タイミングは利用するクライアントの会話境界に従います。
  humanOnTheLoop:
    eyebrow: Human on the Loop
    title: 仕事はAIに。主導権はあなたに。
    lead: 完全自律を追求するのではなく、Human on the Loop を軸に、必要な場面だけ Human in the Loop を取り入れる仕事の進め方を提供します。
    human:
      badge: Human
      title: あなたの役割
      items:
        - 目的を伝える
        - 必要な判断をする
        - 方向を修正する
    ai:
      badge: AI Agent
      title: AIの役割
      items:
        - 仕事を計画する
        - 実行・検証する
        - 証跡を残して完了する
    harness:
      label: Agent Flow Kit
      text: 構造・状態機械・仕様ハッシュ検証・永続化・進捗報告で両者を支えます。AFK 自体が推論やコード実行を行うわけではありません。
    note: 質問を行うのは、意図が不明なとき・人間の選好で決まるとき・不可逆な操作・依頼範囲の拡大などに限られます。常に一律の承認を要求するわけではありません。
  features:
    eyebrow: Features
    title: Plan Mode にはない、4 つの実行ハーネス。
    items:
      - kind: revision
        eyebrow: REVISION
        title: 履歴を壊さない途中改訂
        text: 途中で要件が増えても、完了済みタスクは証跡として保持したまま、残タスクだけを新しいリビジョンへ差し替えます。
        visualTitle: Plan #1 · 途中依頼の反映
        revision:
          from: rev 1
          to: rev 2
          rows:
            - id: "#1–#3"
              state: done
              badge: 証跡保持
              text: 調査・仕様・検索ロジック
            - id: "#5"
              state: retired
              badge: 差し替え
              text: 旧・単純表示の確認
            - id: "#4"
              state: todo
              badge: 継続
              text: テストの追加
            - id: "#6"
              state: added
              badge: 追加要件
              text: 検索0件表示の改善
      - kind: gate
        eyebrow: VERIFICATION GATE
        title: 決定論的な完了ゲートと仕様ロック
        text: 確定した仕様は SHA-256 で保護。すべての受け入れ条件（## Acceptance）の検証番号と ## Result の証跡が揃うまで Done になりません。
        visualTitle: finish_task 検証ゲート
        gate:
          command: "finish_task(plan_id=1, task_id=4, verified=[1, 2, 3])"
          checks:
            - label: spec_hash (SHA-256)
              value: 一致・未改変
            - label: "## Acceptance (1..=3)"
              value: 全 3 項目検証済み
            - label: "## Result 証跡"
              value: 記録済み
          outcome: "✓ Gate passed → Task #4: Done"
      - kind: handoff
        eyebrow: HANDOFF & .afk/
        title: セッション・クライアントを跨ぐ継続
        text: 依頼・計画・質問・結果をプロジェクト内の .afk/ に保存。handoff メモを残せば、別セッションや別クライアントからも安全に引き継げます。
        visualTitle: .afk/ + handoff
        handoff:
          tree:
            - ".afk/"
            - "├── afk.db        # 状態・依存・ハッシュ"
            - "├── plans/        # リビジョン別計画"
            - "├── tasks/        # 完了条件・実行履歴・Result"
            - "├── requests/     # 初回＆途中依頼"
            - "└── questions/    # 質問と封印済み回答"
          noteLabel: "handoff (Attempt 1: claude-code → Attempt 2: codex)"
          noteText: "検索ロジック実装済み。残りはテスト追加と0件表示の境界値確認。"
      - kind: progress
        eyebrow: STOP HOOK
        title: 追加推論なしの自動進捗報告
        text: 会話ターンの末尾に Stop Hook が SQLite を読み取り専用で参照し、現在地を自動表示。毎回こちらから状況を聞く必要がありません。
        note: 報告専用の Tool Call や追加モデル推論を要求しない設計です。表示形式はクライアントにより異なります。
        visualTitle: Turn-end report
        progress:
          percent: 60
          tag: 0 model tokens
          detail: SQLite から直接集計（読み取り専用・fail-open）
          banner: "AFK · 検索機能の改善 [Doing] · 3/5 complete · Next: テストの追加"
  efficiency:
    eyebrow: State Machine & Efficiency
    title: 「お願い」ではなく、状態機械で支える。
    lead: プロンプトや Plan Mode だけでは管理ルールが会話の文脈に流されます。決定論的な状態遷移・検証・集計を Rust + SQLite へ移譲することで、AI が推論と実装に集中できる設計をとっています。
    ai:
      badge: AI Agent
      title: AIが担当すること：推論・創造・実行
      items:
        - 計画の立案と再計画
        - 仕様の理解と実装
        - 成果物の作成
        - 完了条件の実検証
    afk:
      badge: AFK (Rust + SQLite)
      title: AFKが担当すること：決定論的な管理・検証
      items:
        - 状態遷移と依存関係の強制
        - 仕様ハッシュ（SHA-256）の整合性検証
        - 未回答質問・未反映依頼のゲート制御
        - 進捗集計と .afk/ への永続化
    note: 管理負担とトークン消費の低減は設計目標であり、標準エージェント単体との比較ベンチマークは実測データが揃い次第公開予定です。
  architecture:
    eyebrow: Architecture
    title: 新しい開発環境は、必要ありません。
    lead: Don't replace your AI agent. Extend how it works. 既存の AI エージェントに Plugin として追加するだけで、MCP・Skills・Hooks が一つの Kit として機能します。
    diagram:
      ariaLabel: Claude Code や Codex が Agent Flow Kit の MCP・Skills・Hooks を通じて Markdown と SQLite を扱う構成図
      clientsLabel: AI Agents
      clients:
        - Claude Code
        - Codex
      kitLabel: Agent Flow Kit (Plugin)
      kit:
        - name: MCP
          role: 状態機械・検証ゲート
        - name: Skills
          role: 仕事の進め方の指針
        - name: Hooks
          role: ターン末尾の自動進捗報告
      storageLabel: Project (.afk/)
      storage:
        - name: Markdown
          role: AI が標準ツールで読み書きする本文
        - name: SQLite
          role: AFK が管理する状態・依存・ハッシュ
    items:
      - icon: laptop
        title: Local-first
        text: 依頼・計画・進捗データはプロジェクト内の .afk/ に保存します。
      - icon: cpu
        title: 軽量な Rust バイナリ
        text: 追加の実行ランタイムや常駐サーバーを必要としない設計です。
      - icon: plug
        title: Plugin で導入
        text: 既存のエージェントや管理画面への移行を求めません。
      - icon: code
        title: Open Source
        text: 実装とデータ構造をそのまま確認できる透明性があります。
    note: ※「Local-first」は AFK 自身のデータ保存先を指します。Claude Code や Codex が行うクラウドモデルとの通信までローカル化されるという意味ではありません。
  start:
    eyebrow: Getting Started
    title: インストールしたら、いつも通りに。
    lead: Claude Code または Codex に Plugin を追加すれば、すぐに使い始められます。
    steps:
      - title: Plugin をインストール
        text: Claude Code または Codex に AFK の Git マーケットプレイスを追加します。
      - title: プロジェクトを開く
        text: いつもの開発環境とリポジトリをそのまま開きます。
      - title: AI に仕事を頼む
        text: 自然言語でいつも通り依頼します。初回起動時に対象 OS のバイナリを自動取得・検証します。
    clientSelectLabel: クライアントを選択
    clients:
      - id: claude-code
        label: Claude Code
        commands:
          - claude plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
          - claude plugin install agent-flow-kit@afk
      - id: codex
        label: Codex
        commands:
          - codex plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
          - codex plugin add agent-flow-kit@afk
    installNote: 初回起動時のみ GitHub Releases から対象 OS バイナリを自動取得し SHA256 検証します。オフライン導入や Hook の確認手順は
    guideLink:
      text: 導入ガイドを見る
      link: /ja/guide/installation
    cta:
      tagline: "Delegate work.\nStay in control."
      sub: Plan Mode の先にある、やり遂げるための軽量ハーネス。
      primary:
        text: Agent Flow Kit をはじめる
        link: /ja/guide/installation
      secondary:
        text: GitHub
        link: https://github.com/FooQoo/agent-flow-kit
---

<LandingPage />
