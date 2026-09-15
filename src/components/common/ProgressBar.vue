<script setup lang="ts">
const props = withDefaults(defineProps<{ percent: number; showLabel?: boolean }>(), {
  showLabel: true
})
const clamped = Math.max(0, Math.min(100, Math.round(props.percent)))
</script>

<template>
  <div class="flex items-center gap-2" role="progressbar" :aria-valuenow="clamped" aria-valuemin="0" aria-valuemax="100">
    <div class="h-2 flex-1 overflow-hidden rounded-full bg-slate-200 dark:bg-slate-700">
      <div class="h-full rounded-full bg-teal-500 transition-all" :style="{ width: `${clamped}%` }" />
    </div>
    <span v-if="showLabel" class="w-10 text-right text-xs tabular-nums text-slate-500 dark:text-slate-400">
      {{ clamped }}%
    </span>
  </div>
</template>
