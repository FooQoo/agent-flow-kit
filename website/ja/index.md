---
layout: page
title: Agent Flow Kit
description: AI の仕事の遂行を支える軽量なハーネス。既存の AI エージェントを置き換えるのではなく、仕事の進め方を拡張します。
sidebar: false
aside: false
landing:
  copy:
    label: コマンドをコピー
    done: コピーしました
  hero:
    eyebrow: Delegate work. Stay in control.
    title: "いつものAIに、\n仕事を任せる仕組みを。"
    lead: Agent Flow Kit は、AI の仕事の遂行を支える軽量なハーネスです。依頼から計画、質問、実行、完了まで。使い慣れた開発環境のまま、AI に仕事を任せられるようにします。
    primary:
      text: はじめる
      link: /ja/guide/installation
    secondary:
      text: GitHub
      link: https://github.com/FooQoo/agent-flow-kit
    meta: Claude Code / Codex · Local-first · Open Source
    demo:
      header: Claude Code + AFK
      sampleBadge: イメージ
      ariaLabel: Claude Code 上で AFK を使って計画・質問・進捗表示を行う会話イメージ
      caption: ※ 表示イメージです。実際の出力形式は利用するクライアントや Hook 設定によって異なります。
      turns:
        - kind: prompt
          text: 検索機能を改善して
        - kind: reply
          text: 計画を作成しました。1点確認させてください。
        - kind: question
          text: 検索結果は関連度順と新着順のどちらにしますか？
        - kind: prompt
          text: 関連度順でお願いします。
        - kind: reply
          text: 承知しました。実装を進めます。
      status:
        title: 検索機能の改善
        progressText: 3/5 (60%)
        percent: 60
        tasks:
          - state: done
            text: 既存実装の調査
          - state: done
            text: 検索仕様の整理
          - state: done
            text: 検索ロジックの実装
          - state: doing
            text: テストの追加
          - state: todo
            text: 完了条件の検証
  problem:
    eyebrow: Problem
    title: AIは仕事ができる。でも、仕事を任せきれない。
    items:
      - icon: list-x
        title: 計画通りに進まない
        text: 長い仕事になると、当初の目的や作業内容が抜け落ちる。
      - icon: message-circle-question
        title: 勝手に判断される
        text: 人間が決めるべき仕様まで、AI が推測して実装してしまう。
      - icon: circle-check
        title: 完了したか分からない
        text: 本当に要件を満たしたのか、人間が改めて確認する必要がある。
      - icon: history
        title: 仕事の文脈が失われる
        text: セッションが変わるたびに、計画や進捗を説明し直す。
    closing: AI の能力だけでなく、仕事をやり遂げるための仕組みも必要です。
  workflow:
    eyebrow: Workflow
    title: 頼む。答える。あとは任せる。
    lead: 不明点がなければ質問は省略でき、計画だけの依頼と実行依頼も区別します。
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
        text: 仕事を分解し、タスクと完了条件を整理する。
      - step: "03"
        title: 不明点を解消する
        actor: both
        actorLabel: Human + AI
        text: 人間の判断が必要なことだけを質問する。
      - step: "04"
        title: 実行する
        actor: ai
        actorLabel: AI
        text: 計画に沿って仕事を進め、結果を記録する。
      - step: "05"
        title: 完了する
        actor: ai
        actorLabel: AI
        text: 完了条件を検証し、仕事の結果を報告する。
    adaptTitle: いつでも方向を変えられる。
    adaptText: 実行途中でも依頼を追加し、計画や完了条件を修正できます。変更の反映タイミングは利用するクライアントの会話境界に従います。
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
        - 進捗を報告する
    harness:
      label: Agent Flow Kit
      text: 構造・進行支援・状態管理・進捗報告で両者の仕事の進め方を支えます。AFK 自体が推論やコード実行を行うわけではありません。
    note: 質問を行うのは、意図が不明なとき・人間の選好で決まるとき・不可逆な操作・依頼範囲の拡大などに限られます。常に一律の承認を要求するわけではありません。
  features:
    eyebrow: Features
    title: 仕事を任せるための4つの仕組み。
    items:
      - kind: plan
        eyebrow: PLAN
        title: 計画とタスク分解
        text: 大きな依頼を実行可能な単位に分解。順序と完了条件を整理して管理します。
        visualTitle: 検索機能の改善
        items:
          - state: done
            text: 要件整理
          - state: done
            text: 設計
          - state: todo
            text: 実装
          - state: todo
            text: テスト
      - kind: question
        eyebrow: QUESTION
        title: 必要なときだけ質問
        text: 人間の意図や選好が必要な場面では、勝手に判断せず質問として切り出して確認します。
        visualTitle: 検索結果の並び順は？
        items:
          - text: 関連度順
          - text: 新着順
      - kind: progress
        eyebrow: PROGRESS
        title: 自動的な進捗報告
        text: 会話ターンの最後に Stop Hook で進捗を表示。毎回こちらから状況を聞く必要がありません。
        note: 追加のモデル推論や報告専用 Tool Call を要求しない設計です。表示形式はクライアントにより異なります。
        visualTitle: Turn-end report
        progress:
          percent: 60
          label: 60%
          detail: 3 of 5 tasks completed
          banner: "AFK · 検索機能の改善 [Doing] · 3/5 complete · Next: テストの追加"
      - kind: workspace
        eyebrow: WORKSPACE
        title: セッションを超えた継続
        text: 依頼・計画・タスク・質問をプロジェクト直下の .afk/ に保存。セッションをまたいでも状態を引き継げます。
        visualTitle: .afk/
        tree:
          - ".afk/"
          - "├── plans/"
          - "├── tasks/"
          - "├── requests/"
          - "├── questions/"
          - "└── afk.db"
  efficiency:
    eyebrow: Efficiency
    title: AIには、仕事に集中してもらう。
    lead: AI に管理業務をさせるのではなく、プログラムで処理できる決定論的な管理は AFK（Rust）へ移譲する設計方針をとっています。
    ai:
      badge: AI Agent
      title: AIが担当すること：推論・創造・実行
      items:
        - 計画の立案
        - 仕様の理解
        - 成果物の作成
        - 結果の検証
    afk:
      badge: AFK (Rust)
      title: AFKが担当すること：決定論的な管理処理
      items:
        - 状態の管理
        - 実行順序・依存関係の管理
        - 進捗の集計
        - データの永続化
    note: トークン効率と管理負担の低減は設計目標であり、標準エージェント単体との比較ベンチマークは実測データが揃い次第公開予定です。
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
          role: 進行支援・状態操作
        - name: Skills
          role: 仕事の進め方の指針
        - name: Hooks
          role: ターン末尾の進捗報告
      storageLabel: Project (.afk/)
      storage:
        - name: Markdown
          role: AI が標準ツールで読み書きする本文
        - name: SQLite
          role: AFK が管理する状態・順序
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
      sub: AI に仕事を任せる、新しい当たり前。
      primary:
        text: Agent Flow Kit をはじめる
        link: /ja/guide/installation
      secondary:
        text: GitHub
        link: https://github.com/FooQoo/agent-flow-kit
---

<LandingPage />
