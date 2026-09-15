<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import type { Lesson as LessonDto } from '@/types/domain'
import { useLessonsStore } from '@/stores/lessons.store'
import { useProgressStore } from '@/stores/progress.store'
import { useEditorStore } from '@/stores/editor.store'
import { useExecutionStore } from '@/stores/execution.store'
import { useLibraryStore } from '@/stores/library.store'
import { renderMarkdown } from '@/composables/useMarkdown'
import { useShortcuts } from '@/composables/useShortcut'

import LessonOutline from '@/components/lesson/LessonOutline.vue'
import QuizPanel from '@/components/lesson/QuizPanel.vue'
import ExercisePanel from '@/components/lesson/ExercisePanel.vue'
import NotesPanel from '@/components/lesson/NotesPanel.vue'
import OutputPanel from '@/components/lesson/OutputPanel.vue'
import PreviewFrame from '@/components/lesson/PreviewFrame.vue'
import CodeEditor from '@/components/editor/CodeEditor.vue'
import SplitPane from '@/components/common/SplitPane.vue'
import { editorLanguageFor, starterFor, trackUsesWebPreview } from '@/constants/starters'


const route = useRoute()
const router = useRouter()
const lessons = useLessonsStore()
const progress = useProgressStore()
const editorStore = useEditorStore()
const execution = useExecutionStore()
const library = useLibraryStore()

const trackId = computed(() => String(route.meta.track ?? 'rust'))
const moduleParam = computed(() => String(route.params.module ?? ''))
const lessonParam = computed(() => String(route.params.lesson ?? ''))

const moduleDef = computed(() => lessons.findModule(trackId.value, moduleParam.value))
const lesson = computed<LessonDto | undefined>(() =>
  lessons.findLesson(trackId.value, moduleParam.value, lessonParam.value)
)

const key = computed(() => `${trackId.value}:${moduleDef.value?.id}:${lesson.value?.id}`)
const scopeId = computed(() => `lesson:${key.value}`)
const docLanguage = computed(() => editorLanguageFor(trackId.value))

/**
 * Web-track lessons preview in the sandboxed webview; compiled-language
 * tracks (rust, tauri, backend-rust) run through the rustc sandbox.
 */
const isWebTrack = computed(() => trackUsesWebPreview(trackId.value))
const previewTick = ref(0)


type EditorDoc = Awaited<ReturnType<typeof editorStore.open>>
const doc = ref<EditorDoc | null>(null)

const bottomTab = ref<'quiz' | 'exercise' | 'notes'>('quiz')
const activeExample = ref<string | null>(null)

async function openWorkspace(target: LessonDto) {
  const starter = target.examples[0]?.code ?? starterFor(trackId.value)
  doc.value = await editorStore.open(scopeId.value, docLanguage.value, starter)
  activeExample.value = target.examples[0]?.name ?? null
}


onMounted(async () => {
  await library.ensureBookmarks()
  if (lesson.value) {
    bottomTab.value = lesson.value.quiz ? 'quiz' : lesson.value.exercise ? 'exercise' : 'notes'
    await openWorkspace(lesson.value)
  }
})

watch(lesson, async (target) => {
  if (!target) return
  execution.reset()
  bottomTab.value = target.quiz ? 'quiz' : target.exercise ? 'exercise' : 'notes'
  if (!editorStore.documents[scopeId.value]) await openWorkspace(target)
})
function loadExample(name: string) {
  const example = lesson.value?.examples.find((e) => e.name === name)
  if (example && doc.value) {
    doc.value.code = example.code
    activeExample.value = name
    if (isWebTrack.value) previewTick.value++
  }
}

function runCode(code?: string) {
  const source = code ?? doc.value?.code
  if (!source) return
  if (isWebTrack.value) {
    previewTick.value++ // webview preview refresh â€” never rustc
    return
  }
  void execution.run(source)
}

async function persistSnapshot() {
  await editorStore.persist(scopeId.value)
}

/** Learning flow: finishing the exercise advances to the quiz (when present). */
function onExerciseSolved(): void {
  if (lesson.value?.quiz) bottomTab.value = 'quiz'
}


useShortcuts({
  'ctrl+enter': () => runCode(),
  'ctrl+s': () => void persistSnapshot()
})

const html = computed(() => (lesson.value ? renderMarkdown(lesson.value.markdown) : ''))
const isBookmarked = computed(() => library.isBookmarked('lesson', key.value))

