<script setup lang="ts">
defineProps<{ label?: string }>()
</script>

<template>
  <span
    class="badge bg-slate-200 text-slate-600 dark:bg-slate-700 dark:text-slate-300"
    title="Locked — complete prerequisites first"
  >
    🔒 {{ label ?? 'Locked' }}
  </span>
</template>
