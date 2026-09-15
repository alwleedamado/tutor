import { vi } from 'vitest'

/** Spy at the IPC seam so the REAL progress store runs inside components. */
export const recordQuizAttempt = vi.fn<(track: string, moduleId: string, lessonId: string, quizId: string, score: number, total: number) => Promise<void>>()
recordQuizAttempt.mockResolvedValue(undefined)

vi.mock('@/services/ipc', () => ({
  ipc: {
    recordQuizAttempt: (...args: unknown[]) =>
      (recordQuizAttempt as unknown as (...a: unknown[]) => Promise<void>)(...args)
  }
}))

