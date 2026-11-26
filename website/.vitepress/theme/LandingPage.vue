<script setup lang="ts">
import { computed, ref } from 'vue'
import { useData, withBase } from 'vitepress'
import LandingIcon from './LandingIcon.vue'
import HeroDemoCarousel from './HeroDemoCarousel.vue'

type ProblemItem = { icon: string; title: string; text: string }
type DiffRow = { axis: string; planMode: string; afk: string }
type WorkflowStep = { step: string; title: string; actor: 'human' | 'ai' | 'both'; actorLabel: string; text: string }
type FeatureItem = {
  kind: 'revision' | 'gate' | 'handoff' | 'progress'
  eyebrow: string
  title: string
  text: string
  note?: string
  visualTitle: string
  revision?: {
    from: string
    to: string
    rows: { id: string; state: 'done' | 'retired' | 'added' | 'todo'; badge: string; text: string }[]
  }
  gate?: {
    command: string
    checks: { label: string; value: string }[]
    outcome: string
  }
  handoff?: {
    tree: string[]
    noteLabel: string
    noteText: string
  }
  progress?: {
    percent: number
    detail: string
    banner: string
    tag: string
  }
}
type TrustItem = { icon: string; title: string; text: string }
type InstallClient = { id: string; label: string; commands: string[] }

const { frontmatter } = useData()
const c = computed(() => frontmatter.value.landing)

const href = (link: string) => (link.startsWith('/') ? withBase(link) : link)
const isExternal = (link: string) => /^https?:\/\//.test(link)
const lines = (text: string) => text.split('\n')

const selectedClient = ref('claude-code')
const activeInstall = computed<InstallClient>(() => {
  const clients: InstallClient[] = c.value?.start?.clients ?? []
  return clients.find((item) => item.id === selectedClient.value) ?? clients[0]
})

const copiedKey = ref('')
async function copyText(key: string, text: string) {
  try {
    await navigator.clipboard.writeText(text)
    copiedKey.value = key
    setTimeout(() => {
      if (copiedKey.value === key) copiedKey.value = ''
    }, 2000)
  } catch {
    // Clipboard unavailable; commands remain selectable.
  }
}
</script>

