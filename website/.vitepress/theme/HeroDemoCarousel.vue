<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'

type DemoTurn = {
  kind: 'prompt' | 'question' | 'saved' | 'reply'
  text: string
}
type DemoTask = {
  state: 'done' | 'doing' | 'todo'
  text: string
  tag?: string
}
type DemoScene = {
  tab: string
  sessionLabel: string
  turns: DemoTurn[]
  status: {
    title: string
    revision?: string
    progressText: string
    percent: number
    tasks: DemoTask[]
  }
}
type HeroDemo = {
  header: string
  ariaLabel: string
  pauseLabel?: string
  playLabel?: string
  scenes: DemoScene[]
}

const props = defineProps<{ demo: HeroDemo }>()

const activeIdx = ref(0)
const isPaused = ref(false)
const reducedMotion = ref(false)

// During SSR (before mount) or when reducedMotion is active, render the full current scene statically.
const hydrated = ref(false)
const stepIndex = ref(99)
const typedCount = ref(999)
const showStatus = ref(true)
const progressWidth = ref(0)

const currentScene = computed(() => props.demo.scenes[activeIdx.value] ?? props.demo.scenes[0])

let timer: ReturnType<typeof setTimeout> | null = null

function clearTimer() {
  if (timer !== null) {
    clearTimeout(timer)
    timer = null
  }
}

function showFullScene() {
  clearTimer()
  const scene = currentScene.value
  stepIndex.value = scene.turns.length
  typedCount.value = 999
  showStatus.value = true
  progressWidth.value = scene.status.percent
}

function scheduleNext(delay: number, fn: () => void) {
  clearTimer()
  timer = setTimeout(fn, delay)
}

function runStep() {
  if (isPaused.value || reducedMotion.value) return
  const scene = currentScene.value
  const turns = scene.turns

  if (stepIndex.value < turns.length) {
    const turn = turns[stepIndex.value]
    if (turn.kind === 'prompt') {
      if (typedCount.value < turn.text.length) {
        typedCount.value += 1
        scheduleNext(34, runStep)
      } else {
        scheduleNext(380, () => {
          stepIndex.value += 1
          typedCount.value = 0
          runStep()
        })
      }
    } else {
      scheduleNext(620, () => {
        stepIndex.value += 1
        typedCount.value = 0
        runStep()
      })
    }
    return
  }

  if (!showStatus.value) {
    showStatus.value = true
    scheduleNext(60, () => {
      progressWidth.value = scene.status.percent
      scheduleNext(2600, () => {
        if (isPaused.value || reducedMotion.value) return
        activeIdx.value = (activeIdx.value + 1) % props.demo.scenes.length
        startScene()
      })
    })
  }
}

function startScene() {
  clearTimer()
  if (reducedMotion.value) {
    showFullScene()
    return
  }
  stepIndex.value = 0
  typedCount.value = 0
  showStatus.value = false
  progressWidth.value = 0
  scheduleNext(240, runStep)
}

function selectScene(idx: number) {
  activeIdx.value = idx
  if (reducedMotion.value || isPaused.value) {
    showFullScene()
  } else {
    startScene()
  }
}

function togglePause() {
  isPaused.value = !isPaused.value
  if (isPaused.value) {
    showFullScene()
  } else if (!reducedMotion.value) {
    startScene()
  }
}

function visibleText(turn: DemoTurn, idx: number): string {
  if (!hydrated.value || reducedMotion.value || isPaused.value || idx < stepIndex.value) {
    return turn.text
  }
  if (idx === stepIndex.value && turn.kind === 'prompt') {
    return turn.text.slice(0, typedCount.value)
  }
  return turn.text
}

function isTurnVisible(idx: number): boolean {
  if (!hydrated.value || reducedMotion.value || isPaused.value) return true
  return idx <= stepIndex.value
}

function isTypingCursor(turn: DemoTurn, idx: number): boolean {
  if (!hydrated.value || reducedMotion.value || isPaused.value) return false
  return idx === stepIndex.value && turn.kind === 'prompt' && typedCount.value < turn.text.length
}

