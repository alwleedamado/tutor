<script setup lang="ts">
import { useLessonsStore } from '@/stores/lessons.store'
import { useProgressStore } from '@/stores/progress.store'
import ProgressBar from '@/components/common/ProgressBar.vue'

const lessons = useLessonsStore()
const progress = useProgressStore()
</script>

<template>
  <div class="mx-auto max-w-5xl space-y-6">
    <h1 class="text-2xl font-bold text-slate-900 dark:text-white">Progress</h1>

    <section v-for="track in lessons.tracks" :key="track.id" class="card p-5">
      <header class="mb-3 flex items-center justify-between">
        <h2 class="font-semibold text-slate-900 dark:text-white">{{ track.title }}</h2>
        <span class="text-sm tabular-nums text-slate-500 dark:text-slate-400">{{ progress.trackPercent(track.id) }}%</span>
      </header>
      <ProgressBar :percent="progress.trackPercent(track.id)" :show-label="false" />

      <ul class="mt-4 space-y-2">
        <li v-for="moduleDef in track.modules" :key="moduleDef.id" class="flex items-center gap-3 text-sm">
          <span
            class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-[11px] font-bold"
            :class="
              progress.isModuleCompleted(moduleDef.id)
                ? 'bg-emerald-500 text-white'
                : progress.isModuleLocked(moduleDef.id)
                  ? 'bg-slate-200 text-slate-400 dark:bg-slate-800 dark:text-slate-500'
                  : 'bg-teal-600/15 text-teal-700 dark:text-teal-300'
            "
          >
            {{ progress.isModuleCompleted(moduleDef.id) ? '✓' : progress.isModuleLocked(moduleDef.id) ? '🔒' : moduleDef.index }}
          </span>
          <span class="min-w-0 flex-1 truncate" :class="{ 'text-slate-400': progress.isModuleLocked(moduleDef.id) }">
            {{ moduleDef.title }}
          </span>
          <span class="tabular-nums text-xs text-slate-400">
            {{ progress.moduleState(moduleDef.id)?.done ?? 0 }}/{{ progress.moduleState(moduleDef.id)?.total ?? 0 }}
          </span>
        </li>
      </ul>
    </section>
  </div>
</template>