<template>
  <div v-if="c" class="landing">
    <!-- 01. Hero -->
    <section class="lp-hero">
      <div class="lp-container lp-hero__grid">
        <div class="lp-hero__copy">
          <p class="lp-hero__eyebrow">{{ c.hero.eyebrow }}</p>
          <h1 class="lp-hero__title">
            <template v-for="(line, i) in lines(c.hero.title)" :key="i">
              <br v-if="i" />{{ line }}
            </template>
          </h1>
          <p class="lp-hero__lead">{{ c.hero.lead }}</p>
          <div class="lp-actions">
            <a
              class="lp-button lp-button--brand"
              :href="href(c.hero.primary.link)"
              :target="isExternal(c.hero.primary.link) ? '_blank' : undefined"
              :rel="isExternal(c.hero.primary.link) ? 'noopener noreferrer' : undefined"
            >
              {{ c.hero.primary.text }}
            </a>
            <a
              class="lp-button"
              :href="href(c.hero.secondary.link)"
              :target="isExternal(c.hero.secondary.link) ? '_blank' : undefined"
              :rel="isExternal(c.hero.secondary.link) ? 'noopener noreferrer' : undefined"
            >
              {{ c.hero.secondary.text }}
            </a>
          </div>
          <p class="lp-hero__meta">{{ c.hero.meta }}</p>
        </div>

        <div class="lp-hero__visual">
          <HeroDemoCarousel :demo="c.hero.demo" />
        </div>
      </div>
    </section>

    <!-- 02. Problem + Plan Mode Comparison -->
    <section class="lp-section">
      <div class="lp-container">
        <header class="lp-heading">
          <p class="lp-eyebrow">{{ c.problem.eyebrow }}</p>
          <h2>{{ c.problem.title }}</h2>
          <p v-if="c.problem.lead" class="lp-lead">{{ c.problem.lead }}</p>
        </header>
        <ul class="lp-problem-grid">
          <li v-for="item in c.problem.items as ProblemItem[]" :key="item.title" class="lp-card lp-problem-card">
            <span class="lp-problem-card__icon">
              <LandingIcon :name="item.icon" :size="20" />
            </span>
            <h3>{{ item.title }}</h3>
            <p>{{ item.text }}</p>
          </li>
        </ul>

        <div v-if="c.problem.comparison" class="lp-diff">
          <div class="lp-diff__head">
            <h3>{{ c.problem.comparison.title }}</h3>
            <p>{{ c.problem.comparison.lead }}</p>
          </div>
          <div class="lp-diff__table" role="table" :aria-label="c.problem.comparison.title">
            <div class="lp-diff__row lp-diff__row--head" role="row">
              <span role="columnheader">{{ c.problem.comparison.axisHeader }}</span>
              <span role="columnheader">{{ c.problem.comparison.planModeHeader }}</span>
              <span role="columnheader" class="is-afk">{{ c.problem.comparison.afkHeader }}</span>
            </div>
            <div
              v-for="row in c.problem.comparison.rows as DiffRow[]"
              :key="row.axis"
              class="lp-diff__row"
              role="row"
            >
              <span class="lp-diff__axis" role="cell">{{ row.axis }}</span>
              <span class="lp-diff__muted" role="cell">{{ row.planMode }}</span>
              <span class="lp-diff__afk" role="cell">
                <LandingIcon name="check" :size="15" class="lp-ink--brand" />
                <span>{{ row.afk }}</span>
              </span>
            </div>
          </div>
        </div>

        <p class="lp-problem__closing">{{ c.problem.closing }}</p>
      </div>
    </section>

    <!-- 03. Workflow -->
    <section class="lp-section lp-section--soft">
      <div class="lp-container">
        <header class="lp-heading">
          <p class="lp-eyebrow">{{ c.workflow.eyebrow }}</p>
          <h2>{{ c.workflow.title }}</h2>
          <p v-if="c.workflow.lead" class="lp-lead">{{ c.workflow.lead }}</p>
        </header>
        <ol class="lp-workflow">
          <li
            v-for="step in c.workflow.steps as WorkflowStep[]"
            :key="step.step"
            class="lp-card lp-workflow__step"
            :class="{ 'lp-card--human': step.actor === 'human' }"
          >
            <div class="lp-card__meta">
              <span class="lp-card__num">{{ step.step }}</span>
              <span class="lp-chip" :class="`lp-chip--${step.actor}`">{{ step.actorLabel }}</span>
            </div>
            <h3>{{ step.title }}</h3>
            <p>{{ step.text }}</p>
          </li>
        </ol>
        <div class="lp-workflow__note">
          <LandingIcon name="refresh" :size="18" class="lp-workflow__note-icon" />
          <div>
            <p class="lp-workflow__note-title">{{ c.workflow.adaptTitle }}</p>
            <p class="lp-workflow__note-text">{{ c.workflow.adaptText }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- 04. Human on the Loop -->
    <section class="lp-section">
      <div class="lp-container">
        <header class="lp-heading">
          <p class="lp-eyebrow">{{ c.humanOnTheLoop.eyebrow }}</p>
          <h2>{{ c.humanOnTheLoop.title }}</h2>
          <p class="lp-lead">{{ c.humanOnTheLoop.lead }}</p>
        </header>
        <div class="lp-hotl">
          <div class="lp-hotl__roles">
            <article class="lp-card lp-hotl__card">
              <p class="lp-chip lp-chip--human">{{ c.humanOnTheLoop.human.badge }}</p>
              <h3>{{ c.humanOnTheLoop.human.title }}</h3>
              <ul class="lp-hotl__list">
                <li v-for="item in c.humanOnTheLoop.human.items" :key="item">
                  <LandingIcon name="check" :size="16" class="lp-ink--human" />
                  <span>{{ item }}</span>
                </li>
              </ul>
            </article>
            <article class="lp-card lp-hotl__card">
              <p class="lp-chip lp-chip--ai">{{ c.humanOnTheLoop.ai.badge }}</p>
              <h3>{{ c.humanOnTheLoop.ai.title }}</h3>
              <ul class="lp-hotl__list">
                <li v-for="item in c.humanOnTheLoop.ai.items" :key="item">
                  <LandingIcon name="check" :size="16" class="lp-ink--brand" />
                  <span>{{ item }}</span>
                </li>
              </ul>
            </article>
          </div>
          <div class="lp-hotl__harness">
            <span class="lp-hotl__harness-label">{{ c.humanOnTheLoop.harness.label }}</span>
            <p>{{ c.humanOnTheLoop.harness.text }}</p>
          </div>
          <p class="lp-hotl__note">{{ c.humanOnTheLoop.note }}</p>
        </div>
      </div>
    </section>

    <!-- 05. Features (AFK-specific mechanisms) -->
    <section class="lp-section lp-section--soft">
      <div class="lp-container">
        <header class="lp-heading">
          <p class="lp-eyebrow">{{ c.features.eyebrow }}</p>
          <h2>{{ c.features.title }}</h2>
        </header>
        <div class="lp-features">
          <article
            v-for="f in c.features.items as FeatureItem[]"
            :key="f.title"
            class="lp-feature"
          >
            <div class="lp-feature__panel">
              <div class="lp-mock">
                <div class="lp-mock__head">
                  <span class="lp-mock__tag">{{ f.eyebrow }}</span>
                  <span class="lp-mock__title">{{ f.visualTitle }}</span>
                </div>

                <!-- Revision replan visual -->
                <div v-if="f.kind === 'revision' && f.revision" class="lp-mock__rev">
                  <p class="lp-mock__rev-meta">{{ f.revision.from }} → <strong>{{ f.revision.to }}</strong></p>
                  <ul class="lp-mock__list">
                    <li v-for="row in f.revision.rows" :key="row.id" :class="`is-${row.state}`">
                      <span class="lp-mock__id">{{ row.id }}</span>
                      <span class="lp-mock__row-text">{{ row.text }}</span>
                      <span class="lp-mock__pill" :class="`is-${row.state}`">{{ row.badge }}</span>
                    </li>
                  </ul>
                </div>

                <!-- Deterministic verification gate visual -->
                <div v-else-if="f.kind === 'gate' && f.gate" class="lp-mock__gate">
                  <p class="lp-mock__cmd">{{ f.gate.command }}</p>
                  <ul class="lp-mock__checks">
                    <li v-for="chk in f.gate.checks" :key="chk.label">
                      <span class="lp-mock__ok" aria-hidden="true">✓</span>
                      <span class="lp-mock__chk-label">{{ chk.label }}</span>
                      <span class="lp-mock__chk-val">{{ chk.value }}</span>
                    </li>
                  </ul>
                  <p class="lp-mock__outcome">{{ f.gate.outcome }}</p>
                </div>

                <!-- Handoff + .afk/ visual -->
                <div v-else-if="f.kind === 'handoff' && f.handoff" class="lp-mock__handoff">
                  <pre class="lp-mock__tree"><code>{{ f.handoff.tree.join('\n') }}</code></pre>
                  <div class="lp-mock__handoff-note">
                    <span>{{ f.handoff.noteLabel }}</span>
                    <p>{{ f.handoff.noteText }}</p>
                  </div>
                </div>

                <!-- Stop Hook progress visual -->
                <div v-else-if="f.kind === 'progress' && f.progress" class="lp-mock__progress">
                  <div class="lp-mock__progress-row">
                    <strong>{{ f.progress.percent }}%</strong>
                    <span class="lp-mock__pill is-done">{{ f.progress.tag }}</span>
                  </div>
                  <p class="lp-mock__detail">{{ f.progress.detail }}</p>
                  <div class="lp-progress-bar" aria-hidden="true">
                    <span class="lp-progress-bar__fill" :style="{ width: `${f.progress.percent}%` }" />
                  </div>
                  <p class="lp-mock__banner">{{ f.progress.banner }}</p>
                </div>
              </div>
            </div>

            <div class="lp-feature__text">
              <p class="lp-eyebrow">{{ f.eyebrow }}</p>
              <h3>{{ f.title }}</h3>
              <p>{{ f.text }}</p>
              <p v-if="f.note" class="lp-feature__note">{{ f.note }}</p>
            </div>
          </article>
        </div>
      </div>
    </section>

    <!-- 06. Efficiency / Deterministic State Machine -->
    <section class="lp-section">
      <div class="lp-container">
        <header class="lp-heading">
          <p class="lp-eyebrow">{{ c.efficiency.eyebrow }}</p>
          <h2>{{ c.efficiency.title }}</h2>
          <p class="lp-lead">{{ c.efficiency.lead }}</p>
        </header>
        <div class="lp-split-cards">
          <article class="lp-card lp-split-card">
            <p class="lp-chip lp-chip--ai">{{ c.efficiency.ai.badge }}</p>
            <h3>{{ c.efficiency.ai.title }}</h3>
            <ul class="lp-hotl__list">
              <li v-for="item in c.efficiency.ai.items" :key="item">
                <LandingIcon name="arrow-right" :size="15" class="lp-ink--muted" />
                <span>{{ item }}</span>
              </li>
            </ul>
          </article>
          <article class="lp-card lp-split-card lp-split-card--afk">
            <p class="lp-chip lp-chip--both">{{ c.efficiency.afk.badge }}</p>
            <h3>{{ c.efficiency.afk.title }}</h3>
            <ul class="lp-hotl__list">
              <li v-for="item in c.efficiency.afk.items" :key="item">
                <LandingIcon name="check" :size="15" class="lp-ink--brand" />
                <span>{{ item }}</span>
              </li>
            </ul>
          </article>
        </div>
        <p class="lp-efficiency__note">{{ c.efficiency.note }}</p>
      </div>
    </section>

    <!-- 07. Architecture (Dark charcoal band with mint accents) -->
    <section class="lp-section lp-band">
      <div class="lp-container">
        <header class="lp-heading">
          <p class="lp-eyebrow lp-eyebrow--mint">{{ c.architecture.eyebrow }}</p>
          <h2>{{ c.architecture.title }}</h2>
          <p class="lp-lead lp-band__lead">{{ c.architecture.lead }}</p>
        </header>

        <figure class="lp-arch-svg-wrap">
          <svg
            class="lp-arch-svg"
            viewBox="0 0 880 280"
            role="img"
            :aria-label="c.architecture.diagram.ariaLabel"
          >
            <defs>
              <marker id="lp-arrow-mint" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
                <path d="M 0 1 L 10 5 L 0 9 z" fill="#a4e0bd" />
              </marker>
              <marker id="lp-arrow-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                <path d="M 0 1 L 10 5 L 0 9 z" fill="#8fa0b2" />
              </marker>
            </defs>

            <!-- Column 1: AI Agents -->
            <text x="115" y="28" text-anchor="middle" fill="#9fb0c2" font-size="12" font-weight="700">
              {{ c.architecture.diagram.clientsLabel }}
            </text>
            <rect x="24" y="46" width="182" height="58" rx="12" fill="rgba(255,255,255,0.05)" stroke="rgba(255,255,255,0.22)" stroke-width="1.5" />
            <text x="115" y="80" text-anchor="middle" fill="#ffffff" font-size="15" font-weight="700">
              {{ c.architecture.diagram.clients[0] }}
            </text>
            <rect x="24" y="120" width="182" height="58" rx="12" fill="rgba(255,255,255,0.05)" stroke="rgba(255,255,255,0.22)" stroke-width="1.5" />
            <text x="115" y="154" text-anchor="middle" fill="#ffffff" font-size="15" font-weight="700">
              {{ c.architecture.diagram.clients[1] }}
            </text>

            <!-- Arrows: Clients -> AFK -->
            <path d="M 206 75 L 274 96" stroke="#a4e0bd" stroke-width="2" fill="none" marker-end="url(#lp-arrow-mint)" />
            <path d="M 206 149 L 274 128" stroke="#a4e0bd" stroke-width="2" fill="none" marker-end="url(#lp-arrow-mint)" />

            <!-- Column 2: Agent Flow Kit -->
            <text x="435" y="28" text-anchor="middle" fill="#a4e0bd" font-size="12" font-weight="700">
              {{ c.architecture.diagram.kitLabel }}
            </text>
            <rect x="282" y="42" width="306" height="146" rx="16" fill="rgba(164,224,189,0.09)" stroke="rgba(164,224,189,0.55)" stroke-width="1.5" />

            <g v-for="(part, idx) in c.architecture.diagram.kit" :key="part.name">
              <rect :x="298" :y="56 + idx * 42" width="274" height="34" rx="8" fill="rgba(20,25,30,0.72)" stroke="rgba(164,224,189,0.22)" />
              <text :x="312" :y="78 + idx * 42" fill="#a4e0bd" font-size="13" font-weight="700" font-family="ui-monospace, SFMono-Regular, Menlo, monospace">
                {{ part.name }}
              </text>
              <text :x="558" :y="78 + idx * 42" text-anchor="end" fill="#dce7f0" font-size="12">
                {{ part.role }}
              </text>
            </g>

            <!-- Arrows: AFK -> Storage -->
            <path d="M 588 92 L 656 76" stroke="#a4e0bd" stroke-width="2" fill="none" marker-end="url(#lp-arrow-mint)" />
            <path d="M 588 134 L 656 150" stroke="#a4e0bd" stroke-width="2" fill="none" marker-end="url(#lp-arrow-mint)" />

            <!-- Column 3: Storage (.afk/) -->
            <text x="760" y="28" text-anchor="middle" fill="#9fb0c2" font-size="12" font-weight="700">
              {{ c.architecture.diagram.storageLabel }}
            </text>
            <rect x="664" y="46" width="192" height="58" rx="12" fill="rgba(255,255,255,0.05)" stroke="rgba(255,255,255,0.22)" stroke-width="1.5" />
            <text x="760" y="71" text-anchor="middle" fill="#ffffff" font-size="14" font-weight="700">
              {{ c.architecture.diagram.storage[0].name }}
            </text>
            <text x="760" y="91" text-anchor="middle" fill="#b4bdca" font-size="11.5">
              {{ c.architecture.diagram.storage[0].role }}
            </text>

            <rect x="664" y="120" width="192" height="58" rx="12" fill="rgba(255,255,255,0.05)" stroke="rgba(255,255,255,0.22)" stroke-width="1.5" />
            <text x="760" y="145" text-anchor="middle" fill="#ffffff" font-size="14" font-weight="700">
              {{ c.architecture.diagram.storage[1].name }}
            </text>
            <text x="760" y="165" text-anchor="middle" fill="#b4bdca" font-size="11.5">
              {{ c.architecture.diagram.storage[1].role }}
            </text>

            <!-- Dashed direct path: Agents -> Markdown -->
            <path d="M 115 178 L 115 236 L 760 236 L 760 184" stroke="#8fa0b2" stroke-width="1.5" stroke-dasharray="5 5" fill="none" marker-end="url(#lp-arrow-muted)" />
            <rect x="300" y="222" width="270" height="26" rx="13" fill="#14191e" stroke="rgba(143,160,178,0.4)" />
            <text x="435" y="239" text-anchor="middle" fill="#b4bdca" font-size="11.5">
              {{ c.architecture.diagram.directLabel }}
            </text>
          </svg>
        </figure>

        <ul class="lp-trust">
          <li v-for="item in c.architecture.items as TrustItem[]" :key="item.title">
            <LandingIcon :name="item.icon" :size="22" />
            <h3>{{ item.title }}</h3>
            <p>{{ item.text }}</p>
          </li>
        </ul>
        <p class="lp-band__note">{{ c.architecture.note }}</p>
      </div>
    </section>

    <!-- 08. Getting Started -->
    <section class="lp-section">
      <div class="lp-container">
        <header class="lp-heading">
          <p class="lp-eyebrow">{{ c.start.eyebrow }}</p>
          <h2>{{ c.start.title }}</h2>
          <p v-if="c.start.lead" class="lp-lead">{{ c.start.lead }}</p>
        </header>

        <ol class="lp-steps">
          <li v-for="(step, i) in c.start.steps" :key="step.title" class="lp-card lp-steps__card">
            <h3 class="lp-steps__title">
              <span class="lp-steps__num">{{ i + 1 }}</span>
              <span>{{ step.title }}</span>
            </h3>
            <p>{{ step.text }}</p>
          </li>
        </ol>

        <div v-if="activeInstall" class="lp-install-box">
          <div class="lp-install-box__tabs" role="tablist" :aria-label="c.start.clientSelectLabel">
            <button
              v-for="client in c.start.clients as InstallClient[]"
              :key="client.id"
              type="button"
              role="tab"
              :aria-selected="selectedClient === client.id"
              class="lp-tab"
              :class="{ 'is-active': selectedClient === client.id }"
              @click="selectedClient = client.id"
            >
              {{ client.label }}
            </button>
          </div>
          <div class="lp-command lp-command--multi">
            <pre><code><template v-for="(cmd, idx) in activeInstall.commands" :key="idx"><span class="lp-command__prompt">$</span> {{ cmd }}<template v-if="idx < activeInstall.commands.length - 1">&#10;</template></template></code></pre>
            <button
              type="button"
              class="lp-command__copy"
              :aria-label="copiedKey === activeInstall.id ? c.copy.done : c.copy.label"
              @click="copyText(activeInstall.id, activeInstall.commands.join('\n'))"
            >
              <LandingIcon :name="copiedKey === activeInstall.id ? 'check' : 'copy'" :size="16" />
              <span v-if="copiedKey === activeInstall.id" class="lp-command__done">{{ c.copy.done }}</span>
            </button>
          </div>
          <p class="lp-install-box__note">
            {{ c.start.installNote }}
            <a :href="href(c.start.guideLink.link)">{{ c.start.guideLink.text }} →</a>
          </p>
        </div>

        <div class="lp-final-cta">
          <p class="lp-final-cta__tagline">
            <template v-for="(line, i) in lines(c.start.cta.tagline)" :key="i">
              <br v-if="i" />{{ line }}
            </template>
          </p>
          <p class="lp-final-cta__sub">{{ c.start.cta.sub }}</p>
          <div class="lp-actions lp-actions--center">
            <a
              class="lp-button lp-button--brand"
              :href="href(c.start.cta.primary.link)"
              :target="isExternal(c.start.cta.primary.link) ? '_blank' : undefined"
              :rel="isExternal(c.start.cta.primary.link) ? 'noopener noreferrer' : undefined"
            >
              {{ c.start.cta.primary.text }}
            </a>
            <a
              class="lp-button"
              :href="href(c.start.cta.secondary.link)"
              :target="isExternal(c.start.cta.secondary.link) ? '_blank' : undefined"
              :rel="isExternal(c.start.cta.secondary.link) ? 'noopener noreferrer' : undefined"
            >
              {{ c.start.cta.secondary.text }}
            </a>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.landing {
  --lp-radius: 14px;
  color: var(--vp-c-text-1);
}
.landing :where(h1, h2, h3, p, ul, ol, figure, pre) {
  margin: 0;
}
.landing :where(ul, ol) {
  list-style: none;
  padding: 0;
}
.landing :where(h1, h2, h3):lang(ja) {
  word-break: auto-phrase;
}
.landing :where(h1, h2) {
  text-wrap: balance;
}
.landing p {
  text-wrap: pretty;
}
.landing a:focus-visible,
.landing button:focus-visible {
  outline: 2px solid var(--vp-c-brand-1);
  outline-offset: 2px;
}

.lp-container {
  max-width: 1280px;
  margin: 0 auto;
  padding: 0 24px;
}
@media (min-width: 640px) {
  .lp-container {
    padding: 0 48px;
  }
}
@media (min-width: 960px) {
  .lp-container {
    padding: 0 64px;
  }
}

.lp-section {
  padding: 64px 0;
}
@media (min-width: 960px) {
  .lp-section {
    padding: 96px 0;
  }
}
.lp-section--soft {
  background: var(--vp-c-bg-soft);
}

.lp-heading {
  display: grid;
  gap: 12px;
  max-width: 780px;
  margin-bottom: 40px;
}
@media (min-width: 960px) {
  .lp-heading {
    margin-bottom: 48px;
  }
}
.lp-eyebrow {
  color: var(--vp-c-brand-1);
  font-size: 13px;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.lp-eyebrow--mint {
  color: var(--lp-mint);
}
.lp-heading h2 {
  font-size: 24px;
  font-weight: 800;
  line-height: 1.38;
}
@media (min-width: 640px) {
  .lp-heading h2 {
    font-size: 32px;
    line-height: 1.32;
  }
}
.lp-lead {
  color: var(--vp-c-text-2);
  font-size: 15px;
  line-height: 1.85;
}
@media (min-width: 640px) {
  .lp-lead {
    font-size: 16px;
  }
}

/* Shared cards, buttons, chips */
.lp-card {
  display: grid;
  align-content: start;
  gap: 10px;
  padding: 20px;
  border: 1px solid var(--vp-c-divider);
  border-radius: var(--lp-radius);
  background: var(--lp-surface);
}
.lp-card--human {
  border-color: var(--lp-pending-border);
  background: var(--lp-pending-bg);
}
.lp-card h3 {
  font-size: 17px;
  font-weight: 800;
  line-height: 1.4;
}
.lp-card p {
  color: var(--vp-c-text-2);
  font-size: 14px;
  line-height: 1.75;
}
.lp-card__meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}
.lp-card__num {
  color: var(--vp-c-text-2);
  font-family: var(--vp-font-family-mono);
  font-size: 12px;
  font-weight: 700;
}

.lp-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}
.lp-actions--center {
  justify-content: center;
}
.lp-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 44px;
  padding: 0 24px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 22px;
  background: var(--vp-c-bg);
  color: var(--vp-c-text-1);
  font-size: 14px;
  font-weight: 600;
  text-decoration: none;
  transition: border-color 0.2s, background-color 0.2s;
}
.lp-button:hover {
  border-color: var(--vp-c-text-3);
}
.lp-button--brand {
  border-color: var(--vp-button-brand-border);
  background: var(--vp-button-brand-bg);
  color: var(--vp-button-brand-text);
}
.lp-button--brand:hover {
  border-color: var(--vp-button-brand-hover-border);
  background: var(--vp-button-brand-hover-bg);
  color: var(--vp-button-brand-hover-text);
}
@media (max-width: 639px) {
  .lp-hero .lp-actions .lp-button {
    flex: 1;
  }
}

