pub mod model;

use std::collections::HashSet;


use include_dir::{include_dir, Dir};
use serde::de::DeserializeOwned;

pub use model::*;

use crate::error::{AppError, AppResult};

/// The entire curriculum is compiled into the binary — the offline guarantee
/// is structural, not conventional. See docs/content-architecture.md.
pub static CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../content");

/// Declarative track registry. The catalog is derived from this table —
/// adding a track means adding content plus one row here, never engine code.
/// Order defines default recommendation order: core tracks first, then
/// foundations, then specialized tracks.
pub struct TrackDecl {
    pub id: &'static str,
    /// Directory under `content/` holding the track's modules. May contain
    /// slashes for grouped domains (`web/performance`).
    pub dir: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    /// Dashboard grouping category.
    pub category: &'static str,
}

pub static TRACKS: &[TrackDecl] = &[
    // ---- Core tracks (original curriculum) --------------------------------
    TrackDecl {
        id: "rust",
        dir: "rust",
        title: "Rust Mastery",
        description: "From fundamentals and ownership to async networking and advanced Rust.",
        category: "Languages",
    },
    TrackDecl {
        id: "vue",
        dir: "vue",
        title: "Vue 3 Mastery",
        description: "Modern Composition API, Pinia-first state management, routing and tooling.",
        category: "Frontend",
    },
    TrackDecl {
        id: "tauri",
        dir: "tauri",
        title: "Tauri Desktop Development",
        description: "Build secure, offline-capable desktop apps with Tauri v2 and Rust.",
        category: "Desktop",
    },
    // ---- Shared engineering foundations -----------------------------------
    TrackDecl {
        id: "http",
        dir: "web/http",
        title: "HTTP & Web Platform",
        description: "DNS, TCP, TLS, HTTP semantics, caching, cookies, CORS and CSP — the shared substrate for every web track.",
        category: "Foundations",
    },
    TrackDecl {
        id: "architecture",
        dir: "architecture",
        title: "Software Architecture & Design",
        description: "Coupling, cohesion, boundaries, SOLID, ADRs and evolution — taught across Rust, Vue, Node and Tauri examples.",
        category: "Architecture",
    },
    TrackDecl {
        id: "testing",
        dir: "testing",
        title: "Testing & Quality Engineering",
        description: "Test levels, doubles, isolation and strategy across Rust, Vue/Vitest and API testing.",
        category: "Quality",
    },
    TrackDecl {
        id: "git",
        dir: "git",
        title: "Git & Professional Workflow",
        description: "Branches, rebase, conflict resolution, review and release workflow as daily engineering practice.",
        category: "Foundations",
    },
    TrackDecl {
        id: "api-design",
        dir: "api-design",
        title: "API Design & Integration",
        description: "Resource modeling, HTTP semantics, OpenAPI contracts, versioning and API evolution — design before you build.",
        category: "Foundations",
    },
    TrackDecl {
        id: "databases",
        dir: "databases",
        title: "Database Engineering Fundamentals",
        description: "Relational modeling, SQL, transactions, indexes, isolation and connection pooling beyond basic CRUD.",
        category: "Foundations",
    },
    TrackDecl {
        id: "linux",
        dir: "linux-tooling",
        title: "Linux & Developer Tooling",
        description: "Shell, processes, signals, ports, DNS and HTTP debugging — diagnose dev and deployment problems independently.",
        category: "Operations",
    },
    TrackDecl {
        id: "devops",
        dir: "devops",
        title: "DevOps, CI/CD & Containers",
        description: "Docker, images, compose, pipelines and environment configuration: from code to deploy to observe.",
        category: "Operations",
    },
    TrackDecl {
        id: "observability",
        dir: "observability",
        title: "Observability & Production Operations",
        description: "Structured logs, metrics, traces, correlation IDs and health/readiness for systems you can operate.",
        category: "Operations",
    },
    // ---- Specialized tracks ------------------------------------------------
    TrackDecl {
        id: "vue-architecture",
        dir: "vue-architecture",
        title: "Vue Architecture",
        description: "Design maintainable Vue 3 applications: features, boundaries, state architecture and evolution.",
        category: "Architecture",
    },
    TrackDecl {
        id: "web-performance",
        dir: "web/performance",
        title: "Web Performance Engineering",
        description: "Rendering pipeline, network, bundle and runtime performance — measurement before optimization.",
        category: "Performance",
    },
    TrackDecl {
        id: "web-security",
        dir: "web/security",
        title: "Web Security",
        description: "Threat modeling, browser and application security, auth, and defensive engineering practice.",
        category: "Security",
    },
    TrackDecl {
        id: "web-accessibility",
        dir: "web/accessibility",
        title: "Web Accessibility Engineering",
        description: "WCAG, semantic HTML, ARIA discipline, accessible Vue components and a11y testing as normal engineering.",
        category: "Quality",
    },
    TrackDecl {
        id: "backend-rust",
        dir: "backend/rust",
        title: "Backend Rust Development",
        description: "Layered async Rust services: APIs, databases, auth and production operations.",
        category: "Backend",
    },
    TrackDecl {
        id: "backend-express",
        dir: "backend/node-express",
        title: "Production Node.js & Express",
        description: "Node runtime fundamentals and Express engineering at production quality — not just framework syntax.",
        category: "Backend",
    },
];

