//! Skill graph evaluation and deterministic recommendations.
//!
//! The skill graph is declarative content (`content/skills/skills.json`).
//! Skills gather evidence from catalog entities that declare them (lessons
//! and modules via their `skills` arrays). A skill is *demonstrated* when all
//! of its evidence is completed. Nothing here is hardcoded to any track.

use std::collections::{HashMap, HashSet};

use crate::content::model::{
    Catalog, CompletedLesson, ExerciseStat, LessonProgress, ProjectSpec, QuizStat, Recommendation,
    SkillGraph, SkillState,
};

use super::{completed_module_ids, lesson_key, next_recommendation};

/// Skill ids with every evidence entity completed, plus dependency closure.
pub fn demonstrated_skills(
    catalog: &Catalog,
    graph: &SkillGraph,
    done_keys: &HashSet<String>,
) -> HashSet<String> {
    // Base evidence: a skill is satisfied when every lesson and module that
    // grants it is completed.
    let mut resolved = HashSet::new();
    for node in &graph.skills {
        let (lessons_done, lessons_total) = lesson_evidence(catalog, &node.id, done_keys);
        let (mods_done, mods_total) = module_evidence(catalog, &node.id, done_keys);
        let has_evidence = lessons_total + mods_total > 0;
        if has_evidence && lessons_done == lessons_total && mods_done == mods_total {
            resolved.insert(node.id.clone());
        }
    }
    // Gateway skills (no direct evidence) are demonstrated purely through
    // their dependencies; propagate until fixpoint (graph is small). Nodes
    // WITH evidence must always satisfy their evidence directly — closure
    // never substitutes for unfinished lessons/modules.
    for _ in 0..graph.skills.len() + 1 {
        let mut changed = false;
        for node in &graph.skills {
            if resolved.contains(&node.id) || node.requires.is_empty() {
                continue;
            }
            let (lessons_total, mods_total) = {
                let ( _, lt) = lesson_evidence(catalog, &node.id, done_keys);
                let (_, mt) = module_evidence(catalog, &node.id, done_keys);
                (lt, mt)
            };
            if lessons_total + mods_total > 0 {
                continue;
            }
            if node.requires.iter().all(|r| resolved.contains(r)) {
                resolved.insert(node.id.clone());
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    resolved
}

/// (done, total) lessons granting `skill_id`.
fn lesson_evidence(
    catalog: &Catalog,
    skill_id: &str,
    done_keys: &HashSet<String>,
) -> (u32, u32) {
    let mut done = 0u32;
    let mut total = 0u32;
    for track in &catalog.tracks {
        for module in &track.modules {
            for lesson in &module.lessons {
                if lesson.skills.iter().any(|s| s == skill_id) {
                    total += 1;
                    if done_keys.contains(&lesson_key(&track.id, &module.id, &lesson.id)) {
                        done += 1;
                    }
                }
            }
        }
    }
    (done, total)
}

/// (completed, total) modules granting `skill_id`.
fn module_evidence(
    catalog: &Catalog,
    skill_id: &str,
    done_keys: &HashSet<String>,
) -> (u32, u32) {
    let completed = completed_module_ids(catalog, done_keys);
    let mut done = 0u32;
    let mut total = 0u32;
    for track in &catalog.tracks {
        for module in &track.modules {
            if module.skills.iter().any(|s| s == skill_id) {
                total += 1;
                if completed.contains(&module.id) {
                    done += 1;
                }
            }
        }
    }
    (done, total)
}

/// Runtime states for every skill in the graph (graph order = stable output).
pub fn skill_states(
    catalog: &Catalog,
    graph: &SkillGraph,
    done_keys: &HashSet<String>,
) -> Vec<SkillState> {
    let demonstrated = demonstrated_skills(catalog, graph, done_keys);
    let node_by_id: HashMap<&str, &crate::content::model::SkillNode> =
        graph.skills.iter().map(|s| (s.id.as_str(), s)).collect();

    graph
        .skills
        .iter()
        .map(|node| {
            let (lessons_done, lessons_total) = lesson_evidence(catalog, &node.id, done_keys);
            let (mods_done, mods_total) = module_evidence(catalog, &node.id, done_keys);
            let missing: Vec<String> = node
                .requires
                .iter()
                .filter(|r| !demonstrated.contains(*r))
                .filter_map(|r| node_by_id.get(r.as_str()).map(|n| n.label.clone()))
                .collect();
            let evidence_complete =
                lessons_done == lessons_total && mods_done == mods_total && mods_total + lessons_total > 0;
            SkillState {
                id: node.id.clone(),
                label: node.label.clone(),
                category: node.category.clone(),
                demonstrated: evidence_complete,
                lessons_total,
                lessons_done,
                modules_total: mods_total,
                modules_done: mods_done,
                dependencies_met: missing.is_empty(),
                missing_dependencies: missing,
            }
        })
        .collect()
}

/// Rich per-lesson progress derived from completions + attempt evidence.
pub fn lesson_progress(
    catalog: &Catalog,
    done_keys: &HashSet<String>,
    quiz_stats: &[QuizStat],
    exercise_stats: &[ExerciseStat],
) -> Vec<LessonProgress> {
    let quiz_by_key: HashMap<String, &QuizStat> = quiz_stats
        .iter()
        .map(|q| (lesson_key(&q.track_id, &q.module_id, &q.lesson_id), q))
        .collect();
    let ex_by_key: HashMap<String, &ExerciseStat> = exercise_stats
        .iter()
        .map(|e| (lesson_key(&e.track_id, &e.module_id, &e.lesson_id), e))
        .collect();

    let mut out = Vec::new();
    for track in &catalog.tracks {
        for module in &track.modules {
            for lesson in &module.lessons {
                let key = lesson_key(&track.id, &module.id, &lesson.id);
                let quiz = quiz_by_key.get(&key).copied();
                let ex = ex_by_key.get(&key).copied();
                let attempts = ex.map(|e| e.attempts).unwrap_or(0);
                let quiz_tried = quiz.map(|q| q.attempts > 0).unwrap_or(false);
                let state = if done_keys.contains(&key) {
                    let quiz_ok = quiz.map(|q| q.passed).unwrap_or(true);
                    let exercise_ok = match ex {
                        Some(e) => !e.solved || !e.solution_revealed,
                        None => true,
                    };
                    if quiz_ok && exercise_ok {
                        "mastered"
                    } else {
                        "completed"
                    }
                } else if attempts >= 3 || ex.map(|e| e.solution_revealed).unwrap_or(false) {
                    "practicing"
                } else if attempts > 0 || quiz_tried {
                    "attempted"
                } else {
                    "not-started"
                };
                out.push(LessonProgress {
                    track_id: track.id.clone(),
                    module_id: module.id.clone(),
                    lesson_id: lesson.id.clone(),
                    state: state.to_string(),
                    attempts,
                    best_quiz_percent: quiz.map(|q| q.best_percent),
                    quiz_passed: quiz.map(|q| q.passed).unwrap_or(false),
                    solved_without_solution: ex.map(|e| e.solved && !e.solution_revealed).unwrap_or(false),
                });
            }
        }
    }
    out
}

/// Deterministic, explainable recommendations.
///
/// Priority: continue (next lesson) → unblock (prerequisites) → review failed
/// quizzes → close skill gaps → start available projects. Every entry carries
/// a human-readable `reason` derived from catalog + progress facts.
pub fn recommend(
    catalog: &Catalog,
    graph: Option<&SkillGraph>,
    done_keys: &HashSet<String>,
    quiz_stats: &[QuizStat],
    projects: &[ProjectSpec],
    completed_projects: &[String],
) -> Vec<Recommendation> {
    let states = super::module_states(catalog, done_keys);
    let mut out: Vec<Recommendation> = Vec::new();
    let mut seen_modules: HashSet<String> = HashSet::new();

    // 1. Continue: the global next unfinished lesson.
    if let Some(next) = next_recommendation(catalog, done_keys, &states) {
        let module_title = catalog
            .tracks
            .iter()
            .find(|t| t.id == next.track_id)
            .and_then(|t| t.modules.iter().find(|m| m.id == next.module_id))
            .map(|m| m.title.clone())
            .unwrap_or_else(|| next.module_id.clone());
        out.push(Recommendation {
            kind: "next-lesson".into(),
            title: format!("Continue: {}", module_title),
            reason: format!(
                "Next unfinished lesson in {} — the first incomplete lesson across unlocked tracks.",
                module_title
            ),
            track_id: Some(next.track_id),
            module_id: Some(next.module_id),
            lesson_slug: Some(next.lesson_slug),
            project_id: None,
            skill_id: None,
        });
    }

    // 2. Unblock: first locked module per track → its first missing prereq.
    let completed_modules = completed_module_ids(catalog, done_keys);
    for track in &catalog.tracks {
        let mut modules: Vec<&crate::content::model::Module> = track.modules.iter().collect();
        modules.sort_by_key(|m| m.index);
        for module in modules {
            let locked = states
                .iter()
                .any(|s| &s.module_id == &module.id && s.locked);
            if !locked || seen_modules.contains(&module.id) {
                continue;
            }
            let missing = module
                .prerequisites
                .iter()
                .find(|p| !completed_modules.contains(*p));
            if let Some(prereq_id) = missing {
                if seen_modules.contains(prereq_id) {
                    continue;
                }
                if let Some(prereq) = catalog
                    .tracks
                    .iter()
                    .flat_map(|t| t.modules.iter())
                    .find(|m| &m.id == prereq_id)
                {
                    if let Some(first) = prereq.lessons.first() {
                        seen_modules.insert(prereq.id.clone());
                        out.push(Recommendation {
                            kind: "prerequisite".into(),
                            title: format!("Unlock {}: {}", track.title, prereq.title),
                            reason: format!(
                                "{} is locked until “{}” is complete — start with its first lesson.",
                                module.title, prereq.title
                            ),
                            track_id: Some(prereq.track_id.clone()),
                            module_id: Some(prereq.id.clone()),
                            lesson_slug: Some(first.slug.clone()),
                            project_id: None,
                            skill_id: None,
                        });
                    }
                }
            }
            if out.len() >= 4 {
                break;
            }
        }
        if out.len() >= 4 {
            break;
        }
    }

    // 3. Review: attempted quizzes that were not passed.
    let lesson_index: HashMap<String, (&crate::content::model::Lesson, &crate::content::model::Module, &crate::content::model::Track)> = catalog
        .tracks
        .iter()
        .flat_map(|t| t.modules.iter().map(move |m| (t, m)))
        .flat_map(|(t, m)| m.lessons.iter().map(move |l| (t, m, l)))
        .map(|(t, m, l)| (lesson_key(&t.id, &m.id, &l.id), (l, m, t)))
        .collect();
    let mut reviews = 0;
    for stat in quiz_stats {
        if reviews >= 2 {
            break;
        }
        if stat.attempts == 0 || stat.passed {
            continue;
        }
        if let Some((lesson, module, track)) =
            lesson_index.get(&lesson_key(&stat.track_id, &stat.module_id, &stat.lesson_id))
        {
            out.push(Recommendation {
                kind: "review".into(),
                title: format!("Review: {}", lesson.title),
                reason: format!(
                    "Quiz not yet passed in {} · {} (best {}%).",
                    track.title, module.title, stat.best_percent
                ),
                track_id: Some(track.id.clone()),
                module_id: Some(module.id.clone()),
                lesson_slug: Some(lesson.slug.clone()),
                project_id: None,
                skill_id: None,
            });
            reviews += 1;
        }
    }

    // 4. Skill gaps: first unmet dependency of reachable-but-undemonstrated skills.
    if let Some(graph) = graph {
        let skill_states = skill_states(catalog, graph, done_keys);
        let label_by_id: HashMap<&str, &str> =
            graph.skills.iter().map(|s| (s.id.as_str(), s.label.as_str())).collect();
        let mut gaps = 0;
        for state in &skill_states {
            if gaps >= 2 {
                break;
            }
            if state.demonstrated || state.dependencies_met {
                continue;
            }
            if let Some(dep) = state.missing_dependencies.first() {
                out.push(Recommendation {
                    kind: "skill-gap".into(),
                    title: format!("Foundations first: {}", dep),
                    reason: format!(
                        "Skill “{}” requires “{}” to be demonstrated before it can be earned.",
                        state.label, dep
                    ),
                    track_id: None,
                    module_id: None,
                    lesson_slug: None,
                    project_id: None,
                    skill_id: Some(
                        graph
                            .skills
                            .iter()
                            .find(|s| label_by_id.get(s.id.as_str()).copied().as_deref() == Some(dep.as_str()))
                            .map(|s| s.id.clone())
                            .unwrap_or_default(),
                    ),
                });
                gaps += 1;
            }
        }
    }

    // 5. Projects: specs tied to an unlocked module that are not yet completed.
    let done_projects: HashSet<&str> = completed_projects.iter().map(String::as_str).collect();
    let mut started = 0;
    for spec in projects {
        if started >= 2 {
            break;
        }
        let Some(module_id) = &spec.module_id else { continue };
        if done_projects.contains(spec.id.as_str()) {
            continue;
        }
        let locked = states
            .iter()
            .any(|s| &s.module_id == module_id && s.locked);
        if locked {
            continue;
        }
        out.push(Recommendation {
            kind: "project".into(),
            title: format!("Project: {}", spec.title),
            reason: format!(
                "Its module ({}) is unlocked and the project has not been completed yet.",
                module_id
            ),
            track_id: spec.track_id.clone(),
            module_id: Some(module_id.clone()),
            lesson_slug: None,
            project_id: Some(spec.id.clone()),
            skill_id: None,
        });
        started += 1;
    }

    out.truncate(8);
    out
}

/// Convenience wrapper mirroring `CompletedLesson` collection shape.
pub fn done_key_set(completed: &[CompletedLesson]) -> HashSet<String> {
    completed
        .iter()
        .map(|c| lesson_key(&c.track_id, &c.module_id, &c.lesson_id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::model::{ExerciseStat, Lesson, Module, SkillNode, Track};

    fn lesson(id: &str, slug: &str, skills: &[&str]) -> Lesson {
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
            skills: skills.iter().map(|s| s.to_string()).collect(),
            related: vec![],
            markdown: "# x".into(),
        }
    }

    fn module(id: &str, prereqs: &[&str], lessons: Vec<Lesson>, skills: &[&str]) -> Module {
        Module {
            id: id.into(),
            track_id: "t".into(),
            slug: id.into(),
            index: 1,
            title: id.into(),
            summary: String::new(),
            prerequisites: prereqs.iter().map(|s| s.to_string()).collect(),
            difficulty: "beginner".into(),
            skills: skills.iter().map(|s| s.to_string()).collect(),
            lessons,
        }
    }

    fn graph(nodes: &[(&str, &str, &[&str])]) -> SkillGraph {
        SkillGraph {
            version: 1,
            skills: nodes
                .iter()
                .map(|(id, label, requires)| SkillNode {
                    id: id.to_string(),
                    label: label.to_string(),
                    category: "Test".into(),
                    summary: String::new(),
                    requires: requires.iter().map(|s| s.to_string()).collect(),
                })
                .collect(),
        }
    }

    fn catalog(modules: Vec<Module>) -> Catalog {
        Catalog {
            version: "test".into(),
            tracks: vec![Track {
                id: "t".into(),
                title: "T".into(),
                description: String::new(),
                category: "Languages".into(),
                modules,
            }],
        }
    }

    fn keys<'a>(items: impl IntoIterator<Item = &'a str>) -> HashSet<String> {
        items.into_iter().map(str::to_string).collect()
    }

    #[test]
    fn skill_demonstration_requires_all_evidence_and_dependency_closure() {
        let g = graph(&[
            ("a", "Skill A", &[]),
            ("b", "Skill B", &["a"]),
            ("c", "Gateway", &["b"]),
            ("d", "Unreachable", &["missing-dep"]),
        ]);
        let cat = catalog(vec![module(
            "m1",
            &[],
            vec![
                lesson("l1", "one", &["a"]),
                lesson("l2", "two", &["a"]),
                lesson("l3", "three", &["b"]),
            ],
            &[],
        )]);

        // Nothing done → nothing demonstrated (gateway nodes need a demonstrated dep).
        let demonstrated = demonstrated_skills(&cat, &g, &HashSet::new());
        assert!(demonstrated.is_empty());

        // Partial evidence → still not demonstrated.
        let done = keys(["t:m1:l1"]);
        assert!(!demonstrated_skills(&cat, &g, &done).contains("a"));

        // Full evidence → a demonstrated; b/c via closure (b also has lesson l3 unfinished → NOT).
        let done = keys(["t:m1:l1", "t:m1:l2"]);
        let demonstrated = demonstrated_skills(&cat, &g, &done);
        assert!(demonstrated.contains("a"), "all evidence complete");
        assert!(!demonstrated.contains("b"), "b has unfinished lesson evidence");
        assert!(!demonstrated.contains("c"), "c depends on b");

        // b's evidence complete → closure demonstrates b and c.
        let done = keys(["t:m1:l1", "t:m1:l2", "t:m1:l3"]);
        let demonstrated = demonstrated_skills(&cat, &g, &done);
        assert!(demonstrated.contains("b"));
        assert!(demonstrated.contains("c"), "gateway via closure");
        assert!(!demonstrated.contains("d"), "missing dep never demonstrated");
    }

    #[test]
    fn lesson_progress_states_are_derived_deterministically() {
        let cat = catalog(vec![module(
            "m1",
            &[],
            vec![
                lesson("l1", "one", &[]),
                lesson("l2", "two", &[]),
                lesson("l3", "three", &[]),
                lesson("l4", "four", &[]),
                lesson("l5", "five", &[]),
            ],
            &[],
        )]);
        let quiz = |lesson: &str, attempts: u32, best: u32, passed: bool| QuizStat {
            track_id: "t".into(),
            module_id: "m1".into(),
            lesson_id: lesson.into(),
            attempts,
            best_percent: best,
            passed,
        };
        let ex = |lesson: &str, attempts: u32, solved: bool, revealed: bool| ExerciseStat {
            track_id: "t".into(),
            module_id: "m1".into(),
            lesson_id: lesson.into(),
            attempts,
            solved,
            solution_revealed: revealed,
        };
        // l1 untouched; l2 one try; l3 revealed; l4 completed+quiz fail;
        // l5 completed + quiz passed + solved w/o solution → mastered.
        let done = keys(["t:m1:l4", "t:m1:l5"]);
        let qs = vec![quiz("l4", 1, 50, false), quiz("l5", 1, 100, true)];
        let es = vec![ex("l2", 1, false, false), ex("l3", 1, false, true), ex("l5", 2, true, false)];
        let states: HashMap<String, String> = lesson_progress(&cat, &done, &qs, &es)
            .into_iter()
            .map(|p| (p.lesson_id, p.state))
            .collect();
        assert_eq!(states["l1"], "not-started");
        assert_eq!(states["l2"], "attempted");
        assert_eq!(states["l3"], "practicing");
        assert_eq!(states["l4"], "completed", "quiz failed → plain completed");
        assert_eq!(states["l5"], "mastered");
    }

    #[test]
    fn recommendations_are_deterministic_and_explainable() {
        let g = graph(&[("a", "Skill A", &[]), ("b", "Skill B", &["a"])]);
        let cat = catalog(vec![
            module("m1", &[], vec![lesson("m1-l1", "one", &[])], &[]),
            module("m2", &["m1"], vec![lesson("m2-l1", "two", &[])], &[]),
        ]);
        let projects = vec![ProjectSpec {
            id: "proj-1".into(),
            title: "Project One".into(),
            track_id: Some("t".into()),
            module_id: Some("m1".into()),
            summary: "s".into(),
            requirements: vec![],
            milestones: vec![],
            starter_files: vec![],
            hints: vec![],
            completion_criteria: vec![],
            estimated_hours: 4,
        }];

        // Fresh learner: next-lesson first, then unlock hint for m2 via m1, then skill gap.
        let recs = recommend(&cat, Some(&g), &HashSet::new(), &[], &projects, &[]);
        assert_eq!(recs.first().map(|r| r.kind.as_str()), Some("next-lesson"));
        assert!(recs.iter().any(|r| r.kind == "prerequisite" && r.module_id.as_deref() == Some("m1")));
        assert!(
            recs.iter()
                .any(|r| r.kind == "skill-gap" && r.reason.contains("Skill A")),
            "skill gap names the missing dependency"
        );

        // Completed m1 (project not done) → project recommendation appears.
        let done = keys(["t:m1:m1-l1"]);
        let recs = recommend(&cat, Some(&g), &done, &[], &projects, &[]);
        assert!(recs.iter().any(|r| r.kind == "project" && r.project_id.as_deref() == Some("proj-1")));

        // Completed project is not recommended again.
        let recs = recommend(&cat, Some(&g), &done, &[], &projects, &["proj-1".to_string()]);
        assert!(!recs.iter().any(|r| r.kind == "project"));
    }
}