.lp-chip {
  display: inline-flex;
  width: fit-content;
  padding: 2px 10px;
  border: 1px solid transparent;
  border-radius: 10px;
  font-size: 12px;
  font-weight: 700;
}
.lp-chip--human {
  border-color: var(--lp-pending-border);
  background: var(--lp-pending-bg);
  color: var(--lp-pending-text);
}
.lp-chip--ai {
  background: var(--lp-ai-bg);
  color: var(--lp-ai-text);
}
.lp-chip--both {
  background: var(--lp-mint-soft);
  color: var(--vp-c-brand-1);
}

.lp-ink--human {
  color: var(--lp-pending-text);
}
.lp-ink--brand {
  color: var(--vp-c-brand-1);
}
.lp-ink--muted {
  color: var(--vp-c-text-3);
}

/* 01. Hero */
.lp-hero {
  padding: 48px 0 56px;
  background: var(--vp-c-bg-soft);
}
.lp-hero__grid {
  display: grid;
  gap: 40px;
}
.lp-hero__copy {
  display: grid;
  align-content: start;
  gap: 20px;
  min-width: 0;
}
.lp-hero__eyebrow {
  display: inline-flex;
  width: fit-content;
  padding: 4px 12px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 999px;
  background: var(--lp-surface);
  color: var(--vp-c-brand-1);
  font-family: var(--vp-font-family-mono);
  font-size: 12px;
  font-weight: 700;
}
.lp-hero__title {
  font-size: 28px;
  font-weight: 800;
  line-height: 1.3;
  letter-spacing: -0.015em;
}
.lp-hero__lead {
  color: var(--vp-c-text-2);
  font-size: 15px;
  line-height: 1.85;
}
.lp-hero__meta {
  color: var(--vp-c-text-2);
  font-size: 13px;
}
.lp-hero__visual {
  display: grid;
  gap: 10px;
  min-width: 0;
}
.lp-demo {
  overflow: hidden;
  border: 1px solid rgba(164, 224, 189, 0.24);
  border-radius: 14px;
  background: var(--lp-band);
  color: #e6edf3;
  box-shadow: var(--lp-shadow);
}
.lp-demo__bar {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(255, 255, 255, 0.03);
}
.lp-demo__dots {
  display: inline-flex;
  gap: 6px;
}
.lp-demo__dots i {
  width: 9px;
  height: 9px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.2);
}
.lp-demo__title {
  color: #c9d4de;
  font-family: var(--vp-font-family-mono);
  font-size: 12px;
  font-weight: 600;
}
.lp-demo__tag {
  margin-left: auto;
  padding: 1px 8px;
  border: 1px solid rgba(164, 224, 189, 0.3);
  border-radius: 999px;
  color: var(--lp-mint);
  font-size: 11px;
  font-weight: 600;
}
.lp-demo__body {
  display: grid;
  gap: 10px;
  padding: 16px 18px;
  font-family: var(--vp-font-family-mono);
  font-size: 12.5px;
  line-height: 1.6;
}
.lp-demo__session {
  display: grid;
  gap: 6px;
}
.lp-demo__session-label {
  color: #8c98a6;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.lp-demo__divider {
  display: flex;
  align-items: center;
  gap: 10px;
  margin: 4px 0;
  color: #e9b04f;
  font-size: 11px;
  font-weight: 700;
}
.lp-demo__divider::before,
.lp-demo__divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: rgba(233, 176, 79, 0.35);
}
.lp-demo__prompt {
  color: #ffffff;
  font-weight: 600;
}
.lp-demo__caret {
  color: var(--lp-mint);
}
.lp-demo__reply {
  color: #b8c4d0;
}
.lp-demo__question {
  padding: 6px 10px;
  border-left: 3px solid #e9b04f;
  border-radius: 0 6px 6px 0;
  background: rgba(233, 176, 79, 0.1);
  color: #f4d28b;
  font-size: 12px;
}
.lp-demo__saved {
  padding: 6px 10px;
  border-left: 3px solid var(--lp-mint);
  border-radius: 0 6px 6px 0;
  background: rgba(164, 224, 189, 0.09);
  color: var(--lp-mint);
  font-size: 12px;
}
.lp-demo__status {
  display: grid;
  gap: 8px;
  margin-top: 4px;
  padding: 12px 14px;
  border: 1px solid rgba(164, 224, 189, 0.24);
  border-radius: 10px;
  background: rgba(164, 224, 189, 0.06);
}
.lp-demo__status-head {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}
.lp-demo__badge {
  color: var(--lp-mint);
  font-weight: 700;
}
.lp-demo__plan {
  color: #ffffff;
  font-weight: 700;
}
.lp-demo__rev {
  padding: 1px 6px;
  border-radius: 5px;
  background: rgba(164, 224, 189, 0.18);
  color: var(--lp-mint);
  font-size: 11px;
  font-weight: 700;
}
.lp-demo__pct {
  margin-left: auto;
  color: var(--lp-mint);
  font-weight: 700;
}
.lp-progress-bar {
  height: 6px;
  overflow: hidden;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.12);
}
.lp-progress-bar__fill {
  display: block;
  height: 100%;
  border-radius: 999px;
  background: var(--lp-mint);
}
.lp-demo__tasks {
  display: grid;
  gap: 4px;
  font-size: 12px;
}
.lp-demo__tasks li {
  display: flex;
  align-items: center;
  gap: 8px;
}
.lp-demo__tasks li.is-done {
  color: var(--lp-mint);
}
.lp-demo__tasks li.is-doing {
  color: #ffffff;
  font-weight: 600;
}
.lp-demo__tasks li.is-todo {
  color: #9aa7b4;
}
.lp-demo__task-text {
  min-width: 0;
}
.lp-demo__task-tag {
  margin-left: auto;
  padding: 0 6px;
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.08);
  color: #c9d4de;
  font-size: 10.5px;
  white-space: nowrap;
}
.lp-state-mark {
  width: 14px;
  text-align: center;
  font-weight: 700;
}
.lp-demo__caption {
  color: var(--vp-c-text-3);
  font-size: 12px;
  text-align: right;
}
@media (min-width: 640px) {
  .lp-hero__title {
    font-size: 36px;
  }
  .lp-hero__lead {
    font-size: 16px;
  }
}
@media (min-width: 960px) {
  .lp-hero {
    padding: 84px 0 92px;
  }
  .lp-hero__grid {
    grid-template-columns: minmax(0, 500px) minmax(0, 1fr);
    align-items: center;
    gap: 48px;
  }
  .lp-hero__copy {
    gap: 24px;
  }
  .lp-hero__title {
    font-size: 40px;
    line-height: 1.28;
  }
}

