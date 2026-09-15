<script setup lang="ts">
import { computed } from 'vue'
import { useLessonsStore } from '@/stores/lessons.store'
import { useProgressStore } from '@/stores/progress.store'
import TrackCard from '@/components/dashboard/TrackCard.vue'

const lessons = useLessonsStore()
const progress = useProgressStore()

const nextTarget = computed(() => {
  const next = progress.nextUp
  if (!next) return null
  const track = lessons.findTrack(next.trackId)
  const moduleDef = lessons.findModule(next.trackId, next.moduleId)
  const lesson = moduleDef?.lessons.find((l) => l.slug === next.lessonSlug)
  if (!track || !moduleDef || !lesson) return null
  return {
    to: `/${next.trackId}/${moduleDef.slug}/${lesson.slug}`,
    trackTitle: track.title,
    moduleTitle: moduleDef.title,
    lessonTitle: lesson.title,
    minutes: lesson.minutes
  }
})

const blockedMessage = computed(() => {
  const blocked = new URLSearchParams(window.location.hash.split('?')[1] ?? '').get('blocked')
  if (!blocked) return null
  if (blocked === 'capstone') return 'The capstone unlocks after Rust M6, Vue M4 and Tauri M3.'
  if (blocked === 'missing-lesson') return 'That lesson does not exist.'
  return 'Complete the prerequisites first — check the module badges below.'
})

const recent = computed(() =>
  progress.recentCompletions.map((entry) => {
    const moduleDef = lessons.moduleById.get(entry.moduleId)
    const lesson = moduleDef?.lessons.find((l) => l.id === entry.lessonId)
    return {
      key: `${entry.trackId}:${entry.moduleId}:${entry.lessonId}`,
      label: lesson?.title ?? entry.lessonId,
      where: moduleDef?.title ?? entry.moduleId,
      icon: entry.trackId === 'rust' ? '🦀' : entry.trackId === 'vue' ? '💚' : '🛡️',
      at: new Date(entry.completedAt * 1000)
    }
  })
)

/** Extra recommendations beyond the hero "continue" target, deterministically ordered. */
const extraRecommendations = computed(() =>
  progress.recommendations.filter((r) => r.kind !== 'next-lesson').slice(0, 3)
)

const KIND_ICONS: Record<string, string> = {
  prerequisite: '🔓',
  review: '🔁',
  project: '📦',
  'skill-gap': '🧩'
}

const CORE_TRACK_IDS = new Set(['rust', 'vue', 'tauri'])

/** Category groups holding anything beyond the original three core tracks. */
const specialistGroups = computed(() =>
  lessons.tracksByCategory.filter((group) =>
    group.tracks.some((track) => !CORE_TRACK_IDS.has(track.id))
  )
)

function recommendationTo(rec: (typeof progress.recommendations)[number]) {
  const moduleDef = rec.moduleId ? lessons.moduleById.get(rec.moduleId) : undefined
  let to = '/progress'
  if (rec.kind === 'project' && rec.projectId) {
    to = `/project/${rec.projectId}`
  } else if (rec.trackId && moduleDef) {
    const slug =
      (rec.lessonSlug && moduleDef.lessons.find((l) => l.slug === rec.lessonSlug)?.slug) ??
      moduleDef.lessons[0]?.slug
    if (slug) to = `/${rec.trackId}/${moduleDef.slug}/${slug}`
  }
  return { ...rec, to }
}

