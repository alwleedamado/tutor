import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'

import { DEFAULT_SETTINGS, lessonKey, type TrackId } from '@/types/domain'
import type { Bootstrap, ProgressSummary } from '@/types/domain'


const completeLesson = vi.fn()
const uncompleteLesson = vi.fn()

vi.mock('@/services/ipc', () => ({
  ipc: {
    getBootstrap: vi.fn(),
    completeLesson: (...args: unknown[]) => completeLesson(...args),
    uncompleteLesson: (...args: unknown[]) => uncompleteLesson(...args),
    saveSettings: vi.fn()
  }
}))

import { useProgressStore } from '../progress.store'


function summaryWith(completedKeys: string[]): ProgressSummary {
  const lessons = completedKeys.map((key) => {
    const [trackId, moduleId, lessonId] = key.split(':')
    return {
      trackId: trackId as TrackId,
      moduleId,
      lessonId,
      completedAt: 0
    }
  })

  const modules = [
    { moduleId: 'rust-01', total: 1, done: lessons.some((l) => l.moduleId === 'rust-01') ? 1 : 0, completed: false, locked: false },
    { moduleId: 'tauri-01', total: 1, done: 0, completed: false, locked: !completedKeys.includes('rust:rust-01:rust-01-l01') }
  ]
  return {
    lessons,
    modules,
    trackPercent: { rust: lessons.length * 100 },
    capstoneUnlocked: false,
    next: null
  }
}

describe('progress store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    completeLesson.mockReset().mockResolvedValue(summaryWith(['rust:rust-01:rust-01-l01']))
    uncompleteLesson.mockReset().mockResolvedValue(summaryWith([]))
  })

  it('completing a lesson updates completion and lock derivation', async () => {
    const store = useProgressStore()
    store.applySummary(summaryWith([]))

    expect(store.isLessonCompleted('rust', 'rust-01', 'rust-01-l01')).toBe(false)
    expect(store.isModuleLocked('tauri-01')).toBe(true)

    await store.complete('rust', 'rust-01', 'rust-01-l01')

    expect(completeLesson).toHaveBeenCalledWith('rust', 'rust-01', 'rust-01-l01')
    expect(store.isLessonCompleted('rust', 'rust-01', 'rust-01-l01')).toBe(true)
    expect(store.isModuleLocked('tauri-01')).toBe(false)
  })

  it('uncompleting restores the lock', async () => {
    const store = useProgressStore()
    store.applySummary(summaryWith(['rust:rust-01:rust-01-l01']))

    await store.uncomplete('rust', 'rust-01', 'rust-01-l01')

    expect(store.isLessonCompleted('rust', 'rust-01', 'rust-01-l01')).toBe(false)
    expect(store.isModuleLocked('tauri-01')).toBe(true)
  })

  it('lesson keys are stable composites', () => {
    expect(lessonKey('vue', 'vue-02', 'l1')).toBe('vue:vue-02:l1')
  })

  it('settings defaults survive hydration gaps', () => {
    expect(DEFAULT_SETTINGS.theme).toBe('dark')
  })
})

describe('bootstrap payload shape', () => {
  it('accepts a well-formed bootstrap object', async () => {
    const boot: Bootstrap = {
      catalog: { version: 't', tracks: [] },
      projects: [],
      capstone: {
        id: 'capstone-rust-playground',
        title: 'Rust Playground Offline',
        summary: '',
        prerequisites: ['rust-06', 'vue-04', 'tauri-03'],
        requirements: [],
        features: []
      },
      progress: summaryWith([]),
      settings: DEFAULT_SETTINGS,
      skillGraph: null
    }
    const store = useProgressStore()
    store.applySummary(boot.progress)
    expect(boot.capstone.prerequisites).toHaveLength(3)
    expect(store.summary?.capstoneUnlocked).toBe(false)
    expect(store.summary).toStrictEqual(boot.progress)

  })
})
