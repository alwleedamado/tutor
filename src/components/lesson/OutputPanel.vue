<script setup lang="ts">
import { computed, ref } from 'vue'
import { useExecutionStore } from '@/stores/execution.store'
import DiagnosticsList from './DiagnosticsList.vue'

const execution = useExecutionStore()
const tab = ref<'output' | 'diagnostics'>('output')

const result = computed(() => execution.result)
const hasDiagnostics = computed(() => (result.value?.diagnostics.length ?? 0) > 0)

const statusLabel = computed(() => {
  switch (execution.status) {
    case 'running':
      return 'Compiling / running…'
    case 'error':
      return `Error: ${execution.errorMessage ?? 'unknown'}`
    case 'done':
      return result.value?.timedOut
        ? 'Timed out after 5s'
        : result.value?.cancelled
          ? 'Cancelled'
          : result.value?.ok
            ? `Finished in ${result.value.durationMs} ms`
            : 'Process exited with an error'
    default:
      return 'Run your code to see output here.'
  }
})
</script>

<template>
  <section class="flex min-h-[180px] flex-col rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-[#101a2e]" aria-label="Execution output">
    <header class="flex items-center gap-2 border-b border-slate-200 px-3 py-2 dark:border-slate-800">
      <button
        class="rounded-md px-2 py-1 text-xs font-semibold"
        :class="tab === 'output' ? 'bg-teal-600/10 text-teal-700 dark:text-teal-300' : 'text-slate-500'"
        @click="tab = 'output'"
      >
        Output
      </button>
      <button
        class="relative rounded-md px-2 py-1 text-xs font-semibold"
        :class="tab === 'diagnostics' ? 'bg-teal-600/10 text-teal-700 dark:text-teal-300' : 'text-slate-500'"
        @click="tab = 'diagnostics'"
      >
        Diagnostics
        <span
          v-if="hasDiagnostics"
          class="ml-1 inline-block h-1.5 w-1.5 rounded-full align-middle"
          :class="hasDiagnostics && result?.diagnostics.some((d) => d.severity === 'error') ? 'bg-red-500' : 'bg-amber-400'"
        />
      </button>

      <span class="ml-auto truncate text-xs text-slate-500 dark:text-slate-400" data-testid="exec-status">
        {{ statusLabel }}
      </span>
      <button v-if="execution.isBusy" class="btn-danger !px-2 !py-0.5 text-xs" @click="execution.cancel()">
        Stop
      </button>
    </header>

    <div class="min-h-0 flex-1 overflow-auto p-3 text-sm">
      <template v-if="tab === 'output'">
        <pre v-if="result" class="whitespace-pre-wrap font-mono text-xs leading-relaxed text-slate-100"><template v-if="result.stdout"><span class="text-slate-500">stdout ─────────</span>
{{ result.stdout }}</template><template v-if="result.stderr">

<span class="text-red-400">stderr ─────────</span>
{{ result.stderr }}</template></pre>
        <p v-else-if="!execution.isBusy" class="italic text-slate-400">No output yet.</p>
        <p v-else class="animate-pulse italic text-teal-600 dark:text-teal-300">Working…</p>
      </template>

      <template v-else>
        <DiagnosticsList v-if="hasDiagnostics" :diagnostics="result!.diagnostics" />
        <p v-else class="italic text-slate-400">No compiler diagnostics.</p>
      </template>
    </div>
  </section>
</template>
