---
title: 仕事の進め方と 4 つの要素
description: 依頼・計画・タスク・質問の 4 つと完了条件によって、次にやることと完了の基準をはっきりさせる仕組み。
---

# 仕事の進め方と 4 つの要素

*AFK がどのように仕事を整理し、「次に何をするか」「何をもって完了とするか」をわかりやすく保っているかを説明します。*

> 仕事を 4 つの記録に分けることで、次にやることと完了の基準をいつでも確認できるようにします。

## 4 つの基本要素

AFK はすべての仕事を `.afk/` フォルダ内の 4 種類の Markdown ファイルとして残します。

| 要素 | 役割 | 保存先 |
| --- | --- | --- |
| **依頼（Request）** | 最初の依頼と、途中で足した要望の記録。 | `.afk/requests/NNN.md` |
| **計画（Plan）** | 依頼をどう進めるかの全体像、タスクの順番、全体の完了条件。 | `.afk/plans/NNN-rMMM.md` |
| **タスク（Task）** | 実際に進める作業単位。前提順序、完了条件、作業履歴、確認結果を持つ。 | `.afk/tasks/NNN.md` |
| **質問（Question）** | AI から人への確認と、人が答えた内容の記録。 | `.afk/questions/NNN.md` |

## 仕事の進み方（ライフサイクル）

<figure class="doc-diagram">
  <svg viewBox="0 0 760 270" role="img" aria-label="依頼から計画、タスク実行、確認と完了へ進み、必要に応じて質問や追加依頼で計画を更新する流れ図">
    <defs>
      <marker id="wm-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-brand-1)" />
      </marker>
      <marker id="wm-arrow-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-text-3)" />
      </marker>
    </defs>

    <!-- Main flow row -->
    <rect x="24" y="34" width="132" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="90" y="59" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">1. 依頼</text>
    <text x="90" y="78" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">やりたいことを記録</text>

    <line x1="156" y1="63" x2="202" y2="63" stroke="var(--vp-c-brand-1)" stroke-width="1.8" marker-end="url(#wm-arrow)" />

    <rect x="208" y="34" width="144" height="58" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="280" y="59" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="14" font-weight="700">2. 計画</text>
    <text x="280" y="78" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="11.5">順番と完了条件を整理</text>

    <line x1="352" y1="63" x2="398" y2="63" stroke="var(--vp-c-brand-1)" stroke-width="1.8" marker-end="url(#wm-arrow)" />

    <rect x="404" y="34" width="144" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="476" y="59" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">3. タスク実行</text>
    <text x="476" y="78" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">1つずつ実装・記録</text>

    <line x1="548" y1="63" x2="594" y2="63" stroke="var(--vp-c-brand-1)" stroke-width="1.8" marker-end="url(#wm-arrow)" />

    <rect x="600" y="34" width="136" height="58" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="668" y="59" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="14" font-weight="700">4. 確認・完了</text>
    <text x="668" y="78" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="11.5">全条件チェック済み</text>

    <!-- Question box -->
    <rect x="330" y="168" width="210" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--lp-pending-border, #d99b26)" stroke-width="1.5" />
    <text x="435" y="193" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">質問と回答（必要時のみ）</text>
    <text x="435" y="212" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">人が決めるべき点だけ確認</text>

    <!-- Plan / Task -> Question -> Plan -->
    <path d="M 460 92 L 460 162" stroke="var(--vp-c-text-3)" stroke-width="1.5" stroke-dasharray="4 4" fill="none" marker-end="url(#wm-arrow-muted)" />
    <path d="M 380 168 L 315 98" stroke="var(--vp-c-brand-1)" stroke-width="1.5" fill="none" marker-end="url(#wm-arrow)" />
    <text x="328" y="144" fill="var(--vp-c-brand-1)" font-size="11">回答を計画へ反映</text>

    <!-- Follow-up request box -->
    <rect x="44" y="168" width="196" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="142" y="193" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">途中の追加依頼</text>
    <text x="142" y="212" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">完了分を残して残りを更新</text>

    <path d="M 190 168 L 250 98" stroke="var(--vp-c-brand-1)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#wm-arrow)" />
  </svg>
  <figcaption>依頼から完了までの基本フローと、質問・途中変更による計画更新</figcaption>
</figure>

## 「計画だけ」と「そのまま実行」の区別

AFK は、すべての作業に毎回承認を求めるのではなく、あなたの頼み方に合わせて動きを変えます。

- **計画だけ頼んだとき（待機状態）**: 計画を保存したあと待機状態（`Backlog`）で止まり、「進めて」と伝えるまで実装を始めません。
- **そのまま実装を頼んだとき**: 計画の保存と同時に着手可能（`Todo`）になり、迷う点がなければそのまま進みます。

## 完了条件のチェック

すべての計画とタスクには、作業を始める前に **完了条件（`## Acceptance`）** を箇条書きで書きます。確定した内容は自動で保護され、完了時には「全項目を順番に確認したこと」と「確認結果のメモ（`## Result`）」が揃っていないと完了にできません。

## 途中の要望追加と再計画

作業の途中で要件が増えたり方針が変わったりしたときは、新しい依頼を追加したうえで **新しい版の計画** を作ります。このとき、すでに終わったタスクは消されず、確認済みの記録としてそのまま残ります。

## 別セッションからの再開

依頼・計画・タスク・質問・引き継ぎメモがすべて `.afk/` に残っているため、新しいセッションを開いたときも前回の続きから安全に再開できます。

## 次に読むページ

- [人間と AI の役割分担](/ja/concepts/human-in-on-the-loop) — どこまで任せ、いつ確認するか。
- [MCP リファレンス](/ja/reference/mcp) — 6 つのツールの詳しい仕様。