pub fn track_decl(id: &str) -> Option<&'static TrackDecl> {
    TRACKS.iter().find(|t| t.id == id)
}

fn normalize(path: &std::path::Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Recursive file walk — this include_dir version exposes shallow accessors,
/// so we build our own flat view once per query.
fn all_files() -> Vec<&'static include_dir::File<'static>> {
    fn walk<'a>(
        dir: &'a include_dir::Dir<'a>,
        out: &mut Vec<&'a include_dir::File<'a>>,
    ) {
        out.extend(dir.files());
        for sub in dir.dirs() {
            walk(sub, out);
        }
    }
    let mut out = Vec::new();
    walk(&CONTENT_DIR, &mut out);
    out
}


fn find_embedded_file(path: &str) -> Option<&'static include_dir::File<'static>> {
    all_files().into_iter().find(|f| normalize(f.path()) == path)
}

fn read_file(path: &str) -> AppResult<&'static str> {
    find_embedded_file(path)
        .and_then(|f| f.contents_utf8())
        .ok_or_else(|| AppError::Content(format!("missing or non-utf8 content file: {path}")))
}


fn parse_json<T: DeserializeOwned>(path: &str) -> AppResult<T> {
    let raw = read_file(path)?;
    serde_json::from_str(raw).map_err(|e| AppError::Content(format!("invalid JSON in {path}: {e}")))
}

/// All embedded files under `<track-dir>/<module>/module.json` — exactly one
/// directory level below the track dir (which may itself contain slashes).
fn module_json_paths(track_dir: &str) -> Vec<String> {
    let prefix = format!("{track_dir}/");
    let mut paths = Vec::new();
    for file in all_files() {
        let path = normalize(file.path());
        if let Some(rest) = path.strip_prefix(&prefix) {
            if path.ends_with("/module.json")
                && !rest.is_empty()
                && rest.matches('/').count() == 1
            {
                paths.push(path);
            }
        }
    }
    paths.sort();
    paths
}

fn project_json_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for file in all_files() {
        let path = normalize(file.path());
        if path.starts_with("projects/")
            && path.ends_with("/project.json")
            && path.matches('/').count() == 2
        {
            paths.push(path);
        }
    }
    paths.sort();
    paths
}

/// Load and cross-validate the full learning catalog from the embedded content tree.

pub fn load_catalog() -> AppResult<Catalog> {
    let mut tracks = Vec::new();
    let mut module_ids: HashSet<String> = HashSet::new();
    let mut lesson_ids: HashSet<String> = HashSet::new();

    for decl in TRACKS.iter() {
        let tid = decl.id;
        // Phase 1: definitions first (stable ordering before lesson assembly).
        let mut defs: Vec<(ModuleDef, String)> = Vec::new();
        for path in module_json_paths(decl.dir) {
            let def: ModuleDef = parse_json(&path)?;
            if !module_ids.insert(def.id.clone()) {
                return Err(AppError::Content(format!("duplicate module id '{}'", def.id)));
            }
            let module_dir = path
                .strip_suffix("/module.json")
                .unwrap_or(&path)
                .to_string();
            defs.push((def, module_dir));
        }
        defs.sort_by_key(|(d, _)| d.index);

        // A declared track with no shipped content yet is skipped here; the
        // content-integrity test enforces that every declared track ships.
        if defs.is_empty() {
            continue;
        }

        let modules = build_modules(tid, defs, &mut lesson_ids)?;
        tracks.push(Track {
            id: tid.to_string(),
            title: decl.title.to_string(),
            description: decl.description.to_string(),
            category: decl.category.to_string(),
            modules,
        });
    }

    validate_prerequisites(&tracks, &module_ids)?;

    Ok(Catalog {
        version: env!("CARGO_PKG_VERSION").to_string(),
        tracks,
    })
}

