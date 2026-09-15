<script setup lang="ts">
import { computed } from 'vue'
import { useLessonsStore } from '@/stores/lessons.store'
import { useProgressStore } from '@/stores/progress.store'
import { useSkillStore } from '@/stores/skill.store'
import ProgressBar from '@/components/common/ProgressBar.vue'
import LockBadge from '@/components/common/LockBadge.vue'

/**
 * Full track catalog grouped by category — the low-noise home for the
 * expanded curriculum (foundations + specialized tracks).
 */
const lessons = useLessonsStore()
const progress = useProgressStore()
const skills = useSkillStore()

const groups = computed(() => lessons.tracksByCategory)

const demonstrated = computed(() => skills.demonstratedIds)

function trackHref(trackId: string): string {
  return `/${trackId}`
}
</script>

<template>
  <div class="mx-auto max-w-6xl space-y-6">
    <header>
      <h1 class="text-2xl font-bold text-slate-900 dark:text-white">Track catalog</h1>
      <p class="text-sm text-slate-500 dark:text-slate-400">
        {{ lessons.tracks.length }} tracks across {{ groups.length }} categories — foundations first, specialists after.
        Prerequisites are enforced automatically.
      </p>
    </header>

    <!-- Skill graph summary -->
    <section class="card p-4">
      <h2 class="mb-1 text-sm font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400">
        Skill graph
      </h2>
      <p class="mb-3 text-xs text-slate-500 dark:text-slate-400">
        Skills gather evidence from completed lessons — {{ demonstrated.size }} of
        {{ lessons.skillGraph?.skills.length ?? 0 }} demonstrated.
      </p>
      <ul v-if="skills.sortedSkills.length" class="flex flex-wrap gap-1.5">
        <li v-for="skill in skills.sortedSkills" :key="skill.id">
          <span
            v-if="!skill.dependenciesMet"
            class="badge cursor-not-allowed bg-slate-200 text-slate-400 dark:bg-slate-800 dark:text-slate-600"
            :title="`Needs: ${skill.missingDependencies.join(', ')}`"
          >🔒 {{ skill.label }}</span>
          <span
            v-else
            class="badge"
            :class="
              skill.demonstrated
                ? 'bg-emerald-600/10 text-emerald-700 dark:text-emerald-300'
                : 'bg-teal-600/10 text-teal-700 dark:text-teal-300'
            "
            :title="`${skill.lessonsDone}/${skill.lessonsTotal} evidence lessons complete`"
          >
            {{ skill.demonstrated ? '✔' : '◍' }} {{ skill.label }}
          </span>
        </li>
      </ul>
      <p v-else class="text-sm italic text-slate-400">Skill graph not loaded.</p>
    </section>

    <section v-for="group in groups" :key="group.category" class="space-y-3">
      <h2 class="text-xs font-semibold uppercase tracking-wider text-slate-400">{{ group.category }}</h2>
      <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
        <router-link
          v-for="track in group.tracks"
          :key="track.id"
          :to="trackHref(track.id)"
          class="card flex flex-col gap-2 p-4 hover:border-teal-500/50"
        >
          <header class="flex items-start justify-between gap-2">
            <h3 class="font-semibold text-slate-900 dark:text-white">{{ track.title }}</h3>
            <span class="text-xs text-slate-500 dark:text-slate-400">
              {{ track.modules.length }} modules
            </span>
          </header>
          <ProgressBar :percent="progress.trackPercent(track.id)" />
          <p class="line-clamp-2 text-xs leading-relaxed text-slate-500 dark:text-slate-400">
            {{ track.description }}
          </p>
          <ul class="mt-auto flex flex-wrap gap-1 text-[11px]">
            <li v-for="moduleDef in track.modules" :key="moduleDef.id" class="badge bg-slate-200 text-slate-600 dark:bg-slate-800 dark:text-slate-300">
              <LockBadge v-if="progress.isModuleLocked(moduleDef.id)" label="" />
              <span v-else-if="progress.isModuleCompleted(moduleDef.id)">✔</span>
              <span v-else>▶</span>
              {{ moduleDef.title }}
            </li>
          </ul>
        </router-link>
      </div>
    </section>
  </div>
</template>
