import { defineStore } from 'pinia'

import { ipc, isTauriRuntime } from '@/services/ipc'

import type {
  CapstoneSpec,
  Catalog,
  Lesson,
  Module,
  ProjectSpec,
  Settings,
  SkillGraph,
  Track,
  TrackId
} from '@/types/domain'
import { useProgressStore } from './progress.store'
import { useSettingsStore } from './settings.store'

interface LessonsState {
  catalog: Catalog | null
  capstone: CapstoneSpec | null
  projects: ProjectSpec[]
  skillGraph: SkillGraph | null
  ready: boolean
  loading: boolean
  /** Set when the one-shot bootstrap fails (e.g. browser without Tauri). */
  error: string | null
}


/**
 * Owns the embedded learning catalog and the one-shot bootstrap that
 * hydrates every other store. All lookups accept ids OR slugs so routes
 * can stay slug-based.
 */
export const useLessonsStore = defineStore('lessons', {
  state: (): LessonsState => ({
    catalog: null,
    capstone: null,
    projects: [],
    skillGraph: null,
    ready: false,
    loading: false,
    error: null
  }),


  getters: {
    tracks(state): Track[] {
      return state.catalog?.tracks ?? []
    },
    /** Tracks grouped by dashboard category in stable catalog order. */
    tracksByCategory(): Array<{ category: string; tracks: Track[] }> {
      const groups = new Map<string, Track[]>()
      for (const track of this.tracks) {
        const list = groups.get(track.category) ?? []
        list.push(track)
        groups.set(track.category, list)
      }
      return [...groups.entries()].map(([category, tracks]) => ({ category, tracks }))
    },
    /**
     * Sidebar navigation groups for the expanded curriculum (core tracks are
     * surfaced separately). Grouping is a centralized display concern — the
     * track membership comes from the catalog, never from components.
     */
    sidebarGroups(): Array<{ label: string; tracks: Track[] }> {
      const grouping: Record<string, string> = {
        'backend-rust': 'Backend',
        'backend-express': 'Backend',
        'web-performance': 'Frontend',
        'web-accessibility': 'Frontend',
        'web-security': 'Security',
        architecture: 'Architecture',
        'vue-architecture': 'Architecture',
        http: 'Foundations',
        'api-design': 'Foundations',
        databases: 'Foundations',
        testing: 'Foundations',
        git: 'Foundations',
        linux: 'Operations',
        devops: 'Operations',
        observability: 'Operations'
      }
      const order = ['Backend', 'Frontend', 'Security', 'Architecture', 'Foundations', 'Operations']
      const grouped = new Map<string, Track[]>()
      for (const track of this.tracks) {
        const label = grouping[track.id]
        if (!label) continue
        const list = grouped.get(label) ?? []
        list.push(track)
        grouped.set(label, list)
      }
      return order
        .filter((label) => grouped.has(label))
        .map((label) => ({ label, tracks: grouped.get(label) as Track[] }))
    },
    findTrack(): (trackId: string) => Track | undefined {
      return (trackId) => this.tracks.find((t) => t.id === trackId)
    },
    findModule(): (trackId: string, moduleSlugOrId: string) => Module | undefined {
      return (trackId, moduleSlugOrId) =>
        this.findTrack(trackId)?.modules.find(
          (m) => m.slug === moduleSlugOrId || m.id === moduleSlugOrId
        )
    },
    findLesson(): (trackId: string, moduleSlugOrId: string, lessonSlugOrId: string) => Lesson | undefined {
      return (trackId, moduleSlugOrId, lessonSlugOrId) =>
        this.findModule(trackId, moduleSlugOrId)?.lessons.find(
          (l) => l.slug === lessonSlugOrId || l.id === lessonSlugOrId
        )
    },
    moduleById(): Map<string, Module> {
      const map = new Map<string, Module>()
      for (const track of this.tracks) {
        for (const module of track.modules) map.set(module.id, module)
      }
      return map
    },
    projectById(state): (id: string) => ProjectSpec | undefined {
      return (id) => state.projects.find((p) => p.id === id)
    },
    skillById(): (skillId: string) => import('@/types/domain').SkillNode | undefined {
      return (skillId) => this.skillGraph?.skills.find((s) => s.id === skillId)
    }
  },

  actions: {
    /** Fetch the bootstrap payload exactly once; hydrates sibling stores. */
    async ensureLoaded(): Promise<void> {
      if (this.ready || this.loading) return
      this.loading = true
      try {
        const boot = await ipc.getBootstrap()
        this.catalog = boot.catalog
        this.capstone = boot.capstone
        this.projects = boot.projects
        this.skillGraph = boot.skillGraph
        useProgressStore().applySummary(boot.progress)
        useSettingsStore().hydrate(boot.settings)
        this.error = null
        this.ready = true
      } catch (error) {
        this.error = isTauriRuntime()
          ? error instanceof Error
            ? error.message
            : String(error)
          : 'This window has no Rust backend attached. Start the desktop shell with `npm run tauri dev` — a plain browser preview cannot load the offline curriculum.'
      } finally {
        this.loading = false
      }
    },

    /** Clear a failed bootstrap and try again (used by the error screen). */
    async retry(): Promise<void> {
      this.ready = false
      this.loading = false
      this.error = null
      await this.ensureLoaded()
    },

    isTrackId(value: string): value is TrackId {
      return this.tracks.some((t) => t.id === value)
    },

    /** First lesson route target of a module (first incomplete wins later). */
    moduleFirstLesson(module: Module): Lesson | undefined {
      return module.lessons[0]
    }
  }
})

/** Hydrate helper re-exported so settings typing stays local to its store. */
export type { Settings }
