<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useLessonsStore } from '@/stores/lessons.store'
import { useProgressStore } from '@/stores/progress.store'
import { useProjectStore } from '@/stores/project.store'
import LockBadge from '@/components/common/LockBadge.vue'

const lessons = useLessonsStore()
const progress = useProgressStore()
const projects = useProjectStore()

onMounted(() => void projects.ensureStates())

interface Gate {
  moduleId: string
  title: string
  trackId: string
  done: boolean
}

const gates = computed<Gate[]>(() => {
  const prereqs = lessons.capstone?.prerequisites ?? []
  return prereqs.map((moduleId) => {
    const moduleDef = lessons.moduleById.get(moduleId)
    return {
      moduleId,
      title: moduleDef?.title ?? moduleId,
      trackId: moduleDef?.trackId ?? '',
      done: progress.isModuleCompleted(moduleId)
    }
  })
})

const unlocked = computed(() => progress.capstoneUnlocked)
const capstoneSpec = computed(() => projects.specById('capstone-rust-playground'))
</script>

<template>
  <div class="mx-auto max-w-4xl space-y-6">
    <header class="text-center">
      <p class="text-6xl" aria-hidden="true">{{ unlocked ? '🏆' : '🔒' }}</p>
      <h1 class="mt-2 text-2xl font-bold text-slate-900 dark:text-white">Rust Playground Offline</h1>
      <p class="mx-auto mt-1 max-w-2xl text-sm leading-relaxed text-slate-600 dark:text-slate-300">
        The capstone: a complete offline developer-learning environment — lesson system, Markdown renderer,
        Monaco editor, Rust compiler integration, test runner, diagnostics, progress tracking, notes and
        SQLite persistence. Everything you learned across all three tracks, composed into one app.
      </p>
    </header>

    <section class="card p-5">
      <h2 class="mb-3 font-semibold text-slate-900 dark:text-white">Unlock checklist</h2>
      <ul class="space-y-2">
        <li v-for="gate in gates" :key="gate.moduleId" class="flex items-center gap-3 text-sm">
          <span
            class="flex h-6 w-6 items-center justify-center rounded-full text-xs font-bold"
            :class="gate.done ? 'bg-emerald-500 text-white' : 'bg-slate-200 text-slate-500 dark:bg-slate-700'"
          >
            {{ gate.done ? '✓' : '' }}
          </span>
          <span class="flex-1">{{ gate.title }}</span>
          <LockBadge v-if="!gate.done" label="" />
        </li>
      </ul>
      <p v-if="unlocked" class="mt-3 rounded-lg bg-emerald-600/10 px-3 py-2 text-sm text-emerald-700 dark:text-emerald-300">
        🎉 All gates cleared — the playground is yours to build.
      </p>
    </section>

    <template v-if="capstoneSpec">
      <section class="card p-5">
        <h2 class="mb-2 font-semibold text-slate-900 dark:text-white">What you will ship</h2>
        <ul class="list-disc space-y-1 pl-5 text-sm text-slate-700 dark:text-slate-300">
          <li v-for="feature in capstoneSpec.requirements" :key="feature">{{ feature }}</li>
        </ul>
      </section>

      <router-link v-if="unlocked" :to="`/project/${capstoneSpec.id}`" class="btn-primary mx-auto block w-fit">
        Open the capstone project →
      </router-link>
      <router-link v-else to="/progress" class="btn-ghost mx-auto block w-fit">
        Review outstanding work →
      </router-link>
    </template>
  </div>
</template>
