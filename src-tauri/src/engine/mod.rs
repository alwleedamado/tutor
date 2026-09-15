//! Learning engine — pure functions over `(catalog, completed_lessons)`.
//!
//! Unlock evaluation reads declarative prerequisites from content metadata;
//! nothing about Tauri gating or capstone rules is hardcoded here. See
//! docs/learning-engine.md for the model contract.

use std::collections::{BTreeMap, HashSet};

use crate::content::model::{
    Catalog, CompletedLesson, ExerciseStat, ModuleState, NextUp, ProgressSummary, ProjectSpec,
    QuizStat, SkillGraph,
};

pub mod skills;

/// One-shot summary builder shared by bootstrap and mutation commands. The
/// full variant also derives rich lesson progress, skill states and
/// deterministic recommendations.
#[allow(clippy::too_many_arguments)]
pub fn summarize_full(
    catalog: &Catalog,
    completed: &[CompletedLesson],
    capstone_prerequisites: &[String],
    graph: Option<&SkillGraph>,
    quiz_stats: &[QuizStat],
    exercise_stats: &[ExerciseStat],
    projects: &[ProjectSpec],
    completed_projects: &[String],
) -> ProgressSummary {
    let mut summary = summarize(catalog, completed, capstone_prerequisites);
    let done_keys = skills::done_key_set(completed);
    summary.lesson_progress = skills::lesson_progress(catalog, &done_keys, quiz_stats, exercise_stats);
    if let Some(graph) = graph {
        summary.skills = skills::skill_states(catalog, graph, &done_keys);
        summary.recommendations =
            skills::recommend(catalog, Some(graph), &done_keys, quiz_stats, projects, completed_projects);
    }
    summary
}


/// Canonical composite key used everywhere a lesson is addressed.
pub fn lesson_key(track: &str, module: &str, lesson_id: &str) -> String {
    format!("{track}:{module}:{lesson_id}")
}

/// Module ids whose every lesson is completed.
pub fn completed_module_ids(catalog: &Catalog, done_keys: &HashSet<String>) -> HashSet<String> {
    let mut out = HashSet::new();
    for track in &catalog.tracks {
        for module in &track.modules {
            if !module.lessons.is_empty()
                && module
                    .lessons
                    .iter()
                    .all(|l| done_keys.contains(&lesson_key(&track.id, &module.id, &l.id)))
            {
                out.insert(module.id.clone());
            }
        }
    }
    out
}

pub fn module_states(catalog: &Catalog, done_keys: &HashSet<String>) -> Vec<ModuleState> {
    let completed_modules = completed_module_ids(catalog, done_keys);
    let mut states = Vec::new();
    for track in &catalog.tracks {
        for module in &track.modules {
            let total = module.lesson_count() as u32;
            let done = module
                .lessons
                .iter()
                .filter(|l| done_keys.contains(&lesson_key(&track.id, &module.id, &l.id)))
                .count() as u32;
            states.push(ModuleState {
                module_id: module.id.clone(),
                total,
                done,
                completed: total > 0 && done == total,
                locked: !module.prerequisites.iter().all(|p| completed_modules.contains(p)),
            });
        }
    }
    states
}

pub fn track_percents(catalog: &Catalog, done_keys: &HashSet<String>) -> BTreeMap<String, u32> {
    let mut map = BTreeMap::new();
    for track in &catalog.tracks {
        let total: usize = track.modules.iter().map(|m| m.lesson_count()).sum();
        let done: usize = track
            .modules
            .iter()
            .map(|m| {
                m.lessons
                    .iter()
                    .filter(|l| done_keys.contains(&lesson_key(&track.id, &m.id, &l.id)))
                    .count()
            })
            .sum();
        let percent = if total == 0 { 0 } else { (done * 100 / total) as u32 };
        map.insert(track.id.clone(), percent.min(100));
    }
    map
}


