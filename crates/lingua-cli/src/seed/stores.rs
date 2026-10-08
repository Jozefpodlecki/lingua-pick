use duckdb_neo::r2d2::ConnectionManager;
use uuid::Uuid;

use super::state::State;
use crate::store::*;

pub struct SeedResult {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub lang_store: LanguageStore,
    pub user_store: UserStore,
    pub user_stats_store: UserStatsStore,
    pub exercise_store: ExerciseStore,
    pub exercise_definition_store: ExerciseDefinitionStore,
    pub session_store: SessionStore,
    pub exercise_category_store: ExerciseCategoryStore,
    pub concept_store: ConceptStore,
    pub exercise_concept_store: ExerciseConceptStore,
    pub exercise_turn_store: ExerciseTurnStore,
    pub skill_store: SkillStore,
    pub topic_store: TopicStore,
    pub user_concept_store: UserConceptStore,
    pub learning_stage_store: LearningStageStore,
    pub teaching_guideline_store: TeachingGuidelineStore,
    pub learning_evidence_store: LearningEvidenceStore,
    pub language_exercise_store: LanguageExerciseStore,
    pub script_store: ScriptStore,
    pub language_script_store: LanguageScriptStore,
    pub language_feature_store: LanguageFeatureStore,
    pub language_feature_value_store: LanguageFeatureValueStore,
    pub language_feature_assignment_store: LanguageFeatureAssignmentStore,
}

pub(super) fn resolve(pool: r2d2::Pool<ConnectionManager>, state: State) -> SeedResult {
    SeedResult {
        exercise_category_store: ExerciseCategoryStore::new(pool.clone()),
        concept_store: ConceptStore::new(pool.clone()),
        exercise_concept_store: ExerciseConceptStore::new(pool.clone()),
        exercise_turn_store: ExerciseTurnStore::new(pool.clone()),
        skill_store: SkillStore::new(pool.clone()),
        topic_store: TopicStore::new(pool.clone()),
        user_concept_store: UserConceptStore::new(pool.clone()),
        learning_stage_store: LearningStageStore::new(pool.clone()),
        teaching_guideline_store: TeachingGuidelineStore::new(pool.clone()),
        learning_evidence_store: LearningEvidenceStore::new(pool.clone()),
        script_store: ScriptStore::new(pool.clone()),
        language_script_store: LanguageScriptStore::new(pool.clone()),
        language_feature_store: LanguageFeatureStore::new(pool.clone()),
        language_feature_value_store: LanguageFeatureValueStore::new(pool.clone()),
        language_feature_assignment_store: LanguageFeatureAssignmentStore::new(pool.clone()),
        language_exercise_store: LanguageExerciseStore::new(pool.clone()),
        user_id: state.user_id,
        session_id: state.session_id,
        lang_store: LanguageStore::new(pool.clone()),
        user_store: UserStore::new(pool.clone()),
        user_stats_store: UserStatsStore::new(pool.clone()),
        exercise_store: ExerciseStore::new(pool.clone()),
        exercise_definition_store: ExerciseDefinitionStore::new(pool.clone()),
        session_store: SessionStore::new(pool.clone()),
    }
}
