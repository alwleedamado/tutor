<script setup lang="ts">
import { computed } from 'vue'
import type { Track } from '@/types/domain'
import { useProgressStore } from '@/stores/progress.store'
import ProgressBar from '@/components/common/ProgressBar.vue'

const props = defineProps<{ track: Track }>()

const progress = useProgressStore()

const percent = computed(() => progress.trackPercent(props.track.id))

interface Chip {
  id: string
  title: string
  locked: boolean
  completed: boolean
}

const chips = computed<Chip[]>(() =>
  props.track.modules.map((m) => ({
    id: m.id,
    title: m.title,
    locked: progress.isModuleLocked(m.id),
    completed: progress.isModuleCompleted(m.id)
  }))
)

const doneCount = computed(
  () => chips.value.filter((c) => c.completed).length
)

/** Deep-link each module to its next actionable lesson (never a dead slug). */
function hrefFor(moduleId: string): string {
  const moduleDef = props.track.modules.find((m) => m.id === moduleId)
  if (!moduleDef || moduleDef.lessons.length === 0) {
    return `/${props.track.id}`
  }
  const next =
    moduleDef.lessons.find((l) => !progress.isLessonCompleted(props.track.id, moduleId, l.id)) ??
    moduleDef.lessons[0]
  return `/${props.track.id}/${moduleDef.slug}/${next.slug}`
}

</script>

<template>
  <section class="card flex flex-col gap-3 p-4">
    <header class="flex items-baseline justify-between">
      <h2 class="text-base font-semibold text-slate-900 dark:text-white">{{ track.title }}</h2>
      <span class="text-xs text-slate-500 dark:text-slate-400">
        {{ doneCount }}/{{ track.modules.length }} modules
      </span>
    </header>

    <ProgressBar :percent="percent" />

    <ul class="flex flex-wrap gap-1.5">
      <li v-for="chip in chips" :key="chip.id">
        <span
          v-if="chip.locked"
          class="badge cursor-not-allowed bg-slate-200 text-slate-500 dark:bg-slate-800 dark:text-slate-500"
        >
          🔒 {{ chip.title }}
        </span>
        <router-link
          v-else
          :to="hrefFor(chip.id)"
          class="badge"
          :class="
            chip.completed
              ? 'bg-emerald-600/10 text-emerald-700 dark:text-emerald-300'
              : 'bg-teal-600/10 text-teal-700 hover:bg-teal-600/20 dark:text-teal-300'
          "
        >

          {{ chip.completed ? '✔' : '▶' }} {{ chip.title }}
        </router-link>
      </li>
    </ul>

    <p class="text-xs leading-relaxed text-slate-500 dark:text-slate-400">{{ track.description }}</p>
  </section>
</template>
