<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'

import { useProjectStore } from '@/stores/project.store'
import ProgressBar from '@/components/common/ProgressBar.vue'

const props = defineProps<{ id: string }>()

const projects = useProjectStore()


const revealedHints = ref(0)

onMounted(async () => {
  await projects.ensureStates()
})

const spec = computed(() => projects.specById(props.id))
const percent = computed(() =>
  spec.value ? projects.overallProgress(spec.value.id, spec.value.milestones.length) : 0
)
const criteriaDone = ref<Record<string, boolean>>({})

function toggleMilestone(milestoneId: string) {
  if (spec.value) void projects.toggleMilestone(spec.value, milestoneId)
}

async function markComplete() {
  if (!spec.value) return
  const allCriteria =
    spec.value.completionCriteria.every((c) => criteriaDone.value[c]) ||
    confirm('Mark complete even though not every criterion is checked?')
  if (!allCriteria) {
    alert('Check each completion criterion first — honesty makes better engineers. 😉')
    return
  }
  await projects.setCompleted(spec.value.id, true)
}
</script>

<template>
  <div v-if="spec" class="mx-auto max-w-4xl space-y-6">
    <header class="space-y-2">
      <router-link to="/projects" class="text-sm text-teal-600 hover:underline dark:text-teal-300">← All projects</router-link>
      <div class="flex flex-wrap items-center gap-3">
        <h1 class="text-2xl font-bold text-slate-900 dark:text-white">{{ spec.title }}</h1>
        <span
          v-if="projects.isCompleted(spec.id)"
          class="badge bg-emerald-600/10 text-emerald-700 dark:text-emerald-300"
        >✔ Completed</span>
      </div>
      <p class="max-w-3xl text-sm leading-relaxed text-slate-600 dark:text-slate-300">{{ spec.summary }}</p>
      <ProgressBar :percent="percent" />
    </header>

    <section class="grid gap-4 md:grid-cols-2">
      <div class="card p-4">
        <h2 class="mb-2 font-semibold text-slate-900 dark:text-white">Requirements</h2>
        <ul class="list-disc space-y-1 pl-5 text-sm text-slate-700 dark:text-slate-300">
          <li v-for="req in spec.requirements" :key="req">{{ req }}</li>
        </ul>
      </div>

      <div class="card p-4">
        <h2 class="mb-2 font-semibold text-slate-900 dark:text-white">Completion criteria</h2>
        <ul class="space-y-1.5 text-sm">
          <li v-for="criterion in spec.completionCriteria" :key="criterion">
            <label class="flex cursor-pointer items-start gap-2">
              <input v-model="criteriaDone[criterion]" type="checkbox" class="mt-0.5 accent-teal-600" />
              <span>{{ criterion }}</span>
            </label>
          </li>
        </ul>
        <button class="btn-primary mt-3 w-full" @click="markComplete">Mark project complete</button>
      </div>
    </section>

    <section class="card p-4">
      <h2 class="mb-3 font-semibold text-slate-900 dark:text-white">Milestones</h2>
      <ol class="space-y-2">
        <li v-for="(milestone, i) in spec.milestones" :key="milestone.id">
          <label class="flex cursor-pointer items-start gap-3 rounded-lg p-2 hover:bg-slate-100 dark:hover:bg-slate-800/60">
            <input
              type="checkbox"
              class="mt-0.5 accent-teal-600"
              :checked="projects.milestonesDone(spec.id).has(milestone.id)"
              @change="toggleMilestone(milestone.id)"
            />
            <span>
              <strong class="text-sm">{{ i + 1 }}. {{ milestone.title }}</strong>
              <span class="block text-xs text-slate-500 dark:text-slate-400">{{ milestone.detail }}</span>
            </span>
          </label>
        </li>
      </ol>
    </section>

    <section class="card p-4">
      <h2 class="mb-2 font-semibold text-slate-900 dark:text-white">Starter files</h2>
      <details v-for="file in spec.starterFiles" :key="file.name" class="mb-2">
        <summary class="cursor-pointer text-sm font-medium text-teal-700 dark:text-teal-300">{{ file.name }}</summary>
        <pre class="mt-1 max-h-72 overflow-auto rounded bg-slate-950 p-3 font-mono text-xs text-slate-100">{{ file.code }}</pre>
      </details>
      <p class="text-xs italic text-slate-400">
        Copy these into your own workspace to build offline; keep the project anywhere you like.
      </p>
    </section>

    <section class="card p-4">
      <h2 class="mb-2 font-semibold text-slate-900 dark:text-white">Hints</h2>
      <button
        v-if="revealedHints < spec.hints.length"
        class="btn-ghost mb-2"
        @click="revealedHints++"
      >
        💡 Reveal hint {{ revealedHints + 1 }}/{{ spec.hints.length }}
      </button>
      <ol class="space-y-1.5 text-sm">
        <li
          v-for="(hint, i) in spec.hints.slice(0, revealedHints)"
          :key="i"
          class="rounded bg-amber-500/10 px-2 py-1 text-amber-800 dark:text-amber-200"
        >
          {{ hint }}
        </li>
      </ol>
    </section>
  </div>

  <p v-else class="italic text-slate-400">Loading project…</p>
</template>
