<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'

import { useEditorStore } from '@/stores/editor.store'

import { useExecutionStore } from '@/stores/execution.store'
import { editorLanguageFor, starterFor } from '@/constants/starters'
import { useShortcuts } from '@/composables/useShortcut'

import CodeEditor from '@/components/editor/CodeEditor.vue'
import OutputPanel from '@/components/lesson/OutputPanel.vue'
import PreviewFrame from '@/components/lesson/PreviewFrame.vue'
import SplitPane from '@/components/common/SplitPane.vue'

/**
 * Standalone scratchpad for running Rust (rustc sandbox) and Vue (sandboxed
 * webview preview) — independent of any lesson. Buffers persist per language.
 */
const route = useRoute()
const editorStore = useEditorStore()
const execution = useExecutionStore()

type Lang = 'rust' | 'vue'
const lang = computed<Lang>(() => (String(route.params.lang) === 'vue' ? 'vue' : 'rust'))
const isWeb = computed(() => lang.value === 'vue')
const scopeId = computed(() => `playground:${lang.value}`)
const previewTick = ref(0)

type EditorDoc = Awaited<ReturnType<typeof editorStore.open>>
const doc = ref<EditorDoc | null>(null)

async function ensureDoc(): Promise<void> {
  if (doc.value && doc.value.scopeId === scopeId.value) return
  doc.value = await editorStore.open(scopeId.value, editorLanguageFor(lang.value), starterFor(lang.value))
}

onMounted(ensureDoc)
watch(lang, async () => {
  execution.reset()
  await ensureDoc()
})

function run(): void {
  if (!doc.value) return
  if (isWeb.value) {
    previewTick.value++
    return
  }
  void execution.run(doc.value.code)
}

async function persist(): Promise<void> {
  await editorStore.persist(scopeId.value)
}

useShortcuts({
  'ctrl+enter': () => run(),
  'ctrl+s': () => void persist()
})
</script>

<template>
  <div class="flex h-full min-h-0 flex-col gap-3 overflow-hidden px-1">
    <header class="flex shrink-0 flex-wrap items-center gap-3">
      <h1 class="text-xl font-bold text-slate-900 dark:text-white">🧪 Playground</h1>


      <nav class="flex gap-1 rounded-lg bg-slate-200/70 p-1 dark:bg-slate-800" aria-label="Playground language">
        <router-link
          to="/playground/rust"
          class="rounded-md px-3 py-1 text-sm font-semibold"
          :class="!isWeb ? 'bg-white text-teal-700 shadow-sm dark:bg-[#101a2e] dark:text-teal-300' : 'text-slate-500 dark:text-slate-400'"
          exact-active-class="!bg-white dark:!bg-[#101a2e]"
        >🦀 Rust</router-link>
        <router-link
          to="/playground/vue"
          class="rounded-md px-3 py-1 text-sm font-semibold"
          :class="isWeb ? 'bg-white text-teal-700 shadow-sm dark:bg-[#101a2e] dark:text-teal-300' : 'text-slate-500 dark:text-slate-400'"
        >💚 Vue</router-link>
      </nav>

      <span class="badge bg-slate-200 text-slate-500 dark:bg-slate-800 dark:text-slate-400">
        {{ isWeb ? 'webview preview' : 'rustc sandbox · 5s timeout' }}
      </span>
      <span v-if="doc?.dirty" class="text-xs italic text-slate-400">unsaved · Ctrl+S</span>

      <div class="ml-auto flex items-center gap-2">
        <button v-if="!isWeb && execution.isBusy" class="btn-danger" @click="execution.cancel()">Stop</button>
        <button v-else class="btn-primary" data-testid="playground-run" @click="run">
          {{ isWeb ? 'Run preview ▶' : 'Run ▶ Ctrl+Enter' }}
        </button>
      </div>
    </header>

    <SplitPane direction="vertical" :initial="[62, 38]" storage-key="playground-main" class="min-h-0 flex-1" collapsible>
      <template #first>
        <div class="h-full min-h-0 pb-1">
          <CodeEditor v-if="doc" :doc="doc" @run="run" />
        </div>
      </template>
      <template #second>
        <OutputPanel v-if="!isWeb" />
        <PreviewFrame v-else :code="doc?.code ?? ''" :tick="previewTick" height="160px" />
      </template>
    </SplitPane>
  </div>
</template>
