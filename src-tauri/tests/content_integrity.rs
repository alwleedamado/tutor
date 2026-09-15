//! Cross-module guarantees over the REAL embedded curriculum.

use std::collections::HashSet;

use rust_mastery_lib::content;
use rust_mastery_lib::content::model::CompletedLesson;
use rust_mastery_lib::engine;

#[test]
fn embedded_catalog_is_complete_and_consistent() {
    let catalog = content::load_catalog().expect("embedded curriculum must load");

    // The original three tracks keep their canonical head position.
    let track_ids: Vec<&str> = catalog.tracks.iter().map(|t| t.id.as_str()).collect();
    assert_eq!(
        &track_ids[..3],
        &["rust", "vue", "tauri"],
        "core tracks lead the catalog in stable order"
    );

    // Every declared track ships content — no silently missing tracks.
    for decl in content::TRACKS {
        assert!(
            track_ids.contains(&decl.id),
            "declared track '{}' has no content",
            decl.id
        );
    }
    assert_eq!(track_ids.len(), content::TRACKS.len(), "no undeclared tracks");

    // Core structural invariants for every track.
    for track in &catalog.tracks {
        assert!(!track.modules.is_empty(), "{} has modules", track.id);
        assert!(!track.category.is_empty(), "{} has a category", track.id);
        for module in &track.modules {
            assert!(
                !module.lessons.is_empty(),
                "module {} must contain lessons",
                module.id
            );
        }
    }

    // Global uniqueness of lesson ids.
    let mut lesson_ids = HashSet::new();
    for track in &catalog.tracks {
        for module in &track.modules {
            for lesson in &module.lessons {
                assert!(lesson_ids.insert(lesson.id.as_str()), "duplicate {}", lesson.id);
            }
        }
    }
}

#[test]
fn declarative_gates_match_the_curriculum_contract() {
    let catalog = content::load_catalog().unwrap();

    let find_module = |id: &str| {
        catalog
            .tracks
            .iter()
            .flat_map(|t| t.modules.iter())
            .find(|m| m.id == id)
            .unwrap_or_else(|| panic!("module {id} missing"))
    };

    // ---- Original contract (must never regress) ---------------------------
    // Tauri unlocks only after Rust Module 4.
    assert!(
        find_module("tauri-01").prerequisites.contains(&"rust-04".to_string()),
        "tauri-01 must require rust-04"
    );

    // Vue stays independent of Rust/Tauri.
    for module in &catalog.tracks.iter().find(|t| t.id == "vue").unwrap().modules {
        assert!(
            module.prerequisites.iter().all(|p| p.starts_with("vue-")),
            "vue module {} must not depend on other tracks",
            module.id
        );
    }

    // Capstone requires Rust-M6 ∧ Vue-M4 ∧ Tauri-M3.
    let capstone = content::load_capstone().unwrap();
    let mut prereqs = capstone.prerequisites.clone();
    prereqs.sort();
    assert_eq!(prereqs, vec!["rust-06", "tauri-03", "vue-04"]);

    // ---- Expansion contract (specialization graph, docs/prerequisite-model.md)
    // Foundations are the shared substrate; specialists build on them.
    assert!(
        find_module("perf-01").prerequisites.contains(&"http-01".to_string()),
        "performance entry requires HTTP fundamentals"
    );
    assert!(
        find_module("sec-01").prerequisites.contains(&"http-01".to_string()),
        "security entry requires HTTP fundamentals"
    );
    assert!(
        find_module("sec-02").prerequisites.contains(&"http-03".to_string()),
        "browser security builds on the HTTP browser-model module"
    );
    let brust = find_module("brust-01");
    assert!(brust.prerequisites.contains(&"rust-04".to_string()));
    assert!(brust.prerequisites.contains(&"http-01".to_string()));
    assert!(
        find_module("express-01").prerequisites.contains(&"http-01".to_string()),
        "express entry requires HTTP fundamentals"
    );
    assert!(
        find_module("express-06").prerequisites.contains(&"test-01".to_string()),
        "production express practices require the testing foundation"
    );
    assert!(
        find_module("varch-01").prerequisites.contains(&"vue-01".to_string()),
        "vue architecture builds on vue fundamentals"
    );
    assert!(
        find_module("a11y-01").prerequisites.contains(&"vue-01".to_string()),
        "accessible vue builds on vue fundamentals"
    );
    assert!(
        find_module("api-01").prerequisites.contains(&"http-01".to_string()),
        "api design requires HTTP semantics"
    );
    assert!(
        find_module("devops-01").prerequisites.contains(&"linux-01".to_string()),
        "containers require linux fundamentals"
    );
}

