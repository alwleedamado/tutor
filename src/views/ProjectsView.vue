<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useLessonsStore } from '@/stores/lessons.store'
import { useProjectStore } from '@/stores/project.store'
import { useProgressStore } from '@/stores/progress.store'
import LockBadge from '@/components/common/LockBadge.vue'

const lessons = useLessonsStore()
const projects = useProjectStore()
const progress = useProgressStore()

onMounted(() => void projects.ensureStates())

const cards = computed(() =>
  lessons.projects.map((spec) => {
    const moduleDef = spec.moduleId ? lessons.moduleById.get(spec.moduleId) : undefined
    return {
      spec,
      locked: spec.moduleId ? progress.isModuleLocked(spec.moduleId) : false,
      completed: projects.isCompleted(spec.id),
      moduleTitle: moduleDef?.title ?? null,
      doneCount: projects.milestonesDone(spec.id).size
    }
  })
)
</script>

<template>
  <div class="mx-auto max-w-6xl space-y-5">
    <header>
      <h1 class="text-2xl font-bold text-slate-900 dark:text-white">Projects</h1>
      <p class="text-sm text-slate-500 dark:text-slate-400">
        First-class learning entities — requirements, milestones, hints and completion criteria for every build.
      </p>
    </header>

    <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      <router-link
        v-for="card in cards"
        :key="card.spec.id"
        :to="`/project/${card.spec.id}`"
        class="card group flex flex-col gap-2 p-4 transition-shadow hover:shadow-md"
        :aria-disabled="card.locked"
      >
        <div class="flex items-start justify-between gap-2">
          <h2 class="font-semibold text-slate-900 group-hover:text-teal-700 dark:text-white dark:group-hover:text-teal-300">
            {{ card.spec.title }}
          </h2>
          <LockBadge v-if="card.locked" label="" />
        </div>
        <p class="line-clamp-3 text-sm text-slate-500 dark:text-slate-400">{{ card.spec.summary }}</p>
        <div class="mt-auto flex flex-wrap items-center gap-1.5 text-xs">
          <span
            v-if="card.spec.trackId"
            class="badge bg-teal-600/10 text-teal-700 dark:text-teal-300"
          >{{ card.spec.trackId }}</span>
          <span v-if="card.moduleTitle" class="badge bg-slate-200 text-slate-600 dark:bg-slate-800 dark:text-slate-300">
            {{ card.moduleTitle }}
          </span>
          <span class="badge bg-slate-200 text-slate-600 dark:bg-slate-800 dark:text-slate-300">~{{ card.spec.estimatedHours }}h</span>
          <span
            v-if="card.completed"
            class="badge ml-auto bg-emerald-600/10 text-emerald-700 dark:text-emerald-300"
          >✔ Done · {{ card.doneCount }}/{{ card.spec.milestones.length }}</span>
          <span v-else-if="!card.locked && card.doneCount" class="ml-auto text-slate-400">
            {{ card.doneCount }}/{{ card.spec.milestones.length }} milestones
          </span>
        </div>
      </router-link>
    </div>
  </div>
</template>
