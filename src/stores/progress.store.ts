import { defineStore } from 'pinia'

import { ipc } from '@/services/ipc'
import { lessonKey } from '@/types/domain'
import type { ModuleState, ProgressSummary } from '@/types/domain'

interface ProgressState {
  summary: ProgressSummary | null
}

/**
 * Owns derived progress (completions, module stats, locks, recommendation).
 * The backend recomputes everything after each mutation, so store and
 * database can never drift apart.
 */
export const useProgressStore = defineStore('progress', {
  state: (): ProgressState => ({
    summary: null
  }),

  getters: {
    completedKeys(state): Set<string> {
      const set = new Set<string>()
      for (const entry of state.summary?.lessons ?? []) {
        set.add(lessonKey(entry.trackId, entry.moduleId, entry.lessonId))
      }
      return set
    },
    isLessonCompleted(): (trackId: string, moduleId: string, lessonId: string) => boolean {
      return (trackId, moduleId, lessonId) =>
        this.completedKeys.has(lessonKey(trackId, moduleId, lessonId))
    },
    moduleState(): (moduleId: string) => ModuleState | undefined {
      return (moduleId) => this.summary?.modules.find((m) => m.moduleId === moduleId)
    },
    isModuleLocked(): (moduleId: string) => boolean {
      return (moduleId) => this.moduleState(moduleId)?.locked ?? false
    },
    isModuleCompleted(): (moduleId: string) => boolean {
      return (moduleId) => this.moduleState(moduleId)?.completed ?? false
    },
    trackPercent(): (trackId: string) => number {
      return (trackId) => this.summary?.trackPercent[trackId] ?? 0
    },
    capstoneUnlocked(state): boolean {
      return state.summary?.capstoneUnlocked ?? false
    },
    nextUp(state) {
      return state.summary?.next ?? null
    },
    recentCompletions(state) {
      return [...(state.summary?.lessons ?? [])].slice(0, 8)
    },
    /** Rich per-lesson progress keyed by composite lesson key. */
    lessonProgressMap(state): Map<string, import('@/types/domain').LessonProgress> {
      const map = new Map<string, import('@/types/domain').LessonProgress>()
      for (const entry of state.summary?.lessonProgress ?? []) {
        map.set(lessonKey(entry.trackId, entry.moduleId, entry.lessonId), entry)
      }
      return map
    },
    lessonProgress():
      (trackId: string, moduleId: string, lessonId: string) =>
        import('@/types/domain').LessonProgress | undefined {
      return (trackId, moduleId, lessonId) =>
        this.lessonProgressMap.get(lessonKey(trackId, moduleId, lessonId))
    },
    skillStates(state): import('@/types/domain').SkillState[] {
      return state.summary?.skills ?? []
    },
    recommendations(state): import('@/types/domain').Recommendation[] {
      return state.summary?.recommendations ?? []
    }
  },

  actions: {
    applySummary(summary: ProgressSummary) {
      this.summary = summary
    },

    async complete(trackId: string, moduleId: string, lessonId: string) {
      this.applySummary(await ipc.completeLesson(trackId, moduleId, lessonId))
    },

    async uncomplete(trackId: string, moduleId: string, lessonId: string) {
      this.applySummary(await ipc.uncompleteLesson(trackId, moduleId, lessonId))
    },

    async toggleLesson(trackId: string, moduleId: string, lessonId: string) {
      if (this.isLessonCompleted(trackId, moduleId, lessonId)) {
        await this.uncomplete(trackId, moduleId, lessonId)
      } else {
        await this.complete(trackId, moduleId, lessonId)
      }
    },

    async recordQuiz(
      trackId: string,
      moduleId: string,
      lessonId: string,
      quizId: string,
      score: number,
      total: number
    ) {
      await ipc.recordQuizAttempt(trackId, moduleId, lessonId, quizId, score, total)
    },

    async recordExercise(
      trackId: string,
      moduleId: string,
      lessonId: string,
      exerciseId: string,
      status: 'tried' | 'solved',
      solutionRevealed: boolean
    ) {
      await ipc.recordExerciseAttempt(
        trackId,
        moduleId,
        lessonId,
        exerciseId,
        status,
        solutionRevealed
      )
    }
  }
})
