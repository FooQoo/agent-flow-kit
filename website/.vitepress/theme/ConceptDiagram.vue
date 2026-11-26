<script setup lang="ts">
defineProps<{
  name: 'workflow-harness' | 'work-model' | 'positioning'
  lang?: 'ja' | 'en'
}>()
</script>

<template>
  <!-- 1. Workflow Harness -->
  <figure v-if="name === 'workflow-harness'" class="doc-diagram">
    <svg
      viewBox="0 0 760 310"
      role="img"
      :aria-label="
        lang === 'ja'
          ? '人間、AI エージェント、標準ツール、Agent Flow Kit、およびプロジェクト内ファイルの関係図'
          : 'Responsibility diagram showing Human, AI Agent, Native Tools, Agent Flow Kit, and project-local .afk files'
      "
    >
      <defs>
        <marker id="cd-wh-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
          <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-brand-1)" />
        </marker>
        <marker id="cd-wh-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
          <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-text-3)" />
        </marker>
      </defs>

      <rect x="230" y="16" width="300" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
      <text x="380" y="40" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">
        {{ lang === 'ja' ? 'あなた（人間）' : 'Human' }}
      </text>
      <text x="380" y="60" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="12">
        {{ lang === 'ja' ? '目的を伝える · 大事な判断 · 方向修正' : 'Goals · Key decisions · Direction changes' }}
      </text>

      <line x1="380" y1="74" x2="380" y2="112" stroke="var(--vp-c-text-3)" stroke-width="1.5" marker-start="url(#cd-wh-muted)" marker-end="url(#cd-wh-muted)" />
      <text x="392" y="97" fill="var(--vp-c-text-2)" font-size="11.5">
        {{ lang === 'ja' ? '普段通りの会話' : 'Natural conversation' }}
      </text>

      <rect x="210" y="116" width="340" height="60" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
      <text x="380" y="141" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">
        {{ lang === 'ja' ? 'いつもの AI エージェント（Claude Code / Codex）' : 'AI Agent (Claude Code / Codex)' }}
      </text>
      <text x="380" y="161" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="12">
        {{ lang === 'ja' ? '計画を立てる · コードを書く · 動作確認する' : 'Reasoning · Planning · Execution · Verification' }}
      </text>

      <path d="M 290 176 L 190 218" stroke="var(--vp-c-text-3)" stroke-width="1.5" fill="none" marker-end="url(#cd-wh-muted)" />
      <path d="M 470 176 L 560 218" stroke="var(--vp-c-brand-1)" stroke-width="1.8" fill="none" marker-end="url(#cd-wh-arrow)" />

      <rect x="30" y="222" width="310" height="66" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
      <text x="185" y="247" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">
        {{ lang === 'ja' ? '標準ツール（ファイル読み書き・コマンド）' : 'Native Tools (Read / Write / Edit / Shell)' }}
      </text>
      <text x="185" y="269" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="12">
        {{ lang === 'ja' ? 'ソースコードや .afk/*.md を直接編集' : 'Project files & .afk/*.md content' }}
      </text>

      <rect x="410" y="222" width="320" height="66" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
      <text x="570" y="247" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="13.5" font-weight="700">
        {{ lang === 'ja' ? 'Agent Flow Kit（プラグイン）' : 'Agent Flow Kit (Plugin)' }}
      </text>
      <text x="570" y="269" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="12">
        {{ lang === 'ja' ? '進捗・順番・完了チェックを .afk/afk.db で管理' : 'State, order & gates in .afk/afk.db' }}
      </text>

      <path d="M 680 222 L 680 45 L 538 45" stroke="var(--vp-c-brand-1)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#cd-wh-arrow)" />
      <text x="692" y="138" fill="var(--vp-c-brand-1)" font-size="11.5" font-weight="600">
        {{ lang === 'ja' ? '会話末尾の' : 'Turn-end' }}
      </text>
      <text x="692" y="154" fill="var(--vp-c-brand-1)" font-size="11.5" font-weight="600">
        {{ lang === 'ja' ? '進捗表示' : 'progress' }}
      </text>
    </svg>
    <figcaption>
      {{
        lang === 'ja'
          ? '責務の分離イメージ（AFK は人間と AI の全会話を中継するものではありません）'
          : 'Conceptual separation of responsibilities. AFK does not proxy every message between the human and the agent.'
      }}
    </figcaption>
  </figure>

  <!-- 2. Work Model -->
  <figure v-else-if="name === 'work-model'" class="doc-diagram">
    <svg
      viewBox="0 0 760 270"
      role="img"
      :aria-label="
        lang === 'ja'
          ? '依頼から計画、タスク実行、確認と完了へ進み、必要に応じて質問や追加依頼で計画を更新する流れ図'
          : 'Lifecycle diagram from Request to Plan, Task execution, and Verification, with optional Question and Follow-up Request loops'
      "
    >
      <defs>
        <marker id="cd-wm-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
          <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-brand-1)" />
        </marker>
        <marker id="cd-wm-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
          <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-text-3)" />
        </marker>
      </defs>

      <rect x="24" y="34" width="132" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
      <text x="90" y="59" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">
        {{ lang === 'ja' ? '1. 依頼' : '1. Request' }}
      </text>
      <text x="90" y="78" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">
        {{ lang === 'ja' ? 'やりたいことを記録' : 'Record goal' }}
      </text>

      <line x1="156" y1="63" x2="202" y2="63" stroke="var(--vp-c-brand-1)" stroke-width="1.8" marker-end="url(#cd-wm-arrow)" />

      <rect x="208" y="34" width="144" height="58" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
      <text x="280" y="59" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="14" font-weight="700">
        {{ lang === 'ja' ? '2. 計画' : '2. Plan' }}
      </text>
      <text x="280" y="78" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="11.5">
        {{ lang === 'ja' ? '順番と完了条件を整理' : 'Order & acceptance' }}
      </text>

      <line x1="352" y1="63" x2="398" y2="63" stroke="var(--vp-c-brand-1)" stroke-width="1.8" marker-end="url(#cd-wm-arrow)" />

      <rect x="404" y="34" width="144" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
      <text x="476" y="59" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">
        {{ lang === 'ja' ? '3. タスク実行' : '3. Execute' }}
      </text>
      <text x="476" y="78" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">
        {{ lang === 'ja' ? '1つずつ実装・記録' : 'Run & log evidence' }}
      </text>

      <line x1="548" y1="63" x2="594" y2="63" stroke="var(--vp-c-brand-1)" stroke-width="1.8" marker-end="url(#cd-wm-arrow)" />

      <rect x="600" y="34" width="136" height="58" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
      <text x="668" y="59" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="14" font-weight="700">
        {{ lang === 'ja' ? '4. 確認・完了' : '4. Done' }}
      </text>
      <text x="668" y="78" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="11.5">
        {{ lang === 'ja' ? '全条件チェック済み' : 'All checks verified' }}
      </text>

      <rect x="330" y="168" width="210" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--lp-pending-border, #d99b26)" stroke-width="1.5" />
      <text x="435" y="193" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">
        {{ lang === 'ja' ? '質問と回答（必要時のみ）' : 'Question (when needed)' }}
      </text>
      <text x="435" y="212" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">
        {{ lang === 'ja' ? '人が決めるべき点だけ確認' : 'Pause & seal human answer' }}
      </text>

      <path d="M 460 92 L 460 162" stroke="var(--vp-c-text-3)" stroke-width="1.5" stroke-dasharray="4 4" fill="none" marker-end="url(#cd-wm-muted)" />
      <path d="M 380 168 L 315 98" stroke="var(--vp-c-brand-1)" stroke-width="1.5" fill="none" marker-end="url(#cd-wm-arrow)" />
      <text x="328" y="144" fill="var(--vp-c-brand-1)" font-size="11">
        {{ lang === 'ja' ? '回答を計画へ反映' : 'Reflect answer' }}
      </text>

      <rect x="44" y="168" width="196" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
      <text x="142" y="193" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">
        {{ lang === 'ja' ? '途中の追加依頼' : 'Follow-up Request' }}
      </text>
      <text x="142" y="212" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">
        {{ lang === 'ja' ? '完了分を残して残りを更新' : 'Keep Done tasks & replan' }}
      </text>

      <path d="M 190 168 L 250 98" stroke="var(--vp-c-brand-1)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#cd-wm-arrow)" />
    </svg>
    <figcaption>
      {{
        lang === 'ja'
          ? '依頼から完了までの基本フローと、質問・途中変更による計画更新'
          : 'Core lifecycle from Request to verified Done, with optional Question and mid-work replanning loops.'
      }}
    </figcaption>
  </figure>

  <!-- 3. Positioning -->
  <figure v-else class="doc-diagram">
    <svg
      viewBox="0 0 760 270"
      role="img"
      :aria-label="
        lang === 'ja'
          ? '人間と既存 AI エージェントを中心に、AFK・開発方法論・タスク管理ツールがそれぞれ担う役割の位置づけ図'
          : 'Positioning map showing Human and AI Agent connected to AFK, optional methodologies, and optional shared boards'
      "
    >
      <defs>
        <marker id="cd-pos-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
          <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-brand-1)" />
        </marker>
        <marker id="cd-pos-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
          <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-text-3)" />
        </marker>
      </defs>

      <rect x="20" y="106" width="146" height="60" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
      <text x="93" y="131" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">
        {{ lang === 'ja' ? 'あなた（人間）' : 'Human' }}
      </text>
      <text x="93" y="151" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">
        {{ lang === 'ja' ? '目的と判断' : 'Goals & decisions' }}
      </text>

      <line x1="166" y1="136" x2="210" y2="136" stroke="var(--vp-c-text-3)" stroke-width="1.5" marker-start="url(#cd-pos-muted)" marker-end="url(#cd-pos-muted)" />

      <rect x="216" y="98" width="184" height="76" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
      <text x="308" y="128" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">
        {{ lang === 'ja' ? 'いつもの AI エージェント' : 'Existing AI Agent' }}
      </text>
      <text x="308" y="150" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">
        {{ lang === 'ja' ? 'Claude Code / Codex 等' : 'Claude Code / Codex' }}
      </text>

      <path d="M 400 118 L 466 54" stroke="var(--vp-c-brand-1)" stroke-width="2" fill="none" marker-end="url(#cd-pos-arrow)" />
      <path d="M 400 136 L 466 136" stroke="var(--vp-c-text-3)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#cd-pos-muted)" />
      <path d="M 400 154 L 466 218" stroke="var(--vp-c-text-3)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#cd-pos-muted)" />

      <rect x="472" y="22" width="268" height="62" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.8" />
      <text x="606" y="47" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="14" font-weight="700">
        {{ lang === 'ja' ? 'Agent Flow Kit（AFK）' : 'Agent Flow Kit (AFK)' }}
      </text>
      <text x="606" y="68" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="12">
        {{ lang === 'ja' ? '仕事の進行・完了確認・引き継ぎを支える' : 'Work execution harness' }}
      </text>

      <rect x="472" y="105" width="268" height="62" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
      <text x="606" y="130" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">
        AI-DLC / SDD / Spec Kit
      </text>
      <text x="606" y="151" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">
        {{ lang === 'ja' ? '必要に応じて選ぶ開発プロセス・仕様手法' : 'Optional development methodology' }}
      </text>

      <rect x="472" y="188" width="268" height="62" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
      <text x="606" y="213" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">
        Vercel Board / Backlog.md
      </text>
      <text x="606" y="234" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">
        {{ lang === 'ja' ? '必要に応じて使う共有ボード・タスク管理' : 'Optional shared board / backlog' }}
      </text>
    </svg>
    <figcaption>
      {{
        lang === 'ja'
          ? '各ツール・手法の主な役割の位置づけ（技術的な依存や公式連携を示すものではありません）'
          : 'Conceptual map of primary roles—not technical dependencies, official integrations, or mutually exclusive categories.'
      }}
    </figcaption>
  </figure>
</template>
