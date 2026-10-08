use chrono::{Duration, Utc};
use serde_json::json;
use uuid::Uuid;

use super::*;
use crate::{migration::apply_migrations, seed, types::*};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn database() -> Result<r2d2::Pool<duckdb_neo::r2d2::ConnectionManager>, Box<dyn std::error::Error>>
{
    let pool = seed::create_db()?;
    apply_migrations(&*pool.get()?)?;
    LanguageStore::new(pool.clone()).insert_json(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../assets/languages.json"
    ))?;
    Ok(pool)
}

fn learner(
    pool: &r2d2::Pool<duckdb_neo::r2d2::ConnectionManager>,
    target: &str,
) -> Result<(User, Session), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let user = User {
        id: Uuid::now_v7(),
        source_language_id: "en-GB".into(),
        username: "store-test".into(),
        password_hash: "unused".into(),
        created_on: now,
        updated_on: now + Duration::seconds(7),
    };
    UserStore::new(pool.clone()).insert(&user)?;
    UserStatsStore::new(pool.clone()).insert(&UserStats {
        user_id: user.id,
        target_language_id: target.into(),
        created_on: now,
        updated_on: now,
        struggling_categories: vec!["vocabulary".into()],
    })?;
    let session = Session {
        id: Uuid::now_v7(),
        user_id: user.id,
        target_language_id: target.into(),
        created_on: now,
        exercise_count: 0,
        max_exercise_count: 10,
        last_exercise_id: None,
    };
    SessionStore::new(pool.clone()).insert(&session)?;
    Ok((user, session))
}

fn concept(
    pool: &r2d2::Pool<duckdb_neo::r2d2::ConnectionManager>,
    target: &str,
    code: &str,
) -> Result<Concept, Box<dyn std::error::Error>> {
    let model = Concept {
        id: Uuid::now_v7(),
        language_id: target.into(),
        skill_id: "word_recognition".into(),
        code: code.into(),
        name: code.into(),
        description: "Test-only concept".into(),
    };
    ConceptStore::new(pool.clone()).insert(&model)?;
    Ok(model)
}

fn exercise(
    pool: &r2d2::Pool<duckdb_neo::r2d2::ConnectionManager>,
    session: &Session,
    kind: &str,
) -> Result<Exercise, Box<dyn std::error::Error>> {
    let definition = ExerciseDefinitionStore::new(pool.clone())
        .get_by_kind(kind)?
        .ok_or("missing definition")?;
    let model = Exercise::from_generated(
        session.id,
        GeneratedExercise {
            definition_id: definition.id,
            kind: kind.into(),
            payload: json!({"test": ["quoted ' text", "中文"]}),
        },
    );
    ExerciseStore::new(pool.clone()).insert(&model)?;
    Ok(model)
}

fn evidence(
    user: &User,
    session: &Session,
    exercise: &Exercise,
    concept: &Concept,
    mode: EvidenceMode,
    correct: Option<bool>,
) -> LearningEvidence {
    LearningEvidence {
        id: Uuid::now_v7(),
        user_id: user.id,
        target_language_id: session.target_language_id.clone(),
        exercise_id: exercise.id,
        concept_id: concept.id,
        event_key: Uuid::now_v7().to_string(),
        evidence_mode: mode,
        correct,
        assisted: false,
        occurred_on: Utc::now(),
        evaluator: "store-test".into(),
        policy_version: "test-v1".into(),
        detail: json!({"note":"test"}),
    }
}