/* 02. Problem + Plan Mode comparison */
.lp-problem-grid {
  display: grid;
  gap: 16px;
}
.lp-problem-card__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 38px;
  height: 38px;
  border-radius: 10px;
  background: var(--lp-mint-soft);
  color: var(--vp-c-brand-1);
}
.lp-diff {
  margin-top: 44px;
  padding: 24px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 16px;
  background: var(--vp-c-bg-soft);
}
.lp-diff__head {
  display: grid;
  gap: 6px;
  margin-bottom: 20px;
}
.lp-diff__head h3 {
  font-size: 19px;
  font-weight: 800;
}
.lp-diff__head p {
  color: var(--vp-c-text-2);
  font-size: 14px;
  line-height: 1.7;
}
.lp-diff__table {
  display: grid;
  gap: 8px;
}
.lp-diff__row {
  display: grid;
  gap: 8px;
  padding: 14px 16px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 10px;
  background: var(--lp-surface);
  font-size: 14px;
  line-height: 1.65;
}
.lp-diff__row--head {
  display: none;
  padding: 8px 16px;
  border: 0;
  background: transparent;
  color: var(--vp-c-text-2);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.03em;
}
.lp-diff__row--head .is-afk {
  color: var(--vp-c-brand-1);
}
.lp-diff__axis {
  font-weight: 800;
  color: var(--vp-c-text-1);
}
.lp-diff__muted {
  color: var(--vp-c-text-2);
}
.lp-diff__afk {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  color: var(--vp-c-text-1);
  font-weight: 600;
}
.lp-diff__afk .lp-icon {
  margin-top: 4px;
}
@media (min-width: 768px) {
  .lp-diff__row--head {
    display: grid;
  }
  .lp-diff__row {
    grid-template-columns: 180px 1fr 1.25fr;
    align-items: start;
    gap: 20px;
  }
}
.lp-problem__closing {
  margin-top: 32px;
  padding-top: 24px;
  border-top: 1px solid var(--vp-c-divider);
  color: var(--vp-c-text-1);
  font-size: 17px;
  font-weight: 800;
  line-height: 1.6;
}
@media (min-width: 640px) {
  .lp-problem-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 20px;
  }
}