onMounted(() => {
  hydrated.value = true
  reducedMotion.value = window.matchMedia('(prefers-reduced-motion: reduce)').matches
  if (reducedMotion.value) {
    showFullScene()
  } else {
    startScene()
  }
})

onBeforeUnmount(() => {
  clearTimer()
})
</script>

<template>
  <div class="lp-demo" role="region" :aria-label="demo.ariaLabel">
    <div class="lp-demo__bar">
      <span class="lp-demo__dots" aria-hidden="true"><i /><i /><i /></span>
      <span class="lp-demo__title">{{ demo.header }}</span>
      <button
        type="button"
        class="lp-demo__toggle"
        :aria-label="isPaused ? (demo.playLabel ?? 'Play') : (demo.pauseLabel ?? 'Pause')"
        :aria-pressed="isPaused"
        @click="togglePause"
      >
        {{ isPaused ? '▶' : '⏸' }}
      </button>
    </div>

    <div class="lp-demo__tabs" role="tablist">
      <button
        v-for="(scene, idx) in demo.scenes"
        :key="scene.tab"
        type="button"
        role="tab"
        :aria-selected="activeIdx === idx"
        class="lp-demo__tab"
        :class="{ 'is-active': activeIdx === idx }"
        @click="selectScene(idx)"
      >
        {{ scene.tab }}
      </button>
    </div>

    <div class="lp-demo__viewport">
      <Transition name="lp-slide" mode="out-in">
        <div :key="activeIdx" class="lp-demo__body">
          <p class="lp-demo__session-label">{{ currentScene.sessionLabel }}</p>

          <div class="lp-demo__turns">
            <template v-for="(turn, idx) in currentScene.turns" :key="idx">
              <div v-if="isTurnVisible(idx)" class="lp-demo__turn">
                <p v-if="turn.kind === 'prompt'" class="lp-demo__prompt">
                  <span class="lp-demo__caret" aria-hidden="true">❯</span>
                  <span>{{ visibleText(turn, idx) }}</span>
                  <span v-if="isTypingCursor(turn, idx)" class="lp-demo__cursor" aria-hidden="true" />
                </p>
                <p v-else-if="turn.kind === 'question'" class="lp-demo__question">
                  {{ turn.text }}
                </p>
                <p v-else-if="turn.kind === 'saved'" class="lp-demo__saved">
                  {{ turn.text }}
                </p>
                <p v-else class="lp-demo__reply">
                  {{ turn.text }}
                </p>
              </div>
            </template>
          </div>

          <div class="lp-demo__status" :class="{ 'is-visible': showStatus }">
            <div class="lp-demo__status-head">
              <span class="lp-demo__badge">[AFK]</span>
              <span class="lp-demo__plan">{{ currentScene.status.title }}</span>
              <span v-if="currentScene.status.revision" class="lp-demo__rev">{{ currentScene.status.revision }}</span>
              <span class="lp-demo__pct">{{ currentScene.status.progressText }}</span>
            </div>
            <div class="lp-progress-bar" aria-hidden="true">
              <span
                class="lp-progress-bar__fill"
                :style="{ width: `${hydrated ? progressWidth : currentScene.status.percent}%` }"
              />
            </div>
            <ul class="lp-demo__tasks">
              <li
                v-for="task in currentScene.status.tasks"
                :key="task.text"
                :class="`is-${task.state}`"
              >
                <span class="lp-state-mark" aria-hidden="true">
                  {{ task.state === 'done' ? '✓' : task.state === 'doing' ? '●' : '○' }}
                </span>
                <span class="lp-demo__task-text">{{ task.text }}</span>
                <span v-if="task.tag" class="lp-demo__task-tag">{{ task.tag }}</span>
              </li>
            </ul>
          </div>
        </div>
      </Transition>
    </div>

    <div class="lp-demo__dots-nav" aria-hidden="true">
      <span
        v-for="(_, idx) in demo.scenes"
        :key="idx"
        class="lp-demo__dot"
        :class="{ 'is-active': activeIdx === idx }"
      />
    </div>
  </div>
</template>

