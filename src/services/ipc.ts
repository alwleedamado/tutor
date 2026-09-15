/**
 * Typed IPC client — the ONLY file allowed to touch @tauri-apps/api.
 * Stores depend on this seam; tests mock this module wholesale.
 */
import { invoke } from '@tauri-apps/api/core'

import type {
  AppErrorDto,
  BookmarkDto,
  Bootstrap,
  EditorStateDto,
  NoteDto,
  NoteInput,
  ProgressSummary,
  ProjectStateDto,
  RunResult,
  Settings,
  SystemInfo,
  TestResult
} from '@/types/domain'

/** Reified structured error from the Rust backend ({ kind, message }). */
export class IpcError extends Error {
  readonly kind: string
  constructor(dto: AppErrorDto) {
    super(dto.message)
    this.name = 'IpcError'
    this.kind = dto.kind
  }
}

/**
 * True when running inside the Tauri webview. Plain-browser previews have no
 * Rust process attached — invoke() would reject on everything.
 */
export function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}


async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args)
  } catch (raw) {
    if (
      raw &&
      typeof raw === 'object' &&
      'kind' in raw &&
      'message' in raw &&
      typeof (raw as AppErrorDto).message === 'string'
    ) {
      throw new IpcError(raw as AppErrorDto)
    }
    throw raw
  }
}

export const ipc = {
  // bootstrap
  getBootstrap: () => call<Bootstrap>('get_bootstrap'),

  // progress
  completeLesson: (trackId: string, moduleId: string, lessonId: string) =>
    call<ProgressSummary>('complete_lesson', { trackId, moduleId, lessonId }),
  uncompleteLesson: (trackId: string, moduleId: string, lessonId: string) =>
    call<ProgressSummary>('uncomplete_lesson', { trackId, moduleId, lessonId }),
  recordQuizAttempt: (
    trackId: string,
    moduleId: string,
    lessonId: string,
    quizId: string,
    score: number,
    total: number
  ) => call<void>('record_quiz_attempt', { trackId, moduleId, lessonId, quizId, score, total }),
  recordExerciseAttempt: (
    trackId: string,
    moduleId: string,
    lessonId: string,
    exerciseId: string,
    status: 'tried' | 'solved',
    solutionRevealed: boolean
  ) =>
    call<void>('record_exercise_attempt', {
      trackId,
      moduleId,
      lessonId,
      exerciseId,
      status,
      solutionRevealed
    }),

  // notes & bookmarks
  saveNote: (note: NoteInput) => call<NoteDto>('save_note', { note }),
  deleteNote: (id: number) => call<boolean>('delete_note', { id }),
  listNotes: (scopeType?: string, scopeId?: string) =>
    call<NoteDto[]>('list_notes', { scopeType, scopeId }),
  toggleBookmark: (itemType: string, itemId: string) =>
    call<boolean>('toggle_bookmark', { itemType, itemId }),
  listBookmarks: () => call<BookmarkDto[]>('list_bookmarks'),

  // settings & editor state
  saveSettings: (settings: Settings) => call<void>('save_settings', { settings }),
  saveEditorState: (state: EditorStateDto) =>
    call<void>('save_editor_state', {
      scopeId: state.scopeId,
      language: state.language,
      code: state.code,
      cursorOffset: state.cursorOffset
    }),
  getEditorState: (scopeId: string) => call<EditorStateDto | null>('get_editor_state', { scopeId }),

  // projects
  saveProjectState: (project: ProjectStateDto) =>
    call<void>('save_project_state', { project }),
  getProjectStates: () => call<ProjectStateDto[]>('get_project_states'),

  // execution
  runRustCode: (code: string) => call<RunResult>('run_rust_code', { code }),
  cancelExecution: (executionId: string) => call<boolean>('cancel_execution', { executionId }),
  runCargoTest: (path: string) => call<TestResult>('run_cargo_test', { path }),
  getSystemInfo: () => call<SystemInfo>('get_system_info'),

  // system
  resetAllData: () => call<void>('reset_all_data')
}

export type IpcClient = typeof ipc
