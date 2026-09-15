import { defineStore } from 'pinia'

import { useLessonsStore } from './lessons.store'
import { useProgressStore } from './progress.store'

/**
 * Read-model over the declarative skill graph: joins catalog graph data with
 * runtime skill states from the progress summary. Derivation only — no
 * duplicate source of truth lives here.
 */
export const useSkillStore = defineStore('skills', {
  getters: {
    graph(): import('@/types/domain').SkillGraph | null {
      return useLessonsStore().skillGraph
    },
    states(): import('@/types/domain').SkillState[] {
      return useProgressStore().skillStates
    },
    /** Stable display order: dependency depth, then label. */
    sortedSkills(): import('@/types/domain').SkillState[] {
      const depth = new Map<string, number>()
      const nodeById = new Map(this.graph?.skills.map((s) => [s.id, s]) ?? [])
      const depthOf = (id: string, guard: Set<string>): number => {
        if (depth.has(id)) return depth.get(id) as number
        if (guard.has(id)) return 0
        guard.add(id)
        const node = nodeById.get(id)
        const value =
          node && node.requires.length > 0
            ? 1 + Math.max(...node.requires.map((r) => depthOf(r, guard)))
            : 0
        depth.set(id, value)
        return value
      }
      return [...this.states]
        .map((state) => ({ state, depth: depthOf(state.id, new Set()) }))
        .sort((a, b) => a.depth - b.depth || a.state.label.localeCompare(b.state.label))
        .map((entry) => entry.state)
    },
    demonstratedIds(): Set<string> {
      return new Set(this.states.filter((s) => s.demonstrated).map((s) => s.id))
    },
    demonstratedCount(): number {
      return this.demonstratedIds.size
    },
    totalCount(): number {
      return this.states.length
    }
  }
})