function timeAgo(date: Date): string {
  const seconds = Math.max(1, Math.floor((Date.now() - date.getTime()) / 1000))
  if (seconds < 60) return 'just now'
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`
  return date.toLocaleDateString()
}
</script>

<template>
  <div class="mx-auto max-w-6xl space-y-6">
    <p v-if="blockedMessage" class="card border-l-4 !border-l-amber-500 p-3 text-sm" role="alert">
      🔐 {{ blockedMessage }}
    </p>

    <!-- Continue -->
    <section v-if="nextTarget" class="card flex flex-col gap-2 p-5 sm:flex-row sm:items-center">
      <div class="min-w-0 flex-1">
        <p class="text-xs font-semibold uppercase tracking-wide text-teal-600 dark:text-teal-300">Recommended next</p>
        <h1 class="truncate text-lg font-bold text-slate-900 dark:text-white">{{ nextTarget.lessonTitle }}</h1>
        <p class="text-sm text-slate-500 dark:text-slate-400">
          {{ nextTarget.trackTitle }} · {{ nextTarget.moduleTitle }} · ~{{ nextTarget.minutes }} min
        </p>
      </div>
      <router-link :to="nextTarget.to" class="btn-primary shrink-0">Continue ▶</router-link>
    </section>
    <section v-else class="card p-5 text-center">
      <p class="text-lg font-bold text-emerald-600 dark:text-emerald-400">🎉 Every unlocked lesson is complete!</p>
      <p class="text-sm text-slate-500 dark:text-slate-400">Finish remaining tracks to unlock more.</p>
    </section>

    <!-- Deterministic next-up recommendations -->
    <section v-if="extraRecommendations.length" class="card p-4">
      <h2 class="mb-2 text-sm font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400">
        Recommended for you
      </h2>
      <ul class="grid gap-2 md:grid-cols-3">
        <li v-for="rec in extraRecommendations.map(recommendationTo)" :key="rec.kind + rec.title">
          <router-link :to="rec.to" class="block rounded-lg border border-slate-200 p-3 hover:border-teal-500/50 dark:border-slate-700">
            <p class="text-sm font-medium text-slate-900 dark:text-white">
              <span aria-hidden="true">{{ KIND_ICONS[rec.kind] ?? '➡️' }}</span>
              {{ rec.title }}
            </p>
            <p class="mt-1 line-clamp-2 text-xs text-slate-500 dark:text-slate-400">{{ rec.reason }}</p>
          </router-link>
        </li>
      </ul>
    </section>

    <!-- Core tracks -->
    <div class="grid gap-4 md:grid-cols-3">
      <TrackCard v-for="track in lessons.tracks.slice(0, 3)" :key="track.id" :track="track" />
    </div>

    <!-- Foundations & specialist tracks, grouped by category -->
    <section class="space-y-4">
      <h2 class="text-lg font-bold text-slate-900 dark:text-white">Expand your engineering practice</h2>
      <div v-for="group in specialistGroups" :key="group.category" class="space-y-2">
        <h3 class="text-xs font-semibold uppercase tracking-wider text-slate-400">{{ group.category }}</h3>
        <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
          <TrackCard v-for="track in group.tracks" :key="track.id" :track="track" />
        </div>
      </div>
      <p class="text-xs italic text-slate-400">
        Browse everything in the <router-link to="/tracks" class="text-teal-600 underline dark:text-teal-300">track catalog</router-link>.
      </p>
    </section>

    <div class="grid gap-4 md:grid-cols-2">
      <!-- Capstone -->
      <section class="card p-5">
        <header class="flex items-center justify-between">
          <h2 class="font-semibold text-slate-900 dark:text-white">🏆 Capstone</h2>
          <span
            class="badge"
            :class="progress.capstoneUnlocked ? 'bg-emerald-600/10 text-emerald-700 dark:text-emerald-300' : 'bg-slate-200 text-slate-500 dark:bg-slate-800'"
          >
            {{ progress.capstoneUnlocked ? 'Unlocked' : 'Locked' }}
          </span>
        </header>
        <p class="mt-2 text-sm text-slate-600 dark:text-slate-300">
          Build <strong>Rust Playground Offline</strong> — a full offline IDE reusing everything the three tracks taught.
        </p>
        <router-link v-if="progress.capstoneUnlocked" to="/capstone" class="btn-primary mt-3">Open capstone</router-link>
        <router-link v-else to="/progress" class="btn-ghost mt-3">See what's missing</router-link>
      </section>

      <!-- Recent activity -->
      <section class="card p-5">
        <h2 class="mb-2 font-semibold text-slate-900 dark:text-white">Recent activity</h2>
        <ul v-if="recent.length" class="space-y-1.5 text-sm">
          <li v-for="item in recent" :key="item.key" class="flex items-center gap-2">
            <span aria-hidden="true">{{ item.icon }}</span>
            <span class="min-w-0 flex-1 truncate text-slate-700 dark:text-slate-300">{{ item.label }}</span>
            <span class="shrink-0 text-xs text-slate-400">{{ item.where }} · {{ timeAgo(item.at) }}</span>
          </li>
        </ul>
        <p v-else class="text-sm italic text-slate-400">Nothing yet — your completions will appear here.</p>
      </section>
    </div>
  </div>
</template>
