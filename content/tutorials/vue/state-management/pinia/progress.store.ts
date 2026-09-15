// REFERENCE IMPLEMENTATION — the pattern this application ships.
// stores/progress.store.ts follows exactly this shape (options API store).
import { defineStore } from 'pinia'

export interface ProgressState {
  done: string[]
  total: number
}

export const useProgressStore = defineStore('tutorial-progress', {
  state: (): ProgressState => ({
    done: [],
    total: 10
  }),

  getters: {
    percent(state): number {
      return state.total === 0 ? 0 : Math.round((state.done.length * 100) / state.total)
    },
    isDone(state) {
      return (lessonId: string): boolean => state.done.includes(lessonId)
    }
  },

  actions: {
    mark(lessonId: string): void {
      if (!this.done.includes(lessonId)) this.done.push(lessonId)
    },
    unmark(lessonId: string): void {
      this.done = this.done.filter((id) => id !== lessonId)
    }
  }
})

// Component usage — no mapping helpers, fully typed:
//   const progress = useProgressStore()
//   progress.percent
//   progress.mark('rust-01-l01')
