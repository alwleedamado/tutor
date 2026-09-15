/**
 * Shared DTO contract mirrored by serde structs in src-tauri (camelCase).
 * This file is the single source of truth for the IPC boundary shape.
 *
 * Track ids are open-ended strings: the catalog is a declarative registry
 * (core tracks, foundations, specialized tracks) and the UI never hardcodes
 * membership — always resolve tracks through the catalog.
 */
export type TrackId = string

export interface QuizQuestion {
  id: string
  prompt: string
  options: string[]
  answerIndex: number
  explanation?: string
  code?: string
}

export interface Quiz {
  id: string
  passScore: number
  questions: QuizQuestion[]
}

export interface Exercise {
  id: string
  prompt: string
  kind: 'implementation' | 'bug-hunt' | string
  starterCode: string
  language: string
  hints: string[]
  solution: string
}

export interface CodeSmell {
  name: string
  problem: string
  why: string
  badCode: string
  analysis: string
  refactor: string
  goodCode: string
}

export interface ExampleFile {
  name: string
  language: string
  code: string
}

export interface Lesson {
  id: string
  moduleId: string
  trackId: TrackId
  slug: string
  index: number
  title: string
  minutes: number
  kind: string
  objectives: string[]
  smells: CodeSmell[]
  examples: ExampleFile[]
  exercise: Exercise | null
  quiz: Quiz | null
  /** Skill ids (from the skill graph) this lesson provides evidence for. */
  skills: string[]
  /** Informational cross-links to related module ids. */
  related: string[]
  markdown: string
}

export interface Module {
  id: string
  trackId: TrackId
  slug: string
  index: number
  title: string
  summary: string
  prerequisites: string[]
  /** "beginner" | "intermediate" | "advanced". */
  difficulty: string
  /** Skills demonstrated by completing the whole module. */
  skills: string[]
  lessons: Lesson[]
}

export interface Track {
  id: TrackId
  title: string
  description: string
  /** Dashboard grouping: Foundations, Languages, Frontend, Desktop, … */
  category: string
  modules: Module[]
}

export interface Catalog {
  version: string
  tracks: Track[]
}

export interface ProjectMilestone {
  id: string
  title: string
  detail: string
}

export interface ProjectSpec {
  id: string
  title: string
  trackId?: string | null
  moduleId?: string | null
  summary: string
  requirements: string[]
  milestones: ProjectMilestone[]
  starterFiles: ExampleFile[]
  hints: string[]
  completionCriteria: string[]
  estimatedHours: number
}

export interface CapstoneSpec {
  id: string
  title: string
  summary: string
  prerequisites: string[]
  requirements: string[]
  features: string[]
}

// ---------- Progress ----------

export interface CompletedLesson {
  trackId: TrackId
  moduleId: string
  lessonId: string
  completedAt: number
}

export interface ModuleState {
  moduleId: string
  total: number
  done: number
  completed: boolean
  locked: boolean
}

export interface NextUp {
  trackId: TrackId
  moduleId: string
  lessonSlug: string
}

// ---------- Skill graph & rich progress ----------

/** Static node from content/skills/skills.json. */
export interface SkillNode {
  id: string
  label: string
  category: string
  summary: string
  requires: string[]
}

export interface SkillGraph {
  version: number
  skills: SkillNode[]
}

/** Runtime skill state derived from (graph, catalog, progress). */
export interface SkillState {
  id: string
  label: string
  category: string
  demonstrated: boolean
  lessonsTotal: number
  lessonsDone: number
  dependenciesMet: boolean
  missingDependencies: string[]
}

/** Per-lesson progress state: not-started | attempted | practicing | completed | mastered. */
export interface LessonProgress {
  trackId: TrackId
  moduleId: string
  lessonId: string
  state: string
  attempts: number
  bestQuizPercent: number | null
  quizPassed: boolean
  solvedWithoutSolution: boolean
}

/** Deterministic, explainable learning recommendation. */
export interface Recommendation {
  kind: 'next-lesson' | 'prerequisite' | 'review' | 'project' | 'skill-gap' | string
  title: string
  reason: string
  trackId?: string | null
  moduleId?: string | null
  lessonSlug?: string | null
  projectId?: string | null
  skillId?: string | null
}

export interface ProgressSummary {
  lessons: CompletedLesson[]
  modules: ModuleState[]
  trackPercent: Record<string, number>
  capstoneUnlocked: boolean
  next: NextUp | null
  lessonProgress?: LessonProgress[]
  skills?: SkillState[]
  recommendations?: Recommendation[]
}

// ---------- Settings / editor / library ----------

export interface Settings {
  theme: 'dark' | 'light' | 'system'
  fontSize: number
  tabSize: number
  fontLigatures: boolean
  reducedMotion: boolean
  highContrast: boolean
}

export const DEFAULT_SETTINGS: Settings = {
  theme: 'dark',
  fontSize: 14,
  tabSize: 4,
  fontLigatures: true,
  reducedMotion: false,
  highContrast: false
}

export interface EditorStateDto {
  scopeId: string
  language: string
  code: string
  cursorOffset: number
}

export interface NoteDto {
  id: number
  scopeType: 'lesson' | 'project' | 'general' | string
  scopeId: string
  title: string
  body: string
  createdAt: number
  updatedAt: number
}

export interface NoteInput {
  id?: number | null
  scopeType: 'lesson' | 'project' | 'general' | string
  scopeId: string
  title?: string
  body?: string
}

export interface BookmarkDto {

  itemType: 'lesson' | 'project' | string
  itemId: string
  createdAt: number
}

export interface ProjectStateDto {
  projectId: string
  milestones: string[]
  completed: boolean
  updatedAt: number
}

// ---------- Execution ----------

export interface Diagnostic {
  severity: 'error' | 'warning' | 'note' | 'help' | string
  message: string
  span?: string
}

export interface RunResult {
  executionId: string
  ok: boolean
  timedOut: boolean
  cancelled: boolean
  durationMs: number
  stdout: string
  stderr: string
  diagnostics: Diagnostic[]
}

export interface TestResult extends RunResult {
  passed: number
  failed: number
}

export interface SystemInfo {
  os: string
  arch: string
  appVersion: string
  rustcVersion: string | null
  cargoVersion: string | null
  dbPath: string
}

export interface Bootstrap {
  catalog: Catalog
  projects: ProjectSpec[]
  capstone: CapstoneSpec
  progress: ProgressSummary
  settings: Settings
  skillGraph: SkillGraph | null
}

/** Structured error crossing the IPC boundary ({ kind, message }). */
export interface AppErrorDto {
  kind: 'database' | 'content' | 'validation' | 'execution' | 'notFound' | 'io' | string
  message: string
}

/** Composite lesson key used by stores, routes and persistence. */
export function lessonKey(trackId: string, moduleId: string, lessonId: string): string {
  return `${trackId}:${moduleId}:${lessonId}`
}
