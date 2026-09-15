import { defineStore } from 'pinia'

import { ipc } from '@/services/ipc'
import type { ProjectSpec, ProjectStateDto } from '@/types/domain'
import { useLessonsStore } from './lessons.store'

interface ProjectState {
  specs: ProjectSpec[]
  states: Record<string, ProjectStateDto>
  loaded: boolean
}

/** Owns project specifications and their persisted milestone state. */
export const useProjectStore = defineStore('project', {
  state: (): ProjectState => ({
    specs: [],
    states: {},
    loaded: false
  }),

  getters: {
    specById(state): (id: string) => ProjectSpec | undefined {
      return (id) =>
        state.specs.find((p) => p.id === id) ?? useLessonsStore().projectById(id)
    },
    milestonesDone(): (id: string) => Set<string> {
      return (id) => new Set(this.states[id]?.milestones ?? [])
    },
    isCompleted(state): (id: string) => boolean {
      return (id) => state.states[id]?.completed ?? false
    },
    overallProgress(): (id: string, totalMilestones: number) => number {
      return (id, totalMilestones) => {
        if (totalMilestones === 0) return this.isCompleted(id) ? 100 : 0
        return Math.round((this.milestonesDone(id).size * 100) / totalMilestones)
      }
    }
  },

  actions: {
    hydrate(specs: ProjectSpec[]) {
      this.specs = specs
    },

    async ensureStates() {
      if (this.loaded) return
      for (const dto of await ipc.getProjectStates()) {
        this.states[dto.projectId] = dto
      }
      this.loaded = true
    },

    async toggleMilestone(project: ProjectSpec, milestoneId: string) {
      const done = new Set(this.milestonesDone(project.id))
      if (done.has(milestoneId)) done.delete(milestoneId)
      else done.add(milestoneId)

      const allDone =
        project.milestones.length > 0 &&
        project.milestones.every((m) => done.has(m.id))

      await this.persist(project.id, [...done], allDone)
    },

    async setCompleted(projectId: string, completed: boolean) {
      const done = [...this.milestonesDone(projectId)]
      await this.persist(projectId, done, completed)
    },

    async persist(projectId: string, milestones: string[], completed: boolean) {

      const dto: ProjectStateDto = {
        projectId,
        milestones,
        completed,
        updatedAt: Math.floor(Date.now() / 1000)
      }
      await ipc.saveProjectState(dto)
      this.states[projectId] = dto
    }
  }
})