#[test]
fn pedagogical_catalogues_and_dependency_cycles() -> TestResult {
    let pool = database()?;
    let categories = ExerciseCategoryStore::new(pool.clone());
    assert!(categories.get("vocabulary")?.is_some());
    assert_eq!(categories.list()?.len(), 30);
    categories.insert(&ExerciseCategory {
        name: "test".into(),
        description: "Test category".into(),
    })?;
    categories.update(&ExerciseCategory {
        name: "test".into(),
        description: "Updated category".into(),
    })?;
    assert_eq!(
        categories
            .get("test")?
            .ok_or("missing category")?
            .description,
        "Updated category"
    );

    let stages = LearningStageStore::new(pool.clone());
    assert_eq!(stages.list()?.len(), 3);
    assert_eq!(stages.get("words")?.ok_or("missing stage")?.sequence, 1);
    assert_eq!(
        stages
            .definitions("words")?
            .iter()
            .map(|d| d.kind.as_str())
            .collect::<Vec<_>>(),
        vec!["match_words"]
    );
    assert_eq!(stages.prerequisites("grammar")?.len(), 1);
    assert_eq!(
        stages.list_skills("words")?[0].evidence_mode,
        EvidenceMode::Recognition
    );
    assert_eq!(stages.list_exercises("words")?.len(), 1);
    assert!(matches!(
        stages.add_dependency("words", "sentences"),
        Err(StoreError::InvalidInput(_))
    ));

    let skills = SkillStore::new(pool.clone());
    assert_eq!(skills.list()?.len(), 5);
    assert_eq!(skills.prerequisites("word_recall")?.len(), 1);
    assert!(matches!(
        skills.add_dependency("word_recognition", "word_recall"),
        Err(StoreError::InvalidInput(_))
    ));
    skills.insert(&Skill {
        id: "test".into(),
        name: "Test".into(),
        description: "Test skill".into(),
    })?;
    skills.update(&Skill {
        id: "test".into(),
        name: "Updated skill".into(),
        description: "Updated".into(),
    })?;
    assert_eq!(
        skills.get("test")?.ok_or("missing skill")?.name,
        "Updated skill"
    );

    let topics = TopicStore::new(pool.clone());
    assert_eq!(topics.list()?.len(), 6);
    assert_eq!(topics.list_for_stage("words")?.len(), 4);
    topics.insert(&Topic {
        id: "test".into(),
        name: "Test".into(),
        description: "Test topic".into(),
    })?;
    topics.update(&Topic {
        id: "test".into(),
        name: "Updated topic".into(),
        description: "Updated".into(),
    })?;
    assert_eq!(
        topics.get("test")?.ok_or("missing topic")?.name,
        "Updated topic"
    );
    topics.add_to_stage(&TopicStage {
        topic_id: "test".into(),
        stage_code: "words".into(),
        generation_instructions: "Test guidance".into(),
    })?;
    let guidelines = TeachingGuidelineStore::new(pool.clone());
    assert_eq!(guidelines.list("vocabulary")?.len(), 2);
    assert!(guidelines.get("unknown_knowledge")?.is_some());
    guidelines.insert(&TeachingGuideline {
        code: "test".into(),
        category: "test".into(),
        generation_instructions: "Generate".into(),
        evaluation_instructions: "Evaluate".into(),
    })?;
    guidelines.update(&TeachingGuideline {
        code: "test".into(),
        category: "test".into(),
        generation_instructions: "Updated guidance".into(),
        evaluation_instructions: "Evaluate".into(),
    })?;
    assert_eq!(
        guidelines
            .get("test")?
            .ok_or("missing guideline")?
            .generation_instructions,
        "Updated guidance"
    );
    let mut words = stages.get("words")?.ok_or("missing stage")?;
    words.generation_instructions = "Updated stage guidance".into();
    stages.update(&words)?;
    assert_eq!(stages.get("words")?.ok_or("missing stage")?, words);
    assert_eq!(apply_migrations(&*pool.get()?)?, 0);
    Ok(())
}