pub fn capstone_unlocked(prerequisites: &[String], completed_modules: &HashSet<String>) -> bool {
    prerequisites.iter().all(|p| completed_modules.contains(p))
}

/// First incomplete lesson across unlocked modules, scanning tracks in
/// catalog order (core tracks first, then foundations, then specialized).
pub fn next_recommendation(
    catalog: &Catalog,
    done_keys: &HashSet<String>,
    states: &[ModuleState],
) -> Option<NextUp> {
    for track in &catalog.tracks {
        let track_id = track.id.as_str();
        let mut modules: Vec<&crate::content::model::Module> = track.modules.iter().collect();
        modules.sort_by_key(|m| m.index);
        for module in modules {
            let locked = states.iter().any(|s| s.module_id == module.id && s.locked);
            if locked {
                continue;
            }
            if let Some(lesson) = module
                .lessons
                .iter()
                .find(|l| !done_keys.contains(&lesson_key(track_id, &module.id, &l.id)))
            {
                return Some(NextUp {
                    track_id: track_id.to_string(),
                    module_id: module.id.clone(),
                    lesson_slug: lesson.slug.clone(),
                });
            }
        }
    }
    None
}

/// One-shot summary builder shared by bootstrap and mutation commands.
pub fn summarize(
    catalog: &Catalog,
    completed: &[CompletedLesson],
    capstone_prerequisites: &[String],
) -> ProgressSummary {
    let keys: HashSet<String> = completed
        .iter()
        .map(|c| lesson_key(&c.track_id, &c.module_id, &c.lesson_id))
        .collect();
    let completed_modules = completed_module_ids(catalog, &keys);
    let states = module_states(catalog, &keys);
    let next = next_recommendation(catalog, &keys, &states);
    ProgressSummary {
        lessons: completed.to_vec(),
        track_percent: track_percents(catalog, &keys),
        capstone_unlocked: capstone_unlocked(capstone_prerequisites, &completed_modules),
        modules: states,
        next,
        lesson_progress: Vec::new(),
        skills: Vec::new(),
        recommendations: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::model::{Lesson, Module, Track};


    fn lesson(id: &str, slug: &str) -> Lesson {
        Lesson {
            id: id.into(),
            module_id: String::new(),
            track_id: String::new(),
            slug: slug.into(),
            index: 0,
            title: id.into(),
            minutes: 10,
            kind: "concept".into(),
            objectives: vec![],
            smells: vec![],
            examples: vec![],
            exercise: None,
            quiz: None,
            skills: vec![],
            related: vec![],
            markdown: "# x".into(),
        }
    }

    fn module(id: &str, index: u32, prereqs: &[&str], lessons: Vec<Lesson>) -> Module {
        Module {
            id: id.into(),
            track_id: String::new(),
            slug: format!("module-{index:02}"),
            index,
            title: id.into(),
            summary: String::new(),
            prerequisites: prereqs.iter().map(|s| s.to_string()).collect(),
            difficulty: "intermediate".into(),
            skills: vec![],
            lessons,
        }
    }

    /// rust-01 ← rust-02; vue-01 independent; tauri-01 requires rust-01.
    fn sample_catalog() -> Catalog {
        let mut r1 = module(
            "rust-01",
            1,
            &[],
            vec![lesson("rust-01-l01", "a"), lesson("rust-01-l02", "b")],
        );
        for l in &mut r1.lessons {
            l.module_id = "rust-01".into();
            l.track_id = "rust".into();
        }
        let mut r2 = module("rust-02", 2, &["rust-01"], vec![lesson("rust-02-l01", "c")]);
        for l in &mut r2.lessons {
            l.module_id = "rust-02".into();
            l.track_id = "rust".into();
        }
        let mut v1 = module("vue-01", 1, &[], vec![lesson("vue-01-l01", "d")]);
        for l in &mut v1.lessons {
            l.module_id = "vue-01".into();
            l.track_id = "vue".into();
        }
        let mut t1 = module("tauri-01", 1, &["rust-01"], vec![lesson("tauri-01-l01", "e")]);
        for l in &mut t1.lessons {
            l.module_id = "tauri-01".into();
            l.track_id = "tauri".into();
        }
        Catalog {
            version: "test".into(),
            tracks: vec![
                Track { id: "rust".into(), title: "Rust".into(), description: "".into(), category: "Languages".into(), modules: vec![r1, r2] },
                Track { id: "vue".into(), title: "Vue".into(), description: "".into(), category: "Frontend".into(), modules: vec![v1] },
                Track { id: "tauri".into(), title: "Tauri".into(), description: "".into(), category: "Desktop".into(), modules: vec![t1] },
            ],
        }
    }

    fn keys<'a>(items: impl IntoIterator<Item = &'a str>) -> HashSet<String> {
        items.into_iter().map(str::to_string).collect()
    }

    #[test]
    fn empty_progress_locks_dependent_modules_only() {
        let catalog = sample_catalog();
        let states = module_states(&catalog, &HashSet::new());
        let state = |id: &str| states.iter().find(|s| s.module_id == id).unwrap();
        assert!(!state("rust-01").locked);
        assert!(state("rust-02").locked);
        assert!(!state("vue-01").locked, "vue is an independent track");
        assert!(state("tauri-01").locked);
    }

    #[test]
    fn partial_module_completion_does_not_unlock() {
        let catalog = sample_catalog();
        let done = keys(["rust:rust-01:rust-01-l01"]);
        let states = module_states(&catalog, &done);
        let state = |id: &str| states.iter().find(|s| s.module_id == id).unwrap();
        assert_eq!(state("rust-01").done, 1);
        assert!(!state("rust-01").completed);
        assert!(state("rust-02").locked, "prereq requires FULL completion");
        assert!(state("tauri-01").locked);
    }

    #[test]
    fn full_completion_unlocks_both_successors() {
        let catalog = sample_catalog();
        let done = keys(["rust:rust-01:rust-01-l01", "rust:rust-01:rust-01-l02"]);
        let states = module_states(&catalog, &done);
        let state = |id: &str| states.iter().find(|s| s.module_id == id).unwrap();
        assert!(state("rust-01").completed);
        assert!(!state("rust-02").locked);
        assert!(!state("tauri-01").locked);
    }

    #[test]
    fn recommendation_skips_locked_and_completed() {
        let catalog = sample_catalog();
        let rec = next_recommendation(&catalog, &HashSet::new(), &module_states(&catalog, &HashSet::new()))
            .expect("recommendation for fresh learner");
        assert_eq!(rec.track_id, "rust");
        assert_eq!(rec.module_id, "rust-01");

        let done = keys(["rust:rust-01:rust-01-l01", "rust:rust-01:rust-01-l02"]);
        let rec = next_recommendation(&catalog, &done, &module_states(&catalog, &done)).unwrap();
        assert_eq!(rec.module_id, "rust-02", "curriculum order continues within track");
    }

    #[test]
    fn capstone_requires_every_prerequisite() {
        let prereqs: Vec<String> = ["rust-06", "vue-04", "tauri-03"].iter().map(|s| s.to_string()).collect();
        assert!(!capstone_unlocked(&prereqs, &HashSet::new()));
        let two_of_three = keys(["rust-06", "vue-04"]);
        assert!(!capstone_unlocked(&prereqs, &two_of_three));
        let all: HashSet<String> = prereqs.iter().cloned().collect();
        assert!(capstone_unlocked(&prereqs, &all));
    }

    #[test]
    fn track_percent_is_bounded_and_accurate() {
        let catalog = sample_catalog();
        let done = keys(["rust:rust-01:rust-01-l01"]);
        let percents = track_percents(&catalog, &done);
        assert_eq!(percents.get("rust"), Some(&33)); // 1 of 3 rust lessons

        assert_eq!(percents.get("vue"), Some(&0));
        assert_eq!(percents.get("tauri"), Some(&0));
    }
}


