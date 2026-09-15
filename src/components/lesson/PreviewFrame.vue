<script setup lang="ts">
import { computed, ref, watch } from 'vue'

import { buildPreviewDocument, VUE_RUNTIME_PATH } from '@/services/preview'
import { useSettingsStore } from '@/stores/settings.store'


/**
 * Sandboxed live preview for Vue/HTML snippets. The iframe runs with
 * `allow-scripts` only (null origin) — preview code is fully isolated from
 * the app. Rebuilds are debounced; `tick` forces an immediate refresh.
 */
const props = withDefaults(
  defineProps<{
    code: string
    height?: string
    tick?: number
    debounceMs?: number
  }>(),
  { height: '260px', tick: 0, debounceMs: 350 }
)

const settings = useSettingsStore()
const doc = ref('')

/**
 * Absolute runtime URL — resolved against the APP's base URI here, because a
 * sandboxed srcdoc iframe has an opaque origin and must never rely on its own
 * base resolution.
 */
const vueSrc = new URL(VUE_RUNTIME_PATH, document.baseURI).href

const isDark = computed(() => {
  const theme = settings.settings.theme
  if (theme === 'dark') return true
  if (theme === 'light') return false
  return window.matchMedia?.('(prefers-color-scheme: dark)').matches ?? true
})

let timer: ReturnType<typeof setTimeout> | undefined
function rebuild(): void {
  doc.value = buildPreviewDocument(props.code, { dark: isDark.value, vueSrc })
}


watch(
  [() => props.code, () => props.tick, isDark],
  () => {
    clearTimeout(timer)
    timer = setTimeout(rebuild, props.debounceMs)
  },
  { immediate: false }
)

// Initial + forced-refresh builds render immediately.
watch([() => props.tick], () => {
  rebuild()
})

rebuild()
</script>

<template>
  <div class="flex h-full min-h-0 flex-col gap-1">
    <p class="shrink-0 text-[11px] uppercase tracking-wide text-slate-400">Webview preview · sandboxed</p>
    <iframe
      sandbox="allow-scripts"
      :srcdoc="doc"
      title="Vue snippet preview"
      class="w-full flex-1 rounded-lg bg-white ring-1 ring-slate-300 dark:bg-[#0b1220] dark:ring-slate-700"
      :style="{ minHeight: height }"
    />
  </div>
</template>