/* 03. Workflow */
.lp-workflow {
  display: grid;
  gap: 14px;
}
.lp-workflow__note {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-top: 24px;
  padding: 18px 20px;
  border: 1px solid var(--vp-c-divider);
  border-radius: var(--lp-radius);
  background: var(--lp-surface);
}
.lp-workflow__note-icon {
  margin-top: 2px;
  color: var(--vp-c-brand-1);
}
.lp-workflow__note-title {
  color: var(--vp-c-text-1);
  font-size: 15px;
  font-weight: 800;
}
.lp-workflow__note-text {
  margin-top: 4px;
  color: var(--vp-c-text-2);
  font-size: 14px;
  line-height: 1.75;
}
@media (min-width: 960px) {
  .lp-workflow {
    grid-template-columns: repeat(5, minmax(0, 1fr));
  }
}

/* 04. Human on the Loop */
.lp-hotl {
  display: grid;
  gap: 16px;
}
.lp-hotl__roles {
  display: grid;
  gap: 16px;
}
.lp-hotl__list {
  display: grid;
  gap: 10px;
  margin-top: 4px;
}
.lp-hotl__list li {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--vp-c-text-1);
  font-size: 15px;
  font-weight: 600;
}
.lp-hotl__harness {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 12px 16px;
  padding: 18px 22px;
  border: 1px solid var(--vp-c-brand-1);
  border-radius: var(--lp-radius);
  background: var(--lp-mint-soft);
}
.lp-hotl__harness-label {
  padding: 3px 10px;
  border-radius: 8px;
  background: var(--vp-c-brand-1);
  color: var(--vp-button-brand-text);
  font-family: var(--vp-font-family-mono);
  font-size: 12px;
  font-weight: 700;
}
.lp-hotl__harness p {
  color: var(--vp-c-text-1);
  font-size: 14px;
  font-weight: 600;
  line-height: 1.7;
}
.lp-hotl__note {
  color: var(--vp-c-text-2);
  font-size: 13px;
  line-height: 1.75;
}
@media (min-width: 768px) {
  .lp-hotl__roles {
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }
}