/// Assemble a track's modules (and their lessons) from parsed definitions.
fn build_modules(
    track_id: &str,
    defs: Vec<(ModuleDef, String)>,
    lesson_ids: &mut HashSet<String>,
) -> AppResult<Vec<Module>> {
    let mut modules = Vec::with_capacity(defs.len());
    for (def, module_dir) in defs {
        let mut lessons = Vec::with_capacity(def.lessons.len());
        for (i, lref) in def.lessons.iter().enumerate() {
            let dir = format!("{module_dir}/lessons/{}", lref.slug);
            lessons.push(build_lesson(
                track_id,
                &def.id,
                lref.slug.clone(),
                (i + 1) as u32,
                &dir,
                lesson_ids,
            )?);
        }
        modules.push(Module {
            id: def.id,
            track_id: track_id.to_string(),
            slug: def.slug,
            index: def.index,
            title: def.title,
            summary: def.summary,
            prerequisites: def.prerequisites,
            difficulty: def.difficulty,
            skills: def.skills,
            lessons,
        });
    }
    Ok(modules)
}

/// Parse one lesson folder (`lesson.json` + `lesson.md`) with full validation.
fn build_lesson(
    track_id: &str,
    module_id: &str,
    slug: String,
    index: u32,
    dir: &str,
    lesson_ids: &mut HashSet<String>,
) -> AppResult<Lesson> {
    let meta: LessonMeta = parse_json(&format!("{dir}/lesson.json"))?;
    let sibling_markdown =
        find_embedded_file(&format!("{dir}/lesson.md")).and_then(|f| f.contents_utf8());

    let markdown = match sibling_markdown {
        Some(raw) => raw.to_string(),
        None => meta.body.clone(),
    };

    if markdown.trim().is_empty() {
        return Err(AppError::Content(format!("empty lesson body: {dir}/lesson.md")));
    }
    if !lesson_ids.insert(meta.id.clone()) {
        return Err(AppError::Content(format!("duplicate lesson id '{}'", meta.id)));
    }
    if let Some(q) = &meta.quiz {
        if q.questions.is_empty() {
            return Err(AppError::Content(format!("quiz '{}' has no questions", q.id)));
        }
        for question in &q.questions {
            if question.options.len() < 2 || question.answer_index >= question.options.len() {
                return Err(AppError::Content(format!(
                    "quiz question '{}' in {} has invalid answer index",
                    question.id, meta.id
                )));
            }
        }
    }
    if let Some(ex) = &meta.exercise {
        if ex.hints.is_empty() {
            return Err(AppError::Content(format!(
                "exercise '{}' in {} must provide at least one hint",
                ex.id, meta.id
            )));
        }
    }
    Ok(Lesson {
        id: meta.id,
        module_id: module_id.to_string(),
        track_id: track_id.to_string(),
        slug,
        index,
        title: meta.title,
        minutes: meta.minutes,
        kind: meta.kind,
        objectives: meta.objectives,
        smells: meta.smells,
        examples: meta.examples,
        exercise: meta.exercise,
        quiz: meta.quiz,
        skills: meta.skills,
        related: meta.related,
        markdown,
    })
}

fn validate_prerequisites(tracks: &[Track], module_ids: &HashSet<String>) -> AppResult<()> {
    for track in tracks {
        for module in &track.modules {
            for prereq in &module.prerequisites {
                if !module_ids.contains(prereq) {
                    return Err(AppError::Content(format!(
                        "module '{}' references unknown prerequisite '{}'",
                        module.id, prereq
                    )));
                }
            }
        }
    }
    Ok(())
}

/// Load every project specification (`content/projects/<id>/project.json`).
pub fn load_projects() -> AppResult<Vec<ProjectSpec>> {
    let mut projects = Vec::new();
    let mut ids = HashSet::new();
    for path in project_json_paths() {
        let spec: ProjectSpec = parse_json(&path)?;
        if !ids.insert(spec.id.clone()) {
            return Err(AppError::Content(format!("duplicate project id '{}'", spec.id)));
        }
        projects.push(spec);
    }
    projects.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(projects)
}


/// Load the capstone specification (`content/capstone/capstone.json`).
pub fn load_capstone() -> AppResult<CapstoneSpec> {
    parse_json("capstone/capstone.json")
}

/// Load the declarative skill graph (`content/skills/skills.json`).
pub fn load_skill_graph() -> AppResult<SkillGraph> {
    parse_json("skills/skills.json")
}

