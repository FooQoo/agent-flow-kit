---
title: 既存の手法・ツールとの違い
description: AI-DLC・SDD・GitHub Spec Kit・Vercel Board・Backlog.md と Agent Flow Kit の目的・使い分けの整理。
---

# 既存の手法・ツールとの違い

*他の AI 開発手法やタスク管理ツールと AFK がどう違い、どう使い分けられるのかを整理したページです。*

> Agent Flow Kit は、AI に仕事を最後まで任せ切るための軽量な仕組みです。開発プロセス全体を決める方法論や、チーム向けのタスクボードとは出発点が異なります。計画やタスクなど重なる部分もありますが、目的に応じて使い分けられます。

## A. AI 開発の方法論：AI-DLC / SDD / Spec Kit

- **[AI-DLC Workflows](https://awslabs.github.io/aidlc-workflows/guide/00-introduction/)**: AI を使ったソフトウェア開発を再現しやすい工程へ整理する方法論とその実装。
- **SDD（仕様駆動開発）**: 仕様書を起点に開発を進めるアプローチ。
- **[GitHub Spec Kit](https://github.github.com/spec-kit/index.html)**: 仕様駆動開発のためのツールキット。テンプレートだけでなく工程のガイドも備えています。

| 比べるところ | AI-DLC / SDD（一般的な主眼） | Agent Flow Kit |
| --- | --- | --- |
| 主な目的 | ソフトウェア開発の工程や仕様書を整える | 任せた仕事を完了まで進める支えになる |
| 中心になるもの | 開発フェーズ・仕様書・成果物 | 依頼・計画・タスク・質問 |
| 使える範囲 | 主にソフトウェア開発 | 開発を起点に、調査や文書作成にも使える |
| 開発手法の前提 | 具体的な開発工程や仕様のガイドを持つ | 特定の開発手法を前提にしない |
| 人の関わり方 | 各工程のレビューや承認など | 必要な質問への回答、方向修正、進捗確認 |

**関係性:** AFK はこれらの手法と対立するものではありませんが、ガイド同士の競合や状態の二重管理についてはまだ検証していないため、現時点では公式な併用サポートを謳っていません。

## B. ボード・タスク管理ツール：Vercel Board / Backlog.md

- **[Vercel Board](https://github.com/vercel/board)**: チームと AI エージェントがチャンネル・スレッド・タスクで仕事を共有・協調する作業場。
- **[Backlog.md](https://github.com/MrLesk/Backlog.md)**: Git リポジトリで人間と AI のタスクを Markdown で管理するツール。[Manifesto](https://github.com/MrLesk/Backlog.md/blob/main/MANIFESTO.md) でも計画・完了条件・履歴・AI 向けルールを重視しており、AFK と重なる部分が大きいツールです。

| 比べるところ | Vercel Board | Backlog.md | Agent Flow Kit |
| --- | --- | --- | --- |
| 主な目的 | チームと AI の仕事共有と協調 | Git 上でのタスク管理と人・AI の協働 | いつもの AI エージェントの進行支援 |
| 中心になるもの | チャンネル・スレッド・タスク | タスク・計画・完了条件・履歴 | 依頼から完了・引き継ぎまでの流れ |
| 主な操作画面 | 共有の作業画面 | タスク管理の UI や CLI | 普段の会話そのまま |
| 人の関わり方 | 共有・対話・分担 | タスクやレビューを通じた管理 | 依頼・大事な判断・方向修正 |
| AFK の特徴 | — | — | 専用画面を必要とせず、既存エージェントに足すだけ |

Backlog.md にも計画や完了条件・AI 向けルールがあるため、「他は管理だけ、AFK だけが実行を支える」といった単純な切り分けはしません。主な違いは **普段の会話の延長でそのまま使えるか、管理の場を中心に据えるか** にあります。

## C. 全体の位置づけ

<figure class="doc-diagram">
  <svg viewBox="0 0 760 270" role="img" aria-label="人間と既存 AI エージェントを中心に、AFK・開発方法論・タスク管理ツールがそれぞれ担う役割の位置づけ図">
    <defs>
      <marker id="pos-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-brand-1)" />
      </marker>
      <marker id="pos-arrow-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-text-3)" />
      </marker>
    </defs>

    <!-- Human -->
    <rect x="20" y="106" width="146" height="60" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="93" y="131" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">あなた（人間）</text>
    <text x="93" y="151" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">目的と判断</text>

    <line x1="166" y1="136" x2="210" y2="136" stroke="var(--vp-c-text-3)" stroke-width="1.5" marker-start="url(#pos-arrow-muted)" marker-end="url(#pos-arrow-muted)" />

    <!-- Existing Agent -->
    <rect x="216" y="98" width="184" height="76" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="308" y="128" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">いつもの AI エージェント</text>
    <text x="308" y="150" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">Claude Code / Codex 等</text>

    <!-- Arrows to right column -->
    <path d="M 400 118 L 466 54" stroke="var(--vp-c-brand-1)" stroke-width="2" fill="none" marker-end="url(#pos-arrow)" />
    <path d="M 400 136 L 466 136" stroke="var(--vp-c-text-3)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#pos-arrow-muted)" />
    <path d="M 400 154 L 466 218" stroke="var(--vp-c-text-3)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#pos-arrow-muted)" />

    <!-- Right 1: AFK -->
    <rect x="472" y="22" width="268" height="62" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.8" />
    <text x="606" y="47" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="14" font-weight="700">Agent Flow Kit（AFK）</text>
    <text x="606" y="68" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="12">仕事の進行・完了確認・引き継ぎを支える</text>

    <!-- Right 2: Methodologies -->
    <rect x="472" y="105" width="268" height="62" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="606" y="130" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">AI-DLC / SDD / Spec Kit</text>
    <text x="606" y="151" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">必要に応じて選ぶ開発プロセス・仕様手法</text>

    <!-- Right 3: Boards -->
    <rect x="472" y="188" width="268" height="62" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="606" y="213" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">Vercel Board / Backlog.md</text>
    <text x="606" y="234" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">必要に応じて使う共有ボード・タスク管理</text>
  </svg>
  <figcaption>各ツール・手法の主な役割の位置づけ（技術的な依存や公式連携を示すものではありません）</figcaption>
</figure>

## まとめ

> 開発手法を押し付けない。管理画面を必須にしない。エージェントを置き換えない。ただ、AI が仕事をやり遂げるのを支える。

*根拠・確認日: 上記リンク先の公式資料を **2026-11-26** に確認した範囲での定性的な整理です。実機による定量比較は未実施です。*

## 次に読むページ

- [進行を支える仕組み](/ja/concepts/workflow-harness) — AFK の設計思想を読む。
- [MCP リファレンス](/ja/reference/mcp) — 6 つのツールの仕様を確認する。