/* 05. Features (alternating) */
.lp-features {
  display: grid;
  gap: 48px;
}
.lp-feature {
  display: grid;
  gap: 24px;
  align-items: center;
}
.lp-feature__panel {
  padding: 22px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 16px;
  background: var(--lp-band);
  color: #e6edf3;
}
.lp-mock {
  display: grid;
  gap: 12px;
  font-family: var(--vp-font-family-mono);
  font-size: 12.5px;
}
.lp-mock__head {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  padding-bottom: 10px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}
.lp-mock__tag {
  padding: 2px 8px;
  border-radius: 6px;
  background: rgba(164, 224, 189, 0.16);
  color: var(--lp-mint);
  font-size: 11px;
  font-weight: 700;
}
.lp-mock__title {
  color: #ffffff;
  font-weight: 700;
}
.lp-mock__rev {
  display: grid;
  gap: 10px;
}
.lp-mock__rev-meta {
  color: #9aa7b4;
  font-size: 12px;
}
.lp-mock__rev-meta strong {
  color: var(--lp-mint);
}
.lp-mock__list {
  display: grid;
  gap: 7px;
}
.lp-mock__list li {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.03);
}
.lp-mock__list li.is-retired {
  opacity: 0.55;
  text-decoration: line-through;
}
.lp-mock__id {
  color: #8c98a6;
  font-weight: 700;
}
.lp-mock__row-text {
  min-width: 0;
  color: #e6edf3;
}
.lp-mock__pill {
  margin-left: auto;
  padding: 1px 7px;
  border-radius: 999px;
  font-size: 10.5px;
  font-weight: 700;
  white-space: nowrap;
}
.lp-mock__pill.is-done {
  background: rgba(164, 224, 189, 0.18);
  color: var(--lp-mint);
}
.lp-mock__pill.is-added {
  background: rgba(107, 154, 255, 0.2);
  color: #9dbbff;
}
.lp-mock__pill.is-retired {
  background: rgba(255, 255, 255, 0.08);
  color: #9aa7b4;
}
.lp-mock__pill.is-todo {
  background: rgba(233, 176, 79, 0.18);
  color: #f4d28b;
}