#[test]
fn existing_stores_preserve_fields_and_report_missing_updates() -> TestResult {
    let pool = database()?;
    let (mut user, session) = learner(&pool, "pt-BR")?;
    let users = UserStore::new(pool.clone());
    let saved = users.get_by_id(user.id)?.ok_or("missing user")?;
    assert_eq!((saved.updated_on - saved.created_on).num_seconds(), 7);
    user.username = "changed".into();
    users.update(&user)?;
    assert_eq!(
        users.get_by_id(user.id)?.ok_or("missing user")?.username,
        "changed"
    );
    user.id = Uuid::now_v7();
    assert!(matches!(users.update(&user), Err(StoreError::NotFound)));
    assert!(users.get_by_id(user.id)?.is_none());

    let stats = UserStatsStore::new(pool.clone());
    let mut saved = stats.get(saved.id, "pt-BR")?.ok_or("missing stats")?;
    assert_eq!(saved.struggling_categories, vec!["vocabulary"]);
    saved.struggling_categories = vec!["grammar".into(), "spelling".into()];
    stats.update(&saved)?;
    assert_eq!(
        stats
            .get(saved.user_id, "pt-BR")?
            .ok_or("missing stats")?
            .struggling_categories,
        saved.struggling_categories
    );
    assert!(stats.get(saved.user_id, "pt-PT")?.is_none());
    pool.get()?.execute(
        "UPDATE user_stats SET struggling_categories = '{}' WHERE user_id = $1",
        duckdb_neo::Parameters::positional(&[&saved.user_id]),
    )?;
    assert!(matches!(
        stats.get(saved.user_id, "pt-BR"),
        Err(StoreError::Serialization(_))
    ));
    let sessions = SessionStore::new(pool.clone());
    assert_eq!(
        sessions
            .get_by_id(session.id)?
            .ok_or("missing session")?
            .max_exercise_count,
        10
    );
    assert_eq!(sessions.list(saved.user_id, "pt-BR")?.len(), 1);

    let definitions = ExerciseDefinitionStore::new(pool.clone());
    assert_eq!(definitions.list()?.len(), 34);
    let matching = definitions
        .get_by_kind("match_words")?
        .ok_or("missing matching")?;
    assert_eq!(matching.id.get_version_num(), 7);
    assert!(matching.schema.is_object());
    assert!(
        definitions
            .get_by_kind("dialogue")?
            .ok_or("missing dialogue")?
            .answer_schema
            .is_none()
    );
    definitions.add_to_language("pt-BR", matching.id)?;
    assert_eq!(definitions.get_for_language("pt-BR")?.len(), 1);
    definitions.add_skill(matching.id, "word_recognition")?;
    assert_eq!(definitions.skills(matching.id)?.len(), 1);
    let links = LanguageExerciseStore::new(pool.clone());
    assert!(links.exists("pt-BR", matching.id)?);
    assert_eq!(links.get_definition_ids("pt-BR")?, vec![matching.id]);
    links.delete("pt-BR", matching.id)?;
    assert!(!links.exists("pt-BR", matching.id)?);
    Ok(())
}

#[test]
fn concept_relationships_and_nullable_snapshots() -> TestResult {
    let pool = database()?;
    let (user, _) = learner(&pool, "pt-BR")?;
    let first = concept(&pool, "pt-BR", "test.first")?;
    let second = concept(&pool, "pt-BR", "test.second")?;
    let other = concept(&pool, "pt-PT", "test.first")?;
    let concepts = ConceptStore::new(pool.clone());
    assert_eq!(concepts.get(first.id)?.ok_or("missing concept")?, first);
    assert_eq!(
        concepts
            .get_by_code("pt-PT", "test.first")?
            .ok_or("missing concept")?
            .id,
        other.id
    );
    concepts.add_dependency(second.id, first.id)?;
    assert_eq!(concepts.prerequisite_ids(second.id)?, vec![first.id]);
    assert!(matches!(
        concepts.add_dependency(first.id, second.id),
        Err(StoreError::InvalidInput(_))
    ));
    assert!(matches!(
        concepts.add_dependency(first.id, other.id),
        Err(StoreError::InvalidInput(_))
    ));
    let snapshots = UserConceptStore::new(pool.clone());
    let snapshot = UserConcept {
        user_id: user.id,
        target_language_id: "pt-BR".into(),
        concept_id: first.id,
        attempts: 0,
        correct: 0,
        mastery: None,
        confidence: None,
        last_seen: None,
        updated_on: Utc::now(),
    };
    snapshots.insert(&snapshot)?;
    let saved = snapshots
        .get(user.id, "pt-BR", first.id)?
        .ok_or("missing snapshot")?;
    assert!(saved.mastery.is_none() && saved.confidence.is_none() && saved.last_seen.is_none());
    let mut wrong = snapshot.clone();
    wrong.concept_id = other.id;
    assert!(snapshots.insert(&wrong).is_err());
    Ok(())
}

