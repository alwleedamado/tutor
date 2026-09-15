<script setup lang="ts">
import { computed } from 'vue'
import { useProgressStore } from '@/stores/progress.store'
import { useLessonsStore } from '@/stores/lessons.store'
import LockBadge from '@/components/common/LockBadge.vue'
import type { Track } from '@/types/domain'

const progress = useProgressStore()
const lessons = useLessonsStore()

interface NavItem {
  label: string
  icon: string
  to: string
  locked?: () => boolean
}

const items: NavItem[] = [
  { label: 'Dashboard', icon: '🏠', to: '/' },
  { label: 'Rust Track', icon: '🦀', to: '/rust' },
  { label: 'Vue Track', icon: '💚', to: '/vue' },
  { label: 'Tauri Track', icon: '🛡️', to: '/tauri', locked: () => !lessons.findModule('tauri', 'tauri-01') ? false : progress.isModuleLocked('tauri-01') },
  { label: 'Track Catalog', icon: '🗂️', to: '/tracks' },
  { label: 'Projects', icon: '📦', to: '/projects' },
  { label: 'Playground', icon: '🧪', to: '/playground' },
  { label: 'Capstone', icon: '🏆', to: '/capstone', locked: () => !progress.capstoneUnlocked },

  { label: 'Progress', icon: '📊', to: '/progress' },
  { label: 'Notes', icon: '📝', to: '/notes' },
  { label: 'Bookmarks', icon: '🔖', to: '/bookmarks' },
  { label: 'Settings', icon: '⚙️', to: '/settings' }
]

/** Utility items split from the primary nav (items above index 8). */
const primaryItems = items.slice(0, 8)
const utilityItems = items.slice(8)

const TRACK_ICONS: Record<string, string> = {
  'backend-rust': '⚙️',
  'backend-express': '🚂',
  'web-performance': '⚡',
  'web-accessibility': '♿',
  'web-security': '🔐',
  architecture: '🏛️',
  'vue-architecture': '🧱',
  http: '🌐',
  'api-design': '🔌',
  databases: '🗄️',
  testing: '✅',
  git: '🌿',
  linux: '🐧',
  devops: '🐳',
  observability: '📈'
}

const GROUP_ICONS: Record<string, string> = {
  Backend: '⚙️',
  Frontend: '🎨',
  Security: '🔐',
  Architecture: '🏛️',
  Foundations: '🧱',
  Operations: '🛠️'
}

interface SidebarTrack {
  id: string
  title: string
  icon: string
  locked: boolean
  /** Explains the exact blockers when locked ("Complete first: …"). */
  tooltip: string
}

const groups = computed(() =>
  lessons.sidebarGroups.map((group) => ({
    label: group.label,
    tracks: group.tracks.map(
      (track: Track): SidebarTrack => ({
        id: track.id,
        title: track.title,
        icon: TRACK_ICONS[track.id] ?? '📘',
        locked: firstModuleLocked(track),
        tooltip: lockTooltip(track)
      })
    )
  }))
)

/** A track is enterable when its first module is unlocked. */
function firstModuleLocked(track: Track): boolean {
  const first = [...track.modules].sort((a, b) => a.index - b.index)[0]
  return first ? progress.isModuleLocked(first.id) : false
}

/**
 * Explainable lock tooltip: names the entry module's missing prerequisites
 * (module title + owning track), mirroring the recommendation engine.
 */
function lockTooltip(track: Track): string {
  const first = [...track.modules].sort((a, b) => a.index - b.index)[0]
  if (!first || !progress.isModuleLocked(first.id)) {
    return track.title
  }
  const blockers = first.prerequisites
    .filter((id) => !progress.isModuleCompleted(id))
    .map((id) => {
      const moduleDef = lessons.moduleById.get(id)
      if (!moduleDef) return id
      const owner = lessons.findTrack(moduleDef.trackId)?.title
      return owner ? `${moduleDef.title} (${owner})` : moduleDef.title
    })
  const detail = blockers.length
    ? `complete first: ${blockers.join(', ')}`
    : 'complete its prerequisites first'
  return `${track.title} is locked — ${detail}`
}
</script>

