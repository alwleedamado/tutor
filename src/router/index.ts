import { createRouter, createWebHashHistory } from 'vue-router'

import DashboardView from '@/views/DashboardView.vue'
import { useLessonsStore } from '@/stores/lessons.store'
import { useProgressStore } from '@/stores/progress.store'
import { useProjectStore } from '@/stores/project.store'

/**
 * Route table per docs/frontend-architecture.md. Track routes share
 * LessonView; prerequisite enforcement lives HERE and only here.
 */
export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'dashboard', component: DashboardView },
    { path: '/rust/:module/:lesson', name: 'rust-lesson', component: () => import('@/views/LessonView.vue'), props: true, meta: { track: 'rust' } },
    { path: '/vue/:module/:lesson', name: 'vue-lesson', component: () => import('@/views/LessonView.vue'), props: true, meta: { track: 'vue' } },
    { path: '/tauri/:module/:lesson', name: 'tauri-lesson', component: () => import('@/views/LessonView.vue'), props: true, meta: { track: 'tauri' } },
    { path: '/rust', name: 'rust-entry', component: () => import('@/views/TrackEntryView.vue'), meta: { track: 'rust' } },
    { path: '/vue', name: 'vue-entry', component: () => import('@/views/TrackEntryView.vue'), meta: { track: 'vue' } },
    { path: '/tauri', name: 'tauri-entry', component: () => import('@/views/TrackEntryView.vue'), meta: { track: 'tauri' } },
    { path: '/rust/:module', name: 'rust-module-entry', component: () => import('@/views/TrackEntryView.vue'), props: true, meta: { track: 'rust' } },
    { path: '/vue/:module', name: 'vue-module-entry', component: () => import('@/views/TrackEntryView.vue'), props: true, meta: { track: 'vue' } },
    { path: '/tauri/:module', name: 'tauri-module-entry', component: () => import('@/views/TrackEntryView.vue'), props: true, meta: { track: 'tauri' } },

    /**
     * Generic track routes for every catalog track beyond the original three
     * (foundations + specialized). The guard validates the track param
     * against the embedded catalog and injects meta.track so the shared
     * LessonView / TrackEntryView work unchanged. Registered AFTER the named
     * core routes above, which keep their deep-link names.
     */
    { path: '/:track/:module/:lesson', name: 'track-lesson', component: () => import('@/views/LessonView.vue'), props: true },
    { path: '/:track/:module', name: 'track-module-entry', component: () => import('@/views/TrackEntryView.vue'), props: true },
    { path: '/:track', name: 'track-entry', component: () => import('@/views/TrackEntryView.vue') },

    { path: '/tracks', name: 'tracks', component: () => import('@/views/TracksView.vue') },

    { path: '/projects', name: 'projects', component: () => import('@/views/ProjectsView.vue') },
    { path: '/project/:id', name: 'project', component: () => import('@/views/ProjectView.vue'), props: true },
    { path: '/capstone', name: 'capstone', component: () => import('@/views/CapstoneView.vue') },
    { path: '/playground/:lang?', name: 'playground', component: () => import('@/views/PlaygroundView.vue') },
    { path: '/progress', name: 'progress', component: () => import('@/views/ProgressView.vue') },

    { path: '/search', name: 'search', component: () => import('@/views/SearchView.vue') },
    { path: '/bookmarks', name: 'bookmarks', component: () => import('@/views/BookmarksView.vue') },
    { path: '/notes', name: 'notes', component: () => import('@/views/NotesView.vue') },
    { path: '/settings', name: 'settings', component: () => import('@/views/SettingsView.vue') },
    { path: '/:pathMatch(.*)*', name: 'not-found', component: () => import('@/views/NotFoundView.vue') }
  ]
})

router.beforeEach(async (to) => {
  const lessons = useLessonsStore()
  await lessons.ensureLoaded()

  const progress = useProgressStore()

  // Generic track routes: validate the track against the catalog and inject
  // meta.track so the shared views keep working for any track id.
  if (!to.meta.track && typeof to.params.track === 'string') {
    if (!lessons.isTrackId(to.params.track)) {
      return { name: 'not-found' }
    }
    to.meta.track = to.params.track
  }

  // Lesson routes: block when the owning module is locked by prerequisites.
  // Track/module ENTRY routes carry meta.track too but have no lesson param.
  if (
    typeof to.meta.track === 'string' &&
    typeof to.params.module === 'string' &&
    typeof to.params.lesson === 'string'
  ) {
    const moduleParam = String(to.params.module)
    const lessonParam = String(to.params.lesson)
    const moduleDef = lessons.findModule(String(to.meta.track), moduleParam)
    if (
      !moduleDef ||
      !lessons.findLesson(String(to.meta.track), moduleParam, lessonParam)
    ) {
      return { name: 'dashboard', query: { blocked: 'missing-lesson' } }
    }
    if (progress.isModuleLocked(moduleDef.id)) {
      return { name: 'dashboard', query: { blocked: moduleDef.id } }
    }
  }


  // Capstone gate.
  if (to.name === 'capstone' && !progress.capstoneUnlocked) {
    return { name: 'dashboard', query: { blocked: 'capstone' } }
  }

  // Project gate: projects tied to locked modules are blocked too.
  if (to.name === 'project') {
    await useProjectStore().ensureStates()
    const spec = lessons.projectById(String(to.params.id))
    if (!spec) return { name: 'projects' }
    if (spec.moduleId && progress.isModuleLocked(spec.moduleId)) {
      return { name: 'dashboard', query: { blocked: spec.moduleId } }
    }
  }

  return true
})
