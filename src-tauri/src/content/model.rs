use serde::{Deserialize, Serialize};

fn default_pass_score() -> u32 {
    70
}
fn default_minutes() -> u32 {
    25
}
fn default_kind() -> String {
    "concept".into()
}
fn default_exercise_kind() -> String {
    "implementation".into()
}

// ---------- Lesson activity bundle ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestion {
    pub id: String,
    pub prompt: String,
    pub options: Vec<String>,
    pub answer_index: usize,
    #[serde(default)]
    pub explanation: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quiz {
    pub id: String,
    #[serde(default = "default_pass_score")]
    pub pass_score: u32,
    pub questions: Vec<QuizQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exercise {
    pub id: String,
    pub prompt: String,
    #[serde(default = "default_exercise_kind")]
    pub kind: String,
    pub starter_code: String,
    pub language: String,
    #[serde(default)]
    pub hints: Vec<String>,
    pub solution: String,
}

/// Code-smell teaching entry: Problem → Why → Bad → Analysis → Refactor → Good.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSmell {
    pub name: String,
    pub problem: String,
    pub why: String,
    pub bad_code: String,
    pub analysis: String,
    pub refactor: String,
    pub good_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleFile {
    pub name: String,
    pub language: String,
    pub code: String,
}

// ---------- Lesson metadata + runtime catalog ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonMeta {
    pub id: String,
    pub title: String,
    #[serde(default = "default_minutes")]
    pub minutes: u32,
    #[serde(default = "default_kind")]
    pub kind: String,
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default)]
    pub smells: Vec<CodeSmell>,
    #[serde(default)]
    pub examples: Vec<ExampleFile>,
    #[serde(default)]
    pub exercise: Option<Exercise>,
    #[serde(default)]
    pub quiz: Option<Quiz>,
    /// Optional inline lesson body (markdown). A sibling `lesson.md`, when
    /// present, takes precedence — see docs/content-architecture.md.
    #[serde(default)]
    pub body: String,
    /// Skill ids (from the skill graph) this lesson provides evidence for.
    #[serde(default)]
    pub skills: Vec<String>,
    /// Informational cross-links to related module ids in any track.
    #[serde(default)]
    pub related: Vec<String>,
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    pub id: String,
    pub module_id: String,
    pub track_id: String,
    pub slug: String,
    pub index: u32,
    pub title: String,
    pub minutes: u32,
    pub kind: String,
    pub objectives: Vec<String>,
    pub smells: Vec<CodeSmell>,
    pub examples: Vec<ExampleFile>,
    pub exercise: Option<Exercise>,
    pub quiz: Option<Quiz>,
    /// Skill ids (from the skill graph) this lesson provides evidence for.
    #[serde(default)]
    pub skills: Vec<String>,
    /// Informational cross-links to related module ids in any track.
    #[serde(default)]
    pub related: Vec<String>,
    pub markdown: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonRef {
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDef {
    pub id: String,
    pub slug: String,
    pub index: u32,
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub lessons: Vec<LessonRef>,
    /// "beginner" | "intermediate" | "advanced" — display/UX metadata only.
    #[serde(default = "default_difficulty")]
    pub difficulty: String,
    /// Skill ids completed by finishing every lesson in this module.
    #[serde(default)]
    pub skills: Vec<String>,
}

fn default_difficulty() -> String {
    "intermediate".into()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub id: String,
    pub track_id: String,
    pub slug: String,
    pub index: u32,
    pub title: String,
    pub summary: String,
    pub prerequisites: Vec<String>,
    pub difficulty: String,
    pub skills: Vec<String>,
    pub lessons: Vec<Lesson>,
}

impl Module {
    /// Total lesson count; used by the engine to derive completion.
    pub fn lesson_count(&self) -> usize {
        self.lessons.len()
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: String,
    pub title: String,
    pub description: String,
    /// Dashboard grouping: "Foundations", "Languages", "Frontend", "Desktop",
    /// "Backend", "Architecture", "Security", "Performance", "Quality",
    /// "Operations".
    pub category: String,
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub version: String,
    pub tracks: Vec<Track>,
}

// ---------- Projects ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMilestone {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSpec {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub track_id: Option<String>,
    #[serde(default)]
    pub module_id: Option<String>,
    pub summary: String,
    #[serde(default)]
    pub requirements: Vec<String>,
    #[serde(default)]
    pub milestones: Vec<ProjectMilestone>,
    #[serde(default)]
    pub starter_files: Vec<ExampleFile>,
    #[serde(default)]
    pub hints: Vec<String>,
    #[serde(default)]
    pub completion_criteria: Vec<String>,
    #[serde(default)]
    pub estimated_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapstoneSpec {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub requirements: Vec<String>,
    #[serde(default)]
    pub features: Vec<String>,
}

// ---------- Progress / persistence DTOs ----------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedLesson {
    pub track_id: String,
    pub module_id: String,
    pub lesson_id: String,
    pub completed_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleState {
    pub module_id: String,
    pub total: u32,
    pub done: u32,
    pub completed: bool,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NextUp {
    pub track_id: String,
    pub module_id: String,
    pub lesson_slug: String,
}

// ---------- Skill graph & recommendations ----------

/// Static node of the declarative skill graph (`content/skills/skills.json`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillNode {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub summary: String,
    /// Skill ids that must be demonstrated before this one is reachable.
    #[serde(default)]
    pub requires: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillGraph {
    pub version: u32,
    pub skills: Vec<SkillNode>,
}

/// Runtime skill state derived from (skill graph, catalog, progress).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillState {
    pub id: String,
    pub label: String,
    pub category: String,
    /// All catalog evidence (lessons + modules granting this skill) complete.
    pub demonstrated: bool,
    pub lessons_total: u32,
    pub lessons_done: u32,
    pub modules_total: u32,
    pub modules_done: u32,
    pub dependencies_met: bool,
    pub missing_dependencies: Vec<String>,
}

/// Aggregated quiz evidence for one lesson.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizStat {
    pub track_id: String,
    pub module_id: String,
    pub lesson_id: String,
    pub attempts: u32,
    pub best_percent: u32,
    pub passed: bool,
}

/// Aggregated exercise evidence for one lesson.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseStat {
    pub track_id: String,
    pub module_id: String,
    pub lesson_id: String,
    pub attempts: u32,
    pub solved: bool,
    pub solution_revealed: bool,
}