const indexInModule = computed(() =>
  moduleDef.value ? moduleDef.value.lessons.findIndex((l) => l.id === lesson.value?.id) : -1
)
const prevLesson = computed(() =>
  indexInModule.value > 0 ? moduleDef.value?.lessons[indexInModule.value - 1] : undefined
)
const nextLesson = computed(() =>
  moduleDef.value && indexInModule.value < moduleDef.value.lessons.length - 1
    ? moduleDef.value.lessons[indexInModule.value + 1]
    : undefined
)
const isCompleted = computed(() =>
  lesson.value && moduleDef.value
    ? progress.isLessonCompleted(trackId.value, moduleDef.value.id, lesson.value.id)
    : false
)

function jump(target: LessonDto) {
  router.push(`/${trackId.value}/${moduleDef.value?.slug}/${target.slug}`)
}
</script>

<template>
  <div v-if="lesson && moduleDef" class="flex h-full min-h-0 flex-col gap-3 overflow-hidden px-1">

    <header class="flex shrink-0 flex-wrap items-center gap-3">
      <nav class="text-sm text-slate-500 dark:text-slate-400" aria-label="Breadcrumb">
        <router-link to="/" class="hover:underline">Home</router-link>
        <span aria-hidden="true"> / </span>
        <span class="font-medium text-slate-700 dark:text-slate-300">{{ moduleDef.title }}</span>
      </nav>
      <h1 class="text-xl font-bold text-slate-900 dark:text-white">{{ lesson.title }}</h1>
      <span class="badge bg-slate-200 text-slate-600 dark:bg-slate-800 dark:text-slate-300">~{{ lesson.minutes }} min</span>

      <div class="ml-auto flex items-center gap-2">
        <button
          class="btn-ghost"
          :title="isBookmarked ? 'Remove bookmark' : 'Bookmark this lesson'"
          @click="library.toggleBookmark('lesson', key)"
        >
          {{ isBookmarked ? '🔖' : '📑' }}
        </button>
        <button
          class="btn-primary"
          data-testid="toggle-complete"
          @click="progress.toggleLesson(trackId, moduleDef.id, lesson.id)"
        >
          {{ isCompleted ? '✓ Completed' : 'Mark complete' }}
        </button>
      </div>
    </header>

    <SplitPane direction="vertical" :initial="[66, 34]" storage-key="lesson-main" class="min-h-0 flex-1" collapsible>

      <template #first>
        <SplitPane direction="horizontal" :initial="[18, 82]" storage-key="lesson-mid" class="h-full" collapsible>

          <template #first>
            <div class="h-full overflow-y-auto pr-1 pt-1">
              <LessonOutline :module="moduleDef" :track-id="trackId" :lesson="lesson" />
            </div>
          </template>
          <template #second>
            <SplitPane direction="horizontal" :initial="[52, 48]" storage-key="lesson-work" class="h-full" collapsible>

              <template #first>
                <article class="card h-full overflow-y-auto p-4" aria-label="Lesson content">
                  <div class="prose-lesson max-w-none" v-html="html" />

                  <section v-if="lesson.objectives.length" class="mt-5">
                    <h2 class="mb-2 text-sm font-bold uppercase tracking-wide text-teal-600 dark:text-teal-300">Objectives</h2>
                    <ul class="list-disc space-y-1 pl-5 text-sm">
                      <li v-for="objective in lesson.objectives" :key="objective">{{ objective }}</li>
                    </ul>
                  </section>

                  <section
                    v-for="smell in lesson.smells"
                    :key="smell.name"
                    class="mt-5 rounded-lg border border-slate-200 p-3 dark:border-slate-800"
                    data-testid="code-smell"
                  >
                    <h3 class="text-sm font-bold text-red-700 dark:text-red-300">🦨 {{ smell.name }}</h3>
                    <p class="mt-1 text-xs"><strong>Problem.</strong> {{ smell.problem }}</p>
                    <p class="text-xs"><strong>Why it hurts.</strong> {{ smell.why }}</p>
                    <pre class="mt-1 overflow-x-auto rounded bg-slate-950 p-2 font-mono text-[11px] text-slate-100">{{ smell.badCode }}</pre>
                    <p class="mt-1 text-xs"><strong>Refactor.</strong> {{ smell.refactor }}</p>
                    <pre class="mt-1 overflow-x-auto rounded bg-emerald-950 p-2 font-mono text-[11px] text-emerald-100 ring-1 ring-emerald-800">{{ smell.goodCode }}</pre>
                  </section>
                </article>
              </template>

              <template #second>
                <div class="flex h-full min-h-0 flex-col gap-2 pt-1">
                  <div v-if="lesson.examples.length" class="flex shrink-0 flex-wrap gap-1.5">
                    <button
                      v-for="example in lesson.examples"
                      :key="example.name"
                      class="btn-ghost !px-2 !py-0.5 text-xs"
                      :class="{ '!bg-teal-600/10 !text-teal-700 dark:!text-teal-300': activeExample === example.name }"
                      @click="loadExample(example.name)"
                    >
                      📄 {{ example.name }}
                    </button>
                  </div>

                  <div class="flex shrink-0 items-center gap-2">
                    <span v-if="doc?.dirty" class="text-xs italic text-slate-400">unsaved · Ctrl+S</span>
                    <span class="badge bg-slate-200 text-slate-500 dark:bg-slate-800 dark:text-slate-400">
                      {{ isWebTrack ? 'webview preview' : 'rustc sandbox' }}
                    </span>
                    <button
                      v-if="!isWebTrack && execution.isBusy"
                      class="btn-danger ml-auto !py-1"
                      @click="execution.cancel()"
                    >Stop</button>
                    <button
                      v-else
                      class="btn-primary ml-auto !py-1"
                      data-testid="run-code"
                      @click="runCode()"
                    >{{ isWebTrack ? 'Run preview ▶' : 'Run ▶ Ctrl+Enter' }}</button>
                  </div>

                  <SplitPane direction="vertical" :initial="[58, 42]" storage-key="lesson-code" class="min-h-0 flex-1" collapsible>

                    <template #first>
                      <div class="h-full min-h-0 pb-1">
                        <CodeEditor v-if="doc" :doc="doc" @run="runCode()" />
                      </div>
                    </template>
                    <template #second>
                      <OutputPanel v-if="!isWebTrack" />
                      <PreviewFrame v-else :code="doc?.code ?? ''" :tick="previewTick" height="120px" />
                    </template>
                  </SplitPane>
                </div>
              </template>
            </SplitPane>
          </template>
        </SplitPane>
      </template>
      <template #second>
        <section class="flex h-full min-h-0 flex-col rounded-xl border border-slate-200 bg-white p-3 dark:border-slate-800 dark:bg-[#101a2e]">
          <div role="tablist" class="flex shrink-0 gap-1">
            <button
              v-for="tabName in (['quiz', 'exercise', 'notes'] as const)"
              :key="tabName"
              role="tab"
              :aria-selected="bottomTab === tabName"
              class="rounded-md px-3 py-1 text-sm font-semibold capitalize"
              :class="bottomTab === tabName ? 'bg-teal-600 text-white' : 'text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800'"
              @click="bottomTab = tabName"
            >
              {{ tabName }}
            </button>
          </div>

          <div class="min-h-0 flex-1 overflow-y-auto p-2">
            <QuizPanel
              v-if="bottomTab === 'quiz' && lesson.quiz"
              :quiz="lesson.quiz"
              :track-id="trackId"
              :module-id="moduleDef.id"
              :lesson-id="lesson.id"
            />
            <p v-else-if="bottomTab === 'quiz'" class="text-sm italic text-slate-400">
              No quiz here — jump into the exercise instead.
            </p>

            <ExercisePanel
              v-else-if="bottomTab === 'exercise' && lesson.exercise"
              :exercise="lesson.exercise"
              :track-id="trackId"
              :module-id="moduleDef.id"
              :lesson-id="lesson.id"
              @run="runCode($event)"
              @solved="onExerciseSolved"
            />

            <p v-else-if="bottomTab === 'exercise'" class="text-sm italic text-slate-400">No exercise for this lesson.</p>

            <NotesPanel v-else scope-type="lesson" :scope-id="key" />
          </div>
        </section>
      </template>
    </SplitPane>

    <footer class="flex shrink-0 items-center justify-between">
      <button v-if="prevLesson" class="btn-ghost" @click="jump(prevLesson)">← {{ prevLesson.title }}</button>
      <span v-else />
      <button v-if="nextLesson" class="btn-primary" @click="jump(nextLesson)">{{ nextLesson.title }} →</button>
    </footer>
  </div>

  <p v-else class="italic text-slate-400">Loading lesson…</p>
</template>




