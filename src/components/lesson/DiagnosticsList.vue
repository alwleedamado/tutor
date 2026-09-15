<script setup lang="ts">
import type { Diagnostic } from '@/types/domain'

defineProps<{ diagnostics: Diagnostic[] }>()

const styleFor = (severity: string): string => {
  switch (severity) {
    case 'error':
      return 'bg-red-600/10 text-red-700 ring-red-600/30 dark:text-red-300'
    case 'warning':
      return 'bg-amber-500/10 text-amber-700 ring-amber-500/30 dark:text-amber-300'
    case 'help':
      return 'bg-teal-600/10 text-teal-700 ring-teal-600/30 dark:text-teal-300'
    default:
      return 'bg-slate-500/10 text-slate-600 ring-slate-500/30 dark:text-slate-300'
  }
}
</script>

<template>
  <ol class="space-y-1.5" aria-label="Compiler diagnostics">
    <li
      v-for="(diag, i) in diagnostics"
      :key="i"
      class="rounded-lg px-3 py-2 font-mono text-xs leading-relaxed ring-1"
      :class="styleFor(diag.severity)"
    >
      <span class="font-bold uppercase">{{ diag.severity }}</span>
      {{ diag.message }}
      <span v-if="diag.span" class="block opacity-70">{{ diag.span }}</span>
    </li>
  </ol>
</template>
