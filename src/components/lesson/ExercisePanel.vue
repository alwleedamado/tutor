<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { Exercise } from '@/types/domain'
import type { EditorDocument } from '@/stores/editor.store'
import { useEditorStore } from '@/stores/editor.store'
import { useProgressStore } from '@/stores/progress.store'
import { useExecutionStore } from '@/stores/execution.store'
import CodeEditor from '@/components/editor/CodeEditor.vue'
import PreviewFrame from '@/components/lesson/PreviewFrame.vue'
import { trackUsesWebPreview } from '@/constants/starters'


const props = defineProps<{
  exercise: Exercise
  trackId: string
  moduleId: string
  lessonId: string
}>()

const emit = defineEmits<{
  (e: 'run', code: string): void
  (e: 'solved'): void
}>()


const editorStore = useEditorStore()
const progress = useProgressStore()
const execution = useExecutionStore()

const scopeId = `exercise:${props.trackId}:${props.moduleId}:${props.lessonId}:${props.exercise.id}`
const doc = ref<EditorDocument | null>(null)

const revealedHints = ref(0)
const solutionVisible = ref(false)
const attempted = ref(false)
const solved = ref(false)

const kindLabel = computed(() =>
  props.exercise.kind === 'bug-hunt' ? '🐞 Bug hunt' : '🛠️ Implementation'
)

/** Web-track exercises preview in the sandboxed iframe — never rustc. */
const isWeb = computed(() => trackUsesWebPreview(props.trackId))
const webTick = ref(0)

function onRun(): void {
  if (!doc.value) return
  if (isWeb.value) {
    webTick.value++
    return
  }
  emit('run', doc.value.code)
}


onMounted(async () => {
  doc.value = await editorStore.open(scopeId, props.exercise.language, props.exercise.starterCode)
})

async function markAttempted(status: 'tried' | 'solved') {
  attempted.value = attempted.value || status === 'tried'
  solved.value = solved.value || status === 'solved'
  await progress.recordExercise(
    props.trackId,
    props.moduleId,
    props.lessonId,
    props.exercise.id,
    status,
    solutionVisible.value
  )
  if (status === 'solved') emit('solved')
}


async function revealSolution() {
  if (!attempted.value && revealedHints.value === 0) return // attempt first — pedagogy gate
  solutionVisible.value = true
  await markAttempted(attempted.value ? 'solved' : 'tried')
}

function resetStarter() {
  if (doc.value) doc.value.code = props.exercise.starterCode
}

</script>

<template>
  <div class="space-y-3">
    <div class="flex items-center gap-2">
      <span class="badge bg-teal-600/10 text-teal-700 dark:text-teal-300">{{ kindLabel }}</span>
      <span v-if="solved" class="badge bg-emerald-600/10 text-emerald-700 dark:text-emerald-300">Solved</span>
      <span v-if="isWeb" class="badge bg-slate-200 text-slate-500 dark:bg-slate-800 dark:text-slate-400">
        webview preview
      </span>
      <button v-if="!isWeb && execution.isBusy" class="btn-danger ml-auto !py-0.5" @click="execution.cancel()">Stop</button>
      <button v-else class="btn-primary ml-auto !py-0.5" data-testid="exercise-run" @click="onRun">
        {{ isWeb ? 'Run preview ▶' : 'Run ▶' }}
      </button>
    </div>

    <p class="text-sm leading-relaxed text-slate-700 dark:text-slate-300">{{ exercise.prompt }}</p>

    <div v-if="doc" :class="isWeb ? 'grid gap-3 lg:grid-cols-2' : ''">
      <CodeEditor :doc="doc" height="240px" @run="onRun" />
      <PreviewFrame v-if="isWeb" :code="doc.code" :tick="webTick" height="240px" />
    </div>
    <p v-else class="animate-pulse text-xs italic text-slate-400">Loading workspace…</p>


    <div class="card p-3 text-sm">
      <div class="flex flex-wrap gap-2">
        <button
          v-if="revealedHints < exercise.hints.length"
          class="btn-ghost"
          data-testid="reveal-hint"
          @click="revealedHints++"
        >
          💡 Hint {{ revealedHints + 1 }}/{{ exercise.hints.length }}
        </button>
        <button class="btn-ghost" @click="markAttempted('tried')">Mark as attempted</button>
        <button
          v-if="!solved"
          class="btn-ghost !text-emerald-700 dark:!text-emerald-300"
          data-testid="exercise-solved"
          title="Record this exercise as solved"
          @click="markAttempted('solved')"
        >
          ✓ Mark solved
        </button>
        <button
          class="btn-ghost"
          data-testid="reveal-solution"
          :disabled="!attempted && revealedHints === 0"
          title="Attempt it or take a hint first"
          @click="revealSolution"
        >
          👁 Show solution
        </button>

        <button
          v-if="doc && doc.code !== exercise.starterCode"
          class="btn-ghost"
          @click="resetStarter"
        >
          ↺ Reset starter
        </button>
      </div>
      <ol class="mt-2 space-y-1">
        <li
          v-for="(hint, i) in exercise.hints.slice(0, revealedHints)"
          :key="i"
          class="rounded bg-amber-500/10 px-2 py-1 text-xs text-amber-800 dark:text-amber-200"
        >
          Hint {{ i + 1 }}: {{ hint }}
        </li>
      </ol>
    </div>

    <details v-if="solutionVisible" class="rounded-lg border border-emerald-600/30 p-3" open>
      <summary class="cursor-pointer text-sm font-semibold text-emerald-700 dark:text-emerald-300">
        Reference solution
      </summary>
      <pre class="mt-2 overflow-x-auto rounded bg-slate-950 p-3 font-mono text-xs leading-relaxed text-slate-100">{{ exercise.solution }}</pre>
      <p v-if="exercise.kind === 'bug-hunt'" class="mt-2 text-xs italic text-slate-500">
        Compare against your diagnosis of the flawed snippet — the reasoning matters more than the diff.
      </p>
    </details>
  </div>
</template>