#[test]
fn completion_is_atomic_and_progress_is_idempotent() -> TestResult {
    let pool = database()?;
    let (user, session) = learner(&pool, "pt-BR")?;
    let item = concept(&pool, "pt-BR", "test.recognition")?;
    let model = exercise(&pool, &session, "match_words")?;
    let links = ExerciseConceptStore::new(pool.clone());
    links.insert(&ExerciseConcept {
        exercise_id: model.id,
        concept_id: item.id,
        is_primary: true,
    })?;
    let exercises = ExerciseStore::new(pool.clone());
    assert_eq!(
        exercises
            .get_by_id(model.id)?
            .ok_or("missing exercise")?
            .payload,
        model.payload
    );
    assert!(exercises.get_last_for_session(session.id)?.is_some());
    let record = evidence(
        &user,
        &session,
        &model,
        &item,
        EvidenceMode::Recognition,
        Some(true),
    );
    let mut invalid = record.clone();
    invalid.id = Uuid::now_v7();
    invalid.event_key = "invalid".into();
    invalid.user_id = Uuid::now_v7();
    let answer = json!({"matches":[]});
    let verdict = json!({"feedback":"test"});
    assert!(
        exercises
            .complete_with_evidence(
                model.id,
                Some(&answer),
                &verdict,
                &[record.clone(), invalid]
            )
            .is_err()
    );
    let evidence_store = LearningEvidenceStore::new(pool.clone());
    assert!(evidence_store.list(user.id, "pt-BR")?.is_empty());
    assert!(
        exercises
            .get_by_id(model.id)?
            .ok_or("missing exercise")?
            .answered_on
            .is_none()
    );
    assert!(
        UserConceptStore::new(pool.clone())
            .list(user.id, "pt-BR")?
            .is_empty()
    );

    exercises.set_answer(model.id, &answer)?;
    exercises.complete_with_evidence(model.id, Some(&answer), &verdict, &[record.clone()])?;
    exercises.complete(model.id, &answer, &verdict)?;
    let sessions = SessionStore::new(pool.clone());
    sessions.refresh_progress(session.id)?;
    sessions.refresh_progress(session.id)?;
    let progress = sessions.get_by_id(session.id)?.ok_or("missing session")?;
    assert_eq!(progress.exercise_count, 1);
    assert_eq!(progress.last_exercise_id, Some(model.id));
    assert_eq!(
        evidence_store.summarize(user.id, "pt-BR")?[0].accuracy,
        Some(1.0)
    );
    assert_eq!(
        UserConceptStore::new(pool.clone())
            .get(user.id, "pt-BR", item.id)?
            .ok_or("missing snapshot")?
            .attempts,
        1
    );
    assert!(evidence_store.insert(&record).is_err());
    assert!(
        exercises
            .complete(model.id, &json!({"different":true}), &verdict)
            .is_err()
    );
    assert!(exercises.set_answer(model.id, &answer).is_err());
    Ok(())
}

#[test]
fn dialogue_exposure_assistance_and_turns_do_not_invent_accuracy() -> TestResult {
    let pool = database()?;
    let (user, session) = learner(&pool, "pt-BR")?;
    let item = concept(&pool, "pt-BR", "test.exposure")?;
    let mut model = exercise(&pool, &session, "dialogue")?;
    let links = ExerciseConceptStore::new(pool.clone());
    links.insert(&ExerciseConcept {
        exercise_id: model.id,
        concept_id: item.id,
        is_primary: false,
    })?;
    let turns = ExerciseTurnStore::new(pool.clone());
    let turn = ExerciseTurn {
        id: Uuid::now_v7(),
        exercise_id: model.id,
        turn_number: 1,
        role: TurnRole::Assistant,
        content: "Test ' text".into(),
        evaluation: None,
        created_on: Utc::now(),
    };
    turns.insert(&turn)?;
    assert!(
        turns
            .get(turn.id)?
            .ok_or("missing turn")?
            .evaluation
            .is_none()
    );
    turns.set_evaluation(turn.id, &json!({"note":"practice only"}))?;
    assert_eq!(
        turns.get(turn.id)?.ok_or("missing turn")?.evaluation,
        Some(json!({"note":"practice only"}))
    );
    assert_eq!(turns.list(model.id)?[0].turn_number, 1);
    assert!(turns.insert(&turn).is_err());
    let exposure = evidence(&user, &session, &model, &item, EvidenceMode::Exposure, None);
    let exercises = ExerciseStore::new(pool.clone());
    assert!(
        exercises
            .complete(model.id, &json!({}), &json!({}))
            .is_err()
    );
    exercises.complete_with_evidence(model.id, None, &json!({"completed":true}), &[exposure])?;
    assert!(turns.set_evaluation(turn.id, &json!({})).is_err());
    assert!(exercises.set_answer(model.id, &json!({})).is_err());
    let records = LearningEvidenceStore::new(pool.clone());
    let summary = &records.summarize(user.id, "pt-BR")?[0];
    assert_eq!((summary.assessed_attempts, summary.accuracy), (0, None));
    model = exercise(&pool, &session, "match_words")?;
    links.insert(&ExerciseConcept {
        exercise_id: model.id,
        concept_id: item.id,
        is_primary: true,
    })?;
    let mut assisted = evidence(
        &user,
        &session,
        &model,
        &item,
        EvidenceMode::Recognition,
        Some(true),
    );
    assisted.assisted = true;
    records.insert(&assisted)?;
    assert!(
        records
            .summarize(user.id, "pt-BR")?
            .iter()
            .all(|row| row.accuracy.is_none())
    );
    let wrong = concept(&pool, "pt-PT", "test.other")?;
    assert!(
        links
            .insert(&ExerciseConcept {
                exercise_id: model.id,
                concept_id: wrong.id,
                is_primary: true
            })
            .is_err()
    );
    Ok(())
}