<style scoped>
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
.lp-demo__toggle {
  margin-left: auto;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border: 1px solid rgba(255, 255, 255, 0.14);
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.04);
  color: #c9d4de;
  font-size: 11px;
  cursor: pointer;
}
.lp-demo__toggle:hover {
  border-color: rgba(164, 224, 189, 0.5);
  color: var(--lp-mint);
}
.lp-demo__toggle:focus-visible,
.lp-demo__tab:focus-visible {
  outline: 2px solid var(--lp-mint);
  outline-offset: 2px;
}

.lp-demo__tabs {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 6px;
  padding: 10px 14px 0;
}
.lp-demo__tab {
  padding: 6px 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.02);
  color: #8c98a6;
  font-size: 11.5px;
  font-weight: 700;
  text-align: center;
  cursor: pointer;
  transition: border-color 0.2s, color 0.2s, background-color 0.2s;
}
.lp-demo__tab:hover {
  color: #e6edf3;
}
.lp-demo__tab.is-active {
  border-color: rgba(164, 224, 189, 0.45);
  background: rgba(164, 224, 189, 0.12);
  color: var(--lp-mint);
}

.lp-demo__viewport {
  min-height: 360px;
  padding: 16px 18px 12px;
  overflow: hidden;
}
.lp-demo__body {
  display: flex;
  flex-direction: column;
  gap: 10px;
  min-height: 332px;
  font-family: var(--vp-font-family-mono);
  font-size: 12.5px;
  line-height: 1.6;
}
.lp-demo__session-label {
  margin: 0;
  color: #8c98a6;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.04em;
}
.lp-demo__turns {
  display: grid;
  gap: 8px;
}
.lp-demo__turn {
  animation: lp-fade-in 0.22s ease-out;
}
.lp-demo__turn p {
  margin: 0;
}
.lp-demo__prompt {
  color: #ffffff;
  font-weight: 600;
}
.lp-demo__caret {
  margin-right: 6px;
  color: var(--lp-mint);
}
.lp-demo__cursor {
  display: inline-block;
  width: 7px;
  height: 13px;
  margin-left: 2px;
  vertical-align: -2px;
  background: var(--lp-mint);
  animation: lp-blink 0.8s steps(2, start) infinite;
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
  margin-top: auto;
  padding: 12px 14px;
  border: 1px solid rgba(164, 224, 189, 0.24);
  border-radius: 10px;
  background: rgba(164, 224, 189, 0.06);
  opacity: 0;
  transform: translateY(6px);
  transition: opacity 0.28s ease, transform 0.28s ease;
}
.lp-demo__status.is-visible {
  opacity: 1;
  transform: translateY(0);
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
  transition: width 0.6s ease;
}
.lp-demo__tasks {
  display: grid;
  gap: 4px;
  margin: 0;
  padding: 0;
  list-style: none;
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
.lp-state-mark {
  width: 14px;
  text-align: center;
  font-weight: 700;
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

.lp-demo__dots-nav {
  display: flex;
  justify-content: center;
  gap: 6px;
  padding: 0 0 12px;
}
.lp-demo__dot {
  width: 6px;
  height: 6px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.18);
  transition: width 0.25s ease, background-color 0.25s ease;
}
.lp-demo__dot.is-active {
  width: 20px;
  background: var(--lp-mint);
}

/* Carousel slide transition */
.lp-slide-enter-active,
.lp-slide-leave-active {
  transition: opacity 0.26s ease, transform 0.26s ease;
}
.lp-slide-enter-from {
  opacity: 0;
  transform: translateX(18px);
}
.lp-slide-leave-to {
  opacity: 0;
  transform: translateX(-18px);
}

@keyframes lp-fade-in {
  from {
    opacity: 0;
    transform: translateY(3px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
@keyframes lp-blink {
  50% {
    opacity: 0;
  }
}

@media (prefers-reduced-motion: reduce) {
  .lp-demo__turn,
  .lp-demo__cursor,
  .lp-demo__status,
  .lp-progress-bar__fill,
  .lp-slide-enter-active,
  .lp-slide-leave-active {
    animation: none !important;
    transition: none !important;
  }
}
</style>
