<script setup lang="ts">
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
import type * as MonacoNamespace from 'monaco-editor'
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'

import type { EditorDocument } from '@/stores/editor.store'
import { useEditorStore } from '@/stores/editor.store'
import { useSettingsStore } from '@/stores/settings.store'

type Monaco = typeof MonacoNamespace

let loadPromise: Promise<Monaco> | null = null

function loadMonaco(): Promise<Monaco> {
  if (!loadPromise) {
    // Bundled worker → fully offline. Rust colorization is built in.
    ;(self as unknown as { MonacoEnvironment?: unknown }).MonacoEnvironment = {
      getWorker: () => new EditorWorker()
    }
    loadPromise = import('monaco-editor').then((monaco) => {
      monaco.editor.defineTheme('rm-dark', {
        base: 'vs-dark',
        inherit: true,
        rules: [],
        colors: { 'editor.background': '#0b1220', 'editorGutter.background': '#0b1220' }
      })
      monaco.editor.defineTheme('rm-light', {
        base: 'vs',
        inherit: true,
        rules: [],
        colors: { 'editor.background': '#ffffff' }
      })
      return monaco
    })
  }
  return loadPromise
}

const props = defineProps<{
  doc: EditorDocument
  /** Fixed pixel height; omit to fill the parent pane (resizable layouts). */
  height?: string
  readOnly?: boolean
}>()

const emit = defineEmits<{ (e: 'run'): void }>()

const editorStore = useEditorStore()
const settings = useSettingsStore()
const container = ref<HTMLDivElement | null>(null)

let editor: MonacoNamespace.editor.IStandaloneCodeEditor | null = null

function currentTheme(): string {
  return document.documentElement.classList.contains('dark') ? 'rm-dark' : 'rm-light'
}

onMounted(async () => {
  if (!container.value) return
  const monaco = await loadMonaco()
  editor = monaco.editor.create(container.value, {
    value: props.doc.code,
    language: props.doc.language,
    theme: currentTheme(),
    readOnly: props.readOnly ?? false,
    fontSize: settings.settings.fontSize + 1,
    tabSize: settings.settings.tabSize,
    fontLigatures: settings.settings.fontLigatures,
    minimap: { enabled: false },
    automaticLayout: true,
    scrollBeyondLastLine: false,
    renderLineHighlight: 'line',
    padding: { top: 8 }
  })

  editor.onDidChangeModelContent(() => {
    const value = editor?.getValue() ?? ''
    editorStore.setCode(props.doc.scopeId, value)
  })
  editor.onDidChangeCursorPosition((event) => {
    const model = editor?.getModel()
    const offset = model ? model.getOffsetAt(event.position) : 0
    editorStore.setCursor(props.doc.scopeId, offset)
  })



  // Ctrl+Enter runs from inside the editor too.
  editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, () => emit('run'))
})

// External buffer swaps (e.g. loading an example) replace the model value.
watch(
  () => props.doc.code,
  (code) => {
    if (editor && editor.getValue() !== code) editor.setValue(code)
  }
)

watch(
  () => [settings.settings.fontSize, settings.settings.tabSize, settings.settings.fontLigatures],
  ([fontSize, tabSize, ligatures]) => {
    editor?.updateOptions({
      fontSize: Number(fontSize),
      tabSize: Number(tabSize),
      fontLigatures: Boolean(ligatures)
    })
  }
)

watch(currentTheme, (theme) => {
  import('monaco-editor').then((monaco) => monaco.editor.setTheme(theme))
})

onBeforeUnmount(() => {
  editor?.dispose()
})
</script>

<template>
  <div
    ref="container"
    class="h-full w-full overflow-hidden rounded-lg ring-1 ring-slate-300 dark:ring-slate-700"
    :style="height ? { height } : undefined"
    role="region"
    aria-label="Code editor"
  />
</template>