.lp-mock__gate {
  display: grid;
  gap: 10px;
}
.lp-mock__cmd {
  color: #c9d4de;
  font-size: 12px;
}
.lp-mock__checks {
  display: grid;
  gap: 6px;
}
.lp-mock__checks li {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  padding: 7px 10px;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.03);
}
.lp-mock__ok {
  color: var(--lp-mint);
  font-weight: 700;
}
.lp-mock__chk-label {
  color: #ffffff;
}
.lp-mock__chk-val {
  margin-left: auto;
  color: var(--lp-mint);
  font-size: 11.5px;
}
.lp-mock__outcome {
  padding: 8px 10px;
  border: 1px solid rgba(164, 224, 189, 0.35);
  border-radius: 8px;
  background: rgba(164, 224, 189, 0.1);
  color: var(--lp-mint);
  font-weight: 700;
}

.lp-mock__handoff {
  display: grid;
  gap: 12px;
}
.lp-mock__tree {
  overflow-x: auto;
  color: var(--lp-mint);
  font-size: 12.5px;
  line-height: 1.65;
}
.lp-mock__handoff-note {
  display: grid;
  gap: 4px;
  padding: 10px 12px;
  border-left: 3px solid #e9b04f;
  border-radius: 0 8px 8px 0;
  background: rgba(233, 176, 79, 0.08);
}
.lp-mock__handoff-note span {
  color: #f4d28b;
  font-size: 11px;
  font-weight: 700;
}
.lp-mock__handoff-note p {
  color: #e6edf3;
  font-size: 12px;
  line-height: 1.6;
}

.lp-mock__progress {
  display: grid;
  gap: 10px;
}
.lp-mock__progress-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}
.lp-mock__progress-row strong {
  color: var(--lp-mint);
  font-size: 22px;
}
.lp-mock__detail {
  color: #b8c4d0;
  font-size: 12px;
}
.lp-mock__banner {
  padding: 10px 12px;
  border-radius: 8px;
  background: rgba(164, 224, 189, 0.08);
  color: #d8e5ee;
  font-size: 12px;
  line-height: 1.6;
}

.lp-feature__text {
  display: grid;
  gap: 12px;
}
.lp-feature__text h3 {
  font-size: 22px;
  font-weight: 800;
  line-height: 1.35;
}
.lp-feature__text p {
  color: var(--vp-c-text-2);
  font-size: 15px;
  line-height: 1.85;
}
.lp-feature__note {
  font-size: 13px !important;
  color: var(--vp-c-text-3) !important;
}
@media (min-width: 960px) {
  .lp-features {
    gap: 72px;
  }
  .lp-feature {
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    gap: 56px;
  }
  .lp-feature:nth-child(even) .lp-feature__panel {
    order: 2;
  }
  .lp-feature__text h3 {
    font-size: 26px;
  }
}

/* 06. Efficiency */
.lp-split-cards {
  display: grid;
  gap: 16px;
}
.lp-split-card--afk {
  border-color: var(--vp-c-brand-1);
}
.lp-efficiency__note {
  margin-top: 20px;
  color: var(--vp-c-text-2);
  font-size: 13px;
  line-height: 1.75;
}
@media (min-width: 768px) {
  .lp-split-cards {
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }
}

