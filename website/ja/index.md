---
layout: page
title: Agent Flow Kit
description: AI に仕事を最後まで任せ切るための軽量プラグイン。途中の変更・完了チェック・別セッションからの再開をプロジェクト内の .afk/ で支えます。
sidebar: false
aside: false
landing:
  copy:
    label: コマンドをコピー
    done: コピーしました
  hero:
    eyebrow: 仕事は AI に。主導権はあなたに。
    title: "計画を立てたあとも、\nやり遂げる仕組みを。"
    lead: Agent Flow Kit は、Claude Code や Codex にまとまった仕事を任せ切るための軽量プラグインです。その場限りの計画メモで終わらせず、途中の仕様変更・完了条件のチェック・別セッションからの再開までをプロジェクト内の .afk/ フォルダで支えます。
    primary:
      text: はじめる
      link: /ja/guide/installation
    secondary:
      text: GitHub
      link: https://github.com/FooQoo/agent-flow-kit
    meta: Claude Code / Codex 対応 · ローカル保存（.afk/） · オープンソース
    demo:
      header: Claude Code / Codex + Agent Flow Kit
      ariaLabel: セッション1で計画と1点の確認を経て途中まで進め、翌日に別セッションで再開しつつ追加要望を計画へ反映する流れ
      boundaryLabel: 翌日 · 別のセッションで再開
      sessions:
        - label: "セッション 1"
          turns:
            - kind: prompt
              text: 検索機能を改善して、テストも追加して
            - kind: question
              text: "計画を 5 件に分けました。1点だけ確認です：並び順は関連度順と新着順のどちらにしますか？"
            - kind: prompt
              text: 関連度順でお願い
            - kind: saved
              text: "回答を反映して進め、3/5 件まで完了しました。（記録先: .afk/）"
        - label: "セッション 2"
          turns:
            - kind: prompt
              text: 前回の続きを進めて。検索結果が 0 件のときの表示も直して
            - kind: reply
              text: "前回の計画を読み込みました。終わった 3 件の記録は残したまま、残りに「0件表示の改善」を追加します。"
      status:
        title: 検索機能の改善
        revision: 第2版
        progressText: 3/6 完了 (50%)
        percent: 50
        tasks:
          - state: done
            text: "#1–#3 調査・仕様整理・検索ロジック"
            tag: 完了記録を保持
          - state: doing
            text: "#4 テストの追加"
            tag: 続きから再開
          - state: todo
            text: "#6 検索0件表示の改善"
            tag: 途中で追加
  problem:
    eyebrow: よくある課題
    title: 計画は立てられる。でも、そのあとが崩れる。
    lead: 標準のプランモードや Todo リストは「始める前の整理」には便利です。ただ、作業が長引いたり日をまたいだりすると、別の困りごとが出てきます。
    items:
      - icon: list-x
        title: 途中の追加要望で計画がズレる
        text: 作業中に要件が増えたり会話が長くなったりすると、最初の目的や完了条件が会話の奥に流れてしまう。
      - icon: circle-check
        title: 本当に終わったか証拠が残らない
        text: 「完了しました」と言われても、決めた条件を全部チェックしたのか後から確認できない。
      - icon: history
        title: セッションを閉じると振り出しに戻る
        text: 翌日や別のツールで続きをやろうとすると、計画・決めたこと・残りの作業を説明し直す必要がある。
      - icon: message-circle-question
        title: 決めた理由がチャットに埋もれる
        text: 「なぜこの仕様にしたか」という質問と回答が会話ログに埋もれ、引き継ぎや修正に活かせない。
    comparison:
      title: 標準のプランモード・Todo との違い
      lead: プランモードは 1 回の会話で消えるメモ。Agent Flow Kit は、完了まで記録とチェックを保ち続ける仕組みです。
      axisHeader: 比べるところ
      planModeHeader: 標準のプランモード / Todo
      afkHeader: Agent Flow Kit
      rows:
        - axis: 記録の残り方
          planMode: 1 回の会話の中だけ。セッションを閉じたり要約が入ったりすると薄れる。
          afk: プロジェクト内の .afk/ にファイルで保存。翌日や別ツールから続きを再開できる。
        - axis: 途中の要望追加
          planMode: チャット上で上書きされ、終わった部分と残りの境目が曖昧になりやすい。
          afk: 終わった作業の記録は残したまま、残りのタスクだけを新しい計画に書き直せる。
        - axis: 完了の確認
          planMode: AI の「終わりました」という自己申告だけで完了にできる。
          afk: 決めた完了条件を全部チェックし、確認結果を書き込むまで完了にならない。
        - axis: 進み具合の確認
          planMode: チャットを遡るか、「今どこまで？」と毎回聞く必要がある。
          afk: 会話の最後に自動で進捗を表示。わざわざ聞き直す手間がない。
    closing: 必要なのは「最初に計画を書く機能」ではなく、終わるまで計画と確認結果を守り続ける仕組みです。
  workflow:
    eyebrow: 進め方
    title: 頼む。答える。あとは任せる。
    lead: 迷う点がなければ質問は飛ばします。「計画だけ立てて」と「そのまま進めて」もきちんと区別します。
    steps:
      - step: "01"
        title: 依頼する
        actor: human
        actorLabel: あなた
        text: いつも通り、やりたいことを普通の言葉で伝える。
      - step: "02"
        title: 計画する
        actor: ai
        actorLabel: AI
        text: 作業を順番に分け、「何ができていれば完了か」を書き出す。
      - step: "03"
        title: 確認する
        actor: both
        actorLabel: あなた + AI
        text: 人が決めるべきことだけ質問し、回答をファイルに残す。
      - step: "04"
        title: 進める
        actor: ai
        actorLabel: AI
        text: 順番どおりに作業し、確認した結果を記録していく。
      - step: "05"
        title: 完了する
        actor: ai
        actorLabel: AI
        text: 完了条件を全部チェックしてから結果を報告する。
    adaptTitle: 途中で方向を変えても大丈夫。
    adaptText: 作業中でも新しい要望を追加できます。すでに終わった作業の記録は消さず、残りの計画だけを安全に書き直せます（割り込みできるタイミングは利用中のクライアントに依存します）。
  humanOnTheLoop:
    eyebrow: 役割分担
    title: 仕事は AI に。主導権はあなたに。
    lead: すべてを AI に丸投げするのではなく、普段は任せつつ、大事な判断だけ人が関われる進め方をとっています。
    human:
      badge: あなた
      title: あなたがやること
      items:
        - 目的を伝える
        - 大事な判断だけ答える
        - 必要に応じて方向を変える
    ai:
      badge: AI エージェント
      title: AI がやること
      items:
        - 作業を計画する
        - 実装して動作確認する
        - 確認結果を残して報告する
    harness:
      label: Agent Flow Kit
      text: 計画の保存・作業順の整理・完了チェック・進捗表示を裏側で支えます。AI の代わりに推論やコード生成を行うツールではありません。
    note: 質問が出るのは、意図が曖昧なとき・好みの分かれる仕様・元に戻しにくい操作・依頼範囲が広がるときだけです。細かい作業ごとに毎回承認を求めることはありません。
  features:
    eyebrow: 主な仕組み
    title: プランモードにはない、4 つの支え。
    items:
      - kind: revision
        eyebrow: 途中の計画変更
        title: 終わった記録を消さずに計画を書き直せる
        text: 作業中に要望が増えても、完了したタスクの記録はそのまま残し、残りの作業だけを新しい計画へ差し替えます。
        visualTitle: 計画 #1 の更新イメージ
        revision:
          from: 第1版
          to: 第2版
          rows:
            - id: "#1–#3"
              state: done
              badge: 記録を保持
              text: 調査・仕様整理・検索ロジック
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
              badge: 途中で追加
              text: 検索0件表示の改善
      - kind: gate
        eyebrow: 完了チェック
        title: 条件を満たすまで「完了」にしない
        text: 最初に決めた計画が勝手に書き換わらないように保護し、完了条件を全部確認して結果メモを書くまで完了にできません。
        visualTitle: 完了時の自動チェック
        gate:
          command: "タスク #4 の完了チェック"
          checks:
            - label: 確定した計画との一致
              value: 書き換えなし
            - label: 完了条件（全 3 項目）
              value: すべて確認済み
            - label: 確認結果のメモ
              value: 記録済み
          outcome: "✓ チェック通過 → タスク #4 を完了に変更"
      - kind: handoff
        eyebrow: 引き継ぎと保存
        title: 翌日や別ツールからそのまま再開できる
        text: 依頼・計画・質問・確認結果はすべてプロジェクト内の .afk/ に残ります。引き継ぎメモを残せば、別セッションや別ツールでも続きから進められます。
        visualTitle: .afk/ フォルダと引き継ぎメモ
        handoff:
          tree:
            - ".afk/"
            - "├── afk.db        # 進捗・順番・変更チェック"
            - "├── plans/        # 計画の履歴"
            - "├── tasks/        # タスク・完了条件・確認結果"
            - "├── requests/     # 最初と途中の依頼"
            - "└── questions/    # 質問と回答の記録"
          noteLabel: "引き継ぎメモ（セッション 1 → セッション 2）"
          noteText: "検索ロジックは実装済み。残りはテスト追加と 0 件表示の確認。"
      - kind: progress
        eyebrow: 自動の進捗表示
        title: 毎回聞かなくても、最後に進捗が出る
        text: 会話の終わりに自動で進捗を 1 行表示します。AI に追加で考えさせるのではなく、保存データから直接まとめるので軽快です。
        note: 表示の見た目は Claude Code や Codex など利用するクライアントによって異なります。
        visualTitle: 会話の最後に表示される進捗
        progress:
          percent: 60
          tag: AI の追加思考なし
          detail: .afk/ の保存データから直接集計
          banner: "AFK · 検索機能の改善 [進行中] · 3/5 完了 · 次: テストの追加"
  efficiency:
    eyebrow: 役割の切り分け
    title: 「お願い」ではなく、プログラムでルールを守る。
    lead: プロンプトで「忘れないで」と頼むだけでは、会話が長くなると崩れます。進捗や順番の管理を軽いプログラム（Rust + SQLite）に任せることで、AI は考える仕事と実装に集中できます。
    ai:
      badge: AI エージェント
      title: AI が担当すること（考える・作る・確かめる）
      items:
        - 計画を立てる・見直す
        - 仕様を読み解いて実装する
        - コードや文章を作る
        - 実際に動かして結果を確かめる
    afk:
      badge: Agent Flow Kit
      title: AFK が担当すること（記録する・順番を守る）
      items:
        - 作業の状態と順番の管理
        - 計画が勝手に変わっていないかの確認
        - 未回答の質問や未反映の依頼のチェック
        - 進捗の集計と .afk/ への保存
    note: 管理の手間やトークン消費を減らすことを設計目標にしています。実際の比較データは計測が整い次第公開します。
  architecture:
    eyebrow: しくみ
    title: 新しい開発環境は、必要ありません。
    lead: いつもの AI エージェントを置き換えるのではなく、プラグインとして仕事の進め方だけを足します。
    diagram:
      ariaLabel: Claude Code や Codex が Agent Flow Kit のプラグインを通じてプロジェクト内の .afk/（Markdown と SQLite）を扱う構成図
      clientsLabel: いつもの AI エージェント
      clients:
        - Claude Code
        - Codex
      kitLabel: Agent Flow Kit（プラグイン）
      kit:
        - name: MCP
          role: 状態管理と完了チェック
        - name: Skills
          role: 進め方のガイド
        - name: Hooks
          role: 会話末尾の進捗表示
      storageLabel: プロジェクト内（.afk/）
      storage:
        - name: Markdown
          role: 計画や結果の本文
        - name: SQLite
          role: 状態と順番の記録
      directLabel: 標準のファイル操作で直接読み書き
    items:
      - icon: laptop
        title: ローカル保存
        text: 依頼や計画のデータは自分のプロジェクト内（.afk/）だけに保存されます。
      - icon: cpu
        title: 軽い単体バイナリ
        text: 追加のランタイムや常駐サーバーを立ち上げる必要はありません。
      - icon: plug
        title: プラグインで追加
        text: 専用の管理画面や別ツールへの乗り換えは不要です。
      - icon: code
        title: オープンソース
        text: データの持ち方や仕組みをコードでそのまま確認できます。
    note: ※「ローカル保存」は AFK が記録するファイルの保存先のことです。Claude Code や Codex から AI モデルへの通信自体は、各クライアントの設定どおりに行われます。
  start:
    eyebrow: はじめかた
    title: インストールしたら、いつも通りに。
    lead: Claude Code または Codex にプラグインを追加すれば、すぐに使い始められます。
    steps:
      - title: プラグインを追加する
        text: Claude Code または Codex に Agent Flow Kit を登録します。
      - title: プロジェクトを開く
        text: いつものリポジトリとターミナルをそのまま開きます。
      - title: 普通に仕事を頼む
        text: 自然な言葉で依頼します。初回起動時に必要な実行ファイルが自動で準備されます。
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
    installNote: 初回起動時のみ GitHub Releases から対象 OS 用の実行ファイルを自動取得して検証します。オフラインでの導入方法は
    guideLink:
      text: インストールガイドを見る
      link: /ja/guide/installation
    cta:
      tagline: "仕事は AI に。\n主導権はあなたに。"
      sub: 計画を立てたあとも、最後までやり遂げるための軽量プラグイン。
      primary:
        text: Agent Flow Kit をはじめる
        link: /ja/guide/installation
      secondary:
        text: GitHub
        link: https://github.com/FooQoo/agent-flow-kit
---

<LandingPage />
