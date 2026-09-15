import { defineStore } from 'pinia'

import { ipc } from '@/services/ipc'
import type { RunResult, TestResult } from '@/types/domain'

export type ExecutionStatus = 'idle' | 'running' | 'done' | 'error'

interface ExecutionState {
  status: ExecutionStatus
  result: RunResult | null
  testResult: TestResult | null
  activeExecutionId: string | null
  errorMessage: string | null
}

/**
 * Owns the compile/run lifecycle. The backend performs the full pipeline
 * (compile → run → collect) behind one IPC call; `running` covers both.
 */
export const useExecutionStore = defineStore('execution', {
  state: (): ExecutionState => ({
    status: 'idle',
    result: null,
    testResult: null,
    activeExecutionId: null,
    errorMessage: null
  }),

  getters: {
    isBusy(state): boolean {
      return state.status === 'running'
    },
    hasDiagnostics(state): boolean {
      return (state.result?.diagnostics.length ?? 0) > 0
    }
  },

  actions: {
    async run(code: string) {
      if (this.status === 'running') return
      this.status = 'running'
      this.errorMessage = null
      this.testResult = null
      try {
        const result = await ipc.runRustCode(code)
        this.result = result
        this.activeExecutionId = result.executionId
        this.status = 'done'
      } catch (error) {
        this.status = 'error'
        this.errorMessage = error instanceof Error ? error.message : String(error)
      }
    },

    async cancel() {
      if (!this.activeExecutionId) return
      await ipc.cancelExecution(this.activeExecutionId).catch(() => false)
    },

    /** Run `cargo test --offline` for a managed workspace path. */
    async runProjectTests(path: string) {
      if (this.status === 'running') return
      this.status = 'running'
      this.errorMessage = null
      this.result = null
      try {
        const result = await ipc.runCargoTest(path)
        this.testResult = result
        this.activeExecutionId = result.executionId
        this.status = 'done'
      } catch (error) {
        this.status = 'error'
        this.errorMessage = error instanceof Error ? error.message : String(error)
      }
    },

    reset() {
      this.status = 'idle'
      this.result = null
      this.testResult = null
      this.activeExecutionId = null
      this.errorMessage = null
    }
  }
})