/// Per-lesson progress state (richer than a boolean completion).
/// States: not-started | attempted | practicing | completed | mastered.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonProgress {
    pub track_id: String,
    pub module_id: String,
    pub lesson_id: String,
    pub state: String,
    pub attempts: u32,
    pub best_quiz_percent: Option<u32>,
    pub quiz_passed: bool,
    pub solved_without_solution: bool,
}

/// Deterministic, explainable learning recommendation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    /// "next-lesson" | "prerequisite" | "review" | "project" | "skill-gap"
    pub kind: String,
    pub title: String,
    /// Human-readable explanation of WHY this is recommended (deterministic).
    pub reason: String,
    #[serde(default)]
    pub track_id: Option<String>,
    #[serde(default)]
    pub module_id: Option<String>,
    #[serde(default)]
    pub lesson_slug: Option<String>,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub skill_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressSummary {
    pub lessons: Vec<CompletedLesson>,
    pub modules: Vec<ModuleState>,
    pub track_percent: std::collections::BTreeMap<String, u32>,
    pub capstone_unlocked: bool,
    pub next: Option<NextUp>,
    #[serde(default)]
    pub lesson_progress: Vec<LessonProgress>,
    #[serde(default)]
    pub skills: Vec<SkillState>,
    #[serde(default)]
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsDto {
    pub theme: String,
    pub font_size: u32,
    pub tab_size: u32,
    pub font_ligatures: bool,
    pub reduced_motion: bool,
    pub high_contrast: bool,
}

impl Default for SettingsDto {
    fn default() -> Self {
        Self {
            theme: "dark".into(),
            font_size: 14,
            tab_size: 4,
            font_ligatures: true,
            reduced_motion: false,
            high_contrast: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorStateDto {
    pub scope_id: String,
    pub language: String,
    pub code: String,
    pub cursor_offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteInput {
    #[serde(default)]
    pub id: Option<i64>,
    pub scope_type: String,
    pub scope_id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteDto {
    pub id: i64,
    pub scope_type: String,
    pub scope_id: String,
    pub title: String,
    pub body: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkDto {
    pub item_type: String,
    pub item_id: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStateDto {
    pub project_id: String,
    pub milestones: Vec<String>,
    pub completed: bool,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bootstrap {
    pub catalog: Catalog,
    pub projects: Vec<ProjectSpec>,
    pub capstone: CapstoneSpec,
    pub progress: ProgressSummary,
    pub settings: SettingsDto,
    /// Declarative skill graph (empty when skills/skills.json is absent).
    #[serde(default)]
    pub skill_graph: Option<SkillGraph>,
}