<template>
  <aside
    class="flex w-56 shrink-0 flex-col border-r border-slate-200 bg-white dark:border-slate-800 dark:bg-[#101a2e]"
    aria-label="Primary navigation"
  >
    <div class="flex items-center gap-2 px-4 py-4">
      <span aria-hidden="true" class="text-2xl">🧭</span>
      <div>
        <p class="text-sm font-bold leading-tight text-slate-900 dark:text-white">Rust Mastery</p>
        <p class="text-xs text-slate-500 dark:text-slate-400">Offline Workbench</p>
      </div>
    </div>

    <nav class="flex-1 space-y-0.5 overflow-y-auto px-2 pb-4">
      <router-link
        v-for="item in primaryItems"
        :key="item.to"
        :to="item.to"
        class="flex items-center justify-between rounded-lg px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 dark:text-slate-300 dark:hover:bg-slate-800"
        exact-active-class="!bg-teal-600/10 !text-teal-700 dark:!text-teal-300"
        :aria-disabled="item.locked?.()"
      >
        <span class="flex items-center gap-2">
          <span aria-hidden="true">{{ item.icon }}</span>
          {{ item.label }}
        </span>
        <LockBadge v-if="item.locked?.()" />
      </router-link>

      <!-- Expanded curriculum, grouped by discipline -->
      <details
        v-for="group in groups"
        :key="group.label"
        class="group/sidebar-section mt-1 rounded-lg"
      >
        <summary
          class="flex cursor-pointer select-none items-center gap-2 rounded-lg px-3 py-2 text-xs font-semibold uppercase tracking-wider text-slate-400 hover:bg-slate-100 hover:text-slate-600 dark:hover:bg-slate-800 dark:hover:text-slate-300 [&::-webkit-details-marker]:hidden"
        >
          <span class="transition-transform group-open/sidebar-section:rotate-90" aria-hidden="true">▸</span>
          <span aria-hidden="true">{{ GROUP_ICONS[group.label] ?? '📚' }}</span>
          {{ group.label }}
          <span class="ml-auto text-[10px] normal-case text-slate-400">{{ group.tracks.length }}</span>
        </summary>
        <ul class="mt-0.5 space-y-0.5 pl-3">
          <li v-for="track in group.tracks" :key="track.id">
            <router-link
              v-if="!track.locked"
              :to="`/${track.id}`"
              class="flex items-center gap-2 rounded-lg px-3 py-1.5 text-sm text-slate-600 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800"
              active-class="!bg-teal-600/10 !text-teal-700 dark:!text-teal-300"
            >
              <span aria-hidden="true">{{ track.icon }}</span>
              <span class="truncate">{{ track.title }}</span>
            </router-link>
            <span
              v-else
              class="flex cursor-not-allowed items-center justify-between gap-2 rounded-lg px-3 py-1.5 text-sm text-slate-400 dark:text-slate-600"
              :title="track.tooltip"
            >
              <span class="flex min-w-0 items-center gap-2">
                <span aria-hidden="true">{{ track.icon }}</span>
                <span class="truncate">{{ track.title }}</span>
              </span>
              <LockBadge label="" />
            </span>
          </li>
        </ul>
      </details>

      <router-link
        v-for="item in utilityItems"
        :key="item.to"
        :to="item.to"
        class="flex items-center justify-between rounded-lg px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 dark:text-slate-300 dark:hover:bg-slate-800"
        exact-active-class="!bg-teal-600/10 !text-teal-700 dark:!text-teal-300"
        :aria-disabled="item.locked?.()"
      >
        <span class="flex items-center gap-2">
          <span aria-hidden="true">{{ item.icon }}</span>
          {{ item.label }}
        </span>
        <LockBadge v-if="item.locked?.()" />
      </router-link>
    </nav>
  </aside>
</template>
