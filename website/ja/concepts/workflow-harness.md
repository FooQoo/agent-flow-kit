---
title: 進行を支える仕組み
description: Agent Flow Kit がなぜ既存エージェントを置き換えず、状態を持つ軽量な仕組みとして設計されているのか。
---

# 進行を支える仕組み

*「なぜプロンプトやプランモードだけでなく、AFK のような仕組みが必要なのか」をわかりやすく説明します。*

> AI が仕事をする。AFK がその進め方を支える。人間が主導権を握る。

## 「コードを書けること」と「仕事をやり遂げること」は別

今の AI エージェントは、コードを書いたりテストを動かしたりする個別の作業がとても得意です。一方で、まとまった依頼を最初から最後までやり遂げるには、目的を忘れないこと・順番を守ること・迷ったら確認すること・終わった証拠を残すこと・翌日へ引き継ぐことといった **進め方の支え** が必要になります。

## AFK が裏側で支えている 6 つのこと

- **目的を残す** — 最初の依頼と途中の追加要望をそのままファイルに残す。
- **次の作業を迷わない** — 会話の記憶に頼らず、保存された状態から次にやるべきことを決める。
- **判断を埋もれさせない** — 人が決めるべき質問を切り出し、回答を計画に反映させる。
- **進捗を正確に持つ** — 待機・未着手・進行中・完了をデータベース（SQLite）で管理する。
- **完了条件を守る** — 着手前に「何ができていれば完了か」を決め、全項目を確認して結果メモを書くまで完了にしない。
- **最後に進捗を伝える** — 会話の終わりに現在状況を 1 行で自動表示する。

## 責務の分担イメージ

<figure class="doc-diagram">
  <svg viewBox="0 0 760 310" role="img" aria-label="人間、AI エージェント、標準ツール、Agent Flow Kit、およびプロジェクト内ファイルの関係図">
    <defs>
      <marker id="wh-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-brand-1)" />
      </marker>
      <marker id="wh-arrow-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-text-3)" />
      </marker>
    </defs>

    <!-- Human -->
    <rect x="230" y="16" width="300" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="380" y="40" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">あなた（人間）</text>
    <text x="380" y="60" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="12">目的を伝える · 大事な判断 · 方向修正</text>

    <!-- Human <-> Agent -->
    <line x1="380" y1="74" x2="380" y2="112" stroke="var(--vp-c-text-3)" stroke-width="1.5" marker-start="url(#wh-arrow-muted)" marker-end="url(#wh-arrow-muted)" />
    <text x="392" y="97" fill="var(--vp-c-text-2)" font-size="11.5">普段通りの会話</text>

    <!-- AI Agent -->
    <rect x="210" y="116" width="340" height="60" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="380" y="141" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">いつもの AI エージェント（Claude Code / Codex）</text>
    <text x="380" y="161" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="12">計画を立てる · コードを書く · 動作確認する</text>

    <!-- Agent -> Native tools -->
    <path d="M 290 176 L 190 218" stroke="var(--vp-c-text-3)" stroke-width="1.5" fill="none" marker-end="url(#wh-arrow-muted)" />
    <!-- Agent -> AFK -->
    <path d="M 470 176 L 560 218" stroke="var(--vp-c-brand-1)" stroke-width="1.8" fill="none" marker-end="url(#wh-arrow)" />

    <!-- Native tools -->
    <rect x="30" y="222" width="310" height="66" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="185" y="247" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">標準ツール（ファイル読み書き・コマンド）</text>
    <text x="185" y="269" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="12">ソースコードや .afk/*.md を直接編集</text>

    <!-- AFK -->
    <rect x="410" y="222" width="320" height="66" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="570" y="247" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="13.5" font-weight="700">Agent Flow Kit（プラグイン）</text>
    <text x="570" y="269" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="12">進捗・順番・完了チェックを .afk/afk.db で管理</text>

    <!-- AFK -> Human (progress report) -->
    <path d="M 680 222 L 680 45 L 538 45" stroke="var(--vp-c-brand-1)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#wh-arrow)" />
    <text x="692" y="138" fill="var(--vp-c-brand-1)" font-size="11.5" font-weight="600">会話末尾の</text>
    <text x="692" y="154" fill="var(--vp-c-brand-1)" font-size="11.5" font-weight="600">進捗表示</text>
  </svg>
  <figcaption>責務の分離イメージ（AFK は人間と AI の全会話を中継するものではありません）</figcaption>
</figure>

## なぜガイド（プロンプト）だけでは足りないのか

手順をプロンプトやスキルに書くだけでも一定の効果はありますが、会話が長くなると指示が薄れたり、セッションを閉じた瞬間に文脈がリセットされたりします。

AFK は簡潔なガイドに加えて、プロジェクト内のデータベース（`.afk/afk.db`）と Markdown ファイル（`.afk/*.md`）をプログラム側でチェックすることで、順番の守り忘れや完了条件の抜け漏れを防ぎます。

## 置き換えるのではなく、拡張する

AFK は利用中の AI モデル、クライアント（Claude Code / Codex 等）、標準のファイル操作や権限設定をそのまま使います。計画や結果の本文は AI が普段のファイルツールで書き、AFK は「状態管理」と「完了チェック」に徹します。

## できること・できないことの境界

AFK は仕事の進め方を安定させるための仕組みですが、万能ではありません。

- ガイド（Skill）は行動の目安であり、AI がすべての指示に 100% 従うことを強制するものではありません。
- `.afk/` 以外のファイル編集やコマンド実行をすべて監視・制限するものではありません。
- AI の技術的な判断の正しさや、どんな仕事でも自動で完遂することを保証するものではありません。

## 次に読むページ

- [仕事の進め方と 4 つの要素](/ja/concepts/work-model) — 依頼・計画・タスク・質問の関係。
- [人間と AI の役割分担](/ja/concepts/human-in-on-the-loop) — どこまで任せ、いつ確認するか。
- [既存の手法・ツールとの違い](/ja/concepts/positioning) — 他の開発手法や管理ツールとの違い。
