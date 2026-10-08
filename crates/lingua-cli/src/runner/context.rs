use super::{Runner, RunnerError};
use crate::types::{ExerciseRequest, Session, User};

impl Runner {
    pub(super) fn load_context(
        &self,
        user: &User,
        session: &Session,
    ) -> Result<ExerciseRequest, RunnerError> {
        let stores = &self.stores;
        let user_stats = stores
            .user_stats_store
            .get(user.id, &session.target_language_id)?
            .ok_or(RunnerError::InvalidState("target statistics are missing"))?;
        let stage = stores
            .learning_stage_store
            .get(&self.config.stage)?
            .ok_or(RunnerError::InvalidState("teaching stage is missing"))?;
        let stage_definitions = stores.learning_stage_store.definitions(&stage.code)?;
        let mut available_exercises = stores
            .exercise_definition_store
            .get_for_language(&session.target_language_id)?;
        available_exercises.retain(|d| stage_definitions.iter().any(|s| s.id == d.id));
        if available_exercises.is_empty() {
            return Err(RunnerError::InvalidState(
                "no exercises are available for this target and stage",
            ));
        }
        if available_exercises.len() > 1 {
            if let Some(last) = stores.exercise_store.get_last_for_session(session.id)? {
                available_exercises.retain(|d| d.id != last.definition_id);
            }
        }
        let available_topics = stores
            .topic_store
            .list_for_stage(&stage.code)?
            .into_iter()
            .map(|association| {
                stores
                    .topic_store
                    .get(&association.topic_id)?
                    .ok_or(crate::store::StoreError::NotFound)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let request = ExerciseRequest {
            source_language: user.source_language_id.clone(),
            target_language: session.target_language_id.clone(),
            user_stats,
            available_exercises,
            available_scripts: stores
                .script_store
                .for_language(&session.target_language_id)?,
            learning_stage: stage,
            available_topics,
            teaching_guidelines: stores.teaching_guideline_store.list_all()?,
            available_skills: stores.skill_store.list()?,
            known_concepts: stores.concept_store.list(&session.target_language_id)?,
            learner_evidence: stores
                .learning_evidence_store
                .summarize(user.id, &session.target_language_id)?,
            recent_exercises: stores
                .exercise_store
                .list(session.id)?
                .into_iter()
                .rev()
                .take(5)
                .collect(),
        };
        tracing::debug!(stage = %request.learning_stage.code, definitions = request.available_exercises.len(),
            concepts = request.known_concepts.len(), evidence = request.learner_evidence.len(), "Learner context loaded");
        Ok(request)
    }
}