/* 07. Architecture band */
.lp-band {
  background: var(--lp-band);
  color: #ffffff;
}
.lp-band h2 {
  color: #ffffff;
}
.lp-band__lead {
  color: var(--lp-band-text-2);
}
.lp-arch-svg-wrap {
  margin-bottom: 48px;
  padding: 20px;
  overflow-x: auto;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.03);
}
.lp-arch-svg {
  display: block;
  width: 100%;
  min-width: 640px;
  max-width: 880px;
  height: auto;
  margin: 0 auto;
}
.lp-arch {
  display: grid;
  gap: 16px;
  align-items: center;
  margin-bottom: 48px;
  padding: 24px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.03);
}
.lp-arch__col {
  display: grid;
  gap: 10px;
}
.lp-arch__col-label {
  color: var(--lp-band-text-2);
  font-family: var(--vp-font-family-mono);
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.06em;
  text-transform: uppercase;
}
.lp-arch__box {
  display: grid;
  gap: 2px;
  padding: 12px 14px;
  border: 1px solid rgba(255, 255, 255, 0.14);
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.04);
  color: #ffffff;
  font-size: 14px;
  font-weight: 700;
}
.lp-arch__box--store span {
  color: var(--lp-band-text-2);
  font-size: 12px;
  font-weight: 400;
}
.lp-arch__kit {
  display: grid;
  gap: 8px;
  padding: 14px;
  border: 1px solid rgba(164, 224, 189, 0.4);
  border-radius: 12px;
  background: rgba(164, 224, 189, 0.08);
}
.lp-arch__kit-item {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
  padding: 8px 10px;
  border-radius: 8px;
  background: rgba(20, 25, 30, 0.6);
}
.lp-arch__kit-item strong {
  color: var(--lp-mint);
  font-family: var(--vp-font-family-mono);
  font-size: 13px;
}
.lp-arch__kit-item span {
  color: #d6e1ea;
  font-size: 12px;
}
.lp-arch__arrow {
  justify-self: center;
  color: var(--lp-mint);
  font-size: 20px;
  font-weight: 700;
  transform: rotate(90deg);
}
@media (min-width: 960px) {
  .lp-arch {
    grid-template-columns: 1fr auto 1.4fr auto 1fr;
    gap: 20px;
  }
  .lp-arch__arrow {
    transform: none;
  }
}

.lp-trust {
  display: grid;
  gap: 24px;
}
.lp-trust li {
  display: grid;
  align-content: start;
  gap: 8px;
}
.lp-trust .lp-icon {
  color: var(--lp-mint);
}
.lp-trust h3 {
  color: #ffffff;
  font-size: 16px;
  font-weight: 800;
}
.lp-trust p {
  color: var(--lp-band-text-2);
  font-size: 14px;
  line-height: 1.75;
}
.lp-band__note {
  margin-top: 32px;
  padding-top: 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  color: var(--lp-band-text-2);
  font-size: 12px;
  line-height: 1.7;
}
@media (min-width: 640px) {
  .lp-trust {
    grid-template-columns: repeat(2, 1fr);
  }
}
@media (min-width: 960px) {
  .lp-trust {
    grid-template-columns: repeat(4, 1fr);
    gap: 32px;
  }
}

/* 08. Getting Started */
.lp-steps {
  display: grid;
  gap: 16px;
}
.lp-steps__title {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 16px;
  font-weight: 800;
}
.lp-steps__num {
  display: inline-flex;
  flex: none;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border-radius: 50%;
  background: var(--vp-c-brand-1);
  color: var(--vp-button-brand-text);
  font-size: 13px;
  font-weight: 700;
}
@media (min-width: 960px) {
  .lp-steps {
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 20px;
  }
}

.lp-install-box {
  display: grid;
  gap: 12px;
  margin-top: 28px;
  padding: 20px;
  border: 1px solid var(--vp-c-divider);
  border-radius: var(--lp-radius);
  background: var(--vp-c-bg-soft);
}
.lp-install-box__tabs {
  display: flex;
  gap: 8px;
}
.lp-tab {
  padding: 6px 14px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  background: var(--lp-surface);
  color: var(--vp-c-text-2);
  font-size: 13px;
  font-weight: 700;
  cursor: pointer;
  transition: border-color 0.2s, color 0.2s;
}
.lp-tab.is-active {
  border-color: var(--vp-c-brand-1);
  background: var(--lp-mint-soft);
  color: var(--vp-c-brand-1);
}
.lp-command {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
  min-width: 0;
  padding: 12px 12px 12px 16px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 10px;
  background: var(--lp-code-bg);
}
.lp-command pre {
  overflow-x: auto;
  min-width: 0;
  color: var(--vp-c-text-1);
  font-family: var(--vp-font-family-mono);
  font-size: 13px;
  line-height: 1.75;
}
.lp-command__prompt {
  color: var(--vp-c-brand-1);
  user-select: none;
}
.lp-command__copy {
  display: inline-flex;
  flex: none;
  align-items: center;
  gap: 6px;
  min-width: 36px;
  height: 36px;
  justify-content: center;
  padding: 0 8px;
  border-radius: 8px;
  color: var(--vp-c-text-2);
  cursor: pointer;
}
.lp-command__copy:hover {
  background: var(--vp-c-default-soft);
  color: var(--vp-c-text-1);
}
.lp-command__done {
  font-size: 12px;
  font-weight: 600;
}
.lp-install-box__note {
  color: var(--vp-c-text-2);
  font-size: 13px;
  line-height: 1.75;
}
.lp-install-box__note a {
  color: var(--vp-c-brand-1);
  font-weight: 700;
  text-decoration: none;
}
.lp-install-box__note a:hover {
  text-decoration: underline;
}

.lp-final-cta {
  display: grid;
  justify-items: center;
  gap: 14px;
  margin-top: 56px;
  padding: 40px 24px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 18px;
  background: var(--vp-c-bg-soft);
  text-align: center;
}
.lp-final-cta__tagline {
  font-size: 26px;
  font-weight: 800;
  line-height: 1.3;
  letter-spacing: -0.01em;
}
@media (min-width: 640px) {
  .lp-final-cta__tagline {
    font-size: 32px;
  }
}
.lp-final-cta__sub {
  color: var(--vp-c-text-2);
  font-size: 15px;
  margin-bottom: 6px;
}

@media (prefers-reduced-motion: reduce) {
  .landing *,
  .landing *::before,
  .landing *::after {
    transition-duration: 0.01ms !important;
    animation-duration: 0.01ms !important;
  }
}
</style>