#[test]
fn end_to_end_unlock_flow_over_real_content() {
    let catalog = content::load_catalog().unwrap();
    let capstone = content::load_capstone().unwrap();

    let all_keys: Vec<CompletedLesson> = catalog
        .tracks
        .iter()
        .flat_map(|t| t.modules.iter())
        .flat_map(|m| m.lessons.iter())
        .map(|l| CompletedLesson {
            track_id: l.track_id.clone(),
            module_id: l.module_id.clone(),
            lesson_id: l.id.clone(),
            completed_at: 0,
        })
        .collect();

    // Everything done → capstone open, no locks anywhere.
    let summary = engine::summarize(&catalog, &all_keys, &capstone.prerequisites);
    assert!(summary.capstone_unlocked);
    assert!(summary.modules.iter().all(|m| !m.locked && m.completed));
    assert_eq!(summary.track_percent.get("rust"), Some(&100));
    assert!(summary.next.is_none());

    // Drop one Tauri-3 lesson → its module reopens and the capstone closes.
    let missing_lesson: Vec<CompletedLesson> = all_keys
        .iter()
        .filter(|c| !(c.module_id == "tauri-03" && c.lesson_id.ends_with("-l01")))
        .cloned()
        .collect();
    let summary = engine::summarize(&catalog, &missing_lesson, &capstone.prerequisites);
    assert!(!summary.capstone_unlocked);
}

#[test]
fn skill_graph_matches_catalog_and_progress() {
    let catalog = content::load_catalog().unwrap();
    let capstone = content::load_capstone().unwrap();
    let graph = content::load_skill_graph().expect("skill graph ships with content");

    // Skill ids referenced by content must exist in the graph.
    let graph_ids: HashSet<&str> = graph.skills.iter().map(|s| s.id.as_str()).collect();
    for track in &catalog.tracks {
        for module in &track.modules {
            for skill in &module.skills {
                assert!(graph_ids.contains(skill.as_str()), "module {} references unknown skill {}", module.id, skill);
            }
            for lesson in &module.lessons {
                for skill in &lesson.skills {
                    assert!(graph_ids.contains(skill.as_str()), "lesson {} references unknown skill {}", lesson.id, skill);
                }
            }
        }
    }

    // Dependencies must reference existing skills (no dangling edges).
    for skill in &graph.skills {
        for dep in &skill.requires {
            assert!(graph_ids.contains(dep.as_str()), "skill {} requires unknown {}", skill.id, dep);
        }
    }

    // Full completion demonstrates every evidence-backed skill.
    let all_keys: Vec<CompletedLesson> = catalog
        .tracks
        .iter()
        .flat_map(|t| t.modules.iter())
        .flat_map(|m| m.lessons.iter())
        .map(|l| CompletedLesson {
            track_id: l.track_id.clone(),
            module_id: l.module_id.clone(),
            lesson_id: l.id.clone(),
            completed_at: 0,
        })
        .collect();
    let summary = engine::summarize_full(
        &catalog,
        &all_keys,
        &capstone.prerequisites,
        Some(&graph),
        &[],
        &[],
        &[],
        &[],
    );
    let evidence_backed = graph
        .skills
        .iter()
        .filter(|s| {
            summary
                .skills
                .iter()
                .find(|st| st.id == s.id)
                .map(|st| st.lessons_total > 0 || st.modules_total > 0)
                .unwrap_or(false)
        })
        .count();
    let demonstrated = summary.skills.iter().filter(|s| s.demonstrated).count();
    assert_eq!(
        demonstrated, evidence_backed,
        "full completion demonstrates every evidence-backed skill"
    );
    // Every recommendation is explainable (non-empty reason).
    for rec in &summary.recommendations {
        assert!(!rec.reason.is_empty(), "recommendations must explain themselves");
    }
}

#[test]
fn project_specs_are_actionable() {
    let projects = content::load_projects().unwrap();
    assert!(!projects.is_empty(), "at least one project spec exists");
    for project in &projects {
        assert!(!project.requirements.is_empty(), "{} has requirements", project.id);
        assert!(!project.milestones.is_empty(), "{} has milestones", project.id);
        assert!(!project.hints.is_empty(), "{} provides hints", project.id);
        assert!(!project.completion_criteria.is_empty(), "{} defines completion criteria", project.id);
    }
}