#[test]
fn import_paths_are_data_and_invalid_batches_do_not_partially_write() -> TestResult {
    let pool = database()?;
    let languages = LanguageStore::new(pool.clone());
    let path = std::env::temp_dir().join(format!("lingua-import-'{}.json", Uuid::now_v7()));
    let contents = json!([
        {"id":"test-import","name":"Test","region":"Test","native_name":"Test"},
        {"id":"test-import","name":"Duplicate","region":"Test","native_name":"Test"}
    ]);
    std::fs::write(&path, serde_json::to_string(&contents)?)?;
    let result = languages.insert_json(path.to_str().ok_or("invalid path")?);
    std::fs::remove_file(&path)?;
    result?;
    assert_eq!(
        languages
            .get_by_id("test-import")?
            .ok_or("missing import")?
            .name,
        "Test"
    );
    let count = languages.get_all()?.len();
    languages.insert_json(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../assets/languages.json"
    ))?;
    assert_eq!(languages.get_all()?.len(), count);
    std::fs::write(
        &path,
        serde_json::to_string(&json!([
            {"id":"should-not-import","name":"Test","region":"Test","native_name":"Test"},
            {"id":"invalid-import","name":null,"region":"Test","native_name":"Test"}
        ]))?,
    )?;
    let result = languages.insert_json(path.to_str().ok_or("invalid path")?);
    std::fs::remove_file(&path)?;
    assert!(result.is_err());
    assert!(languages.get_by_id("should-not-import")?.is_none());
    Ok(())
}

#[test]
fn completion_cannot_overfill_a_session_and_rolls_back_outcome() -> TestResult {
    let pool = database()?;
    let (user, session) = learner(&pool, "pt-BR")?;
    pool.get()?.execute(
        "UPDATE session SET max_exercise_count = 1 WHERE id = $1",
        duckdb_neo::Parameters::positional(&[&session.id]),
    )?;
    let exercises = ExerciseStore::new(pool.clone());
    let first = exercise(&pool, &session, "match_words")?;
    exercises.complete(first.id, &json!({}), &json!({}))?;
    let next = exercise(&pool, &session, "match_words")?;
    assert!(exercises.complete(next.id, &json!({}), &json!({})).is_err());
    assert!(
        exercises
            .get_by_id(next.id)?
            .ok_or("missing exercise")?
            .answered_on
            .is_none()
    );
    assert_eq!(
        SessionStore::new(pool.clone()).list(user.id, "pt-BR")?[0].exercise_count,
        1
    );
    Ok(())
}

#[test]
fn seed_exposes_stores_and_only_registers_beginner_matching() -> TestResult {
    let seeded = seed::generate("store-test", "en-GB", "pt-BR")?;
    let definitions = seeded.exercise_definition_store.get_for_language("pt-BR")?;
    assert_eq!(
        definitions
            .iter()
            .map(|definition| definition.kind.as_str())
            .collect::<Vec<_>>(),
        vec!["match_words"]
    );
    assert_eq!(seeded.concept_store.list("pt-BR")?.len(), 100);
    assert!(
        seeded
            .learning_evidence_store
            .list(seeded.user_id, "pt-BR")?
            .is_empty()
    );
    assert_eq!(seeded.exercise_category_store.list()?.len(), 30);
    Ok(())
}
