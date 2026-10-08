use super::{Runner, RunnerError};
use crate::{
    evaluator::{EvaluationRequest, EvaluationResult}, exercise, simulator::{SimulationPlan, SimulationRequest}, types::{Concept, Exercise, ExerciseConcept, ExerciseRequest, GenerationResult, Session, User},
};

struct PreparedAnswer {
    answer: Option<serde_json::Value>,
    plan: Option<SimulationPlan>,
}

impl Runner {
    #[tracing::instrument(skip_all, err, fields(exercise_id = tracing::field::Empty, kind = tracing::field::Empty))]
    pub(super) async fn run_exercise(
        &self,
        user: &User,
        session: &Session,
    ) -> Result<(), RunnerError> {
        let request = self.load_context(user, session)?;
        let (exercise, concepts) = self.select_exercise(session, &request).await?;
        tracing::Span::current().record("exercise_id", tracing::field::display(exercise.id));
        tracing::Span::current().record("kind", &exercise.kind);
        let definition = self
            .stores
            .exercise_definition_store
            .get_by_id(exercise.definition_id)?
            .ok_or(RunnerError::InvalidState("exercise definition is missing"))?;
        let answer = self
            .prepare_answer(user, session, &exercise, &definition, &request)
            .await?;
        let mut evaluation = self
            .evaluator
            .evaluate(&EvaluationRequest {
                user_id: user.id,
                source_language: &user.source_language_id,
                target_language: &session.target_language_id,
                exercise: &exercise,
                definition: &definition,
                answer: answer.answer.as_ref(),
                concepts: &concepts,
                learning_stage: &request.learning_stage,
                teaching_guidelines: &request.teaching_guidelines,
            })
            .await?;
        self.complete(&exercise, answer, &mut evaluation)?;
        Ok(())
    }

    async fn select_exercise(
        &self,
        session: &Session,
        request: &ExerciseRequest,
    ) -> Result<(Exercise, Vec<Concept>), RunnerError> {
        if let Some(exercise) = self
            .stores
            .exercise_store
            .list(session.id)?
            .into_iter()
            .find(|e| e.answered_on.is_none())
        {
            let ids = exercise::concept_ids(&exercise.payload)?;
            let links = self.stores.exercise_concept_store.list(exercise.id)?;
            let linked: std::collections::BTreeSet<_> =
                links.iter().map(|l| l.concept_id).collect();
            if ids != linked {
                return Err(RunnerError::InvalidState(
                    "pending exercise concept links are incomplete",
                ));
            }
            let concepts = ids
                .into_iter()
                .map(|id| {
                    self.stores
                        .concept_store
                        .get(id)?
                        .ok_or(crate::store::StoreError::NotFound)
                })
                .collect::<Result<Vec<_>, _>>()?;
            tracing::info!(exercise_id = %exercise.id, kind = %exercise.kind, "Resuming pending exercise");
            return Ok((exercise, concepts));
        }
        let mut tools = crate::tools::GenerationTools::new(&self.stores, request, session.id)?;
        let generated = self.generator.generate(request, &mut tools).await?;
        self.persist_generated(session, generated)
    }

    fn persist_generated(
        &self,
        session: &Session,
        generated: GenerationResult,
    ) -> Result<(Exercise, Vec<Concept>), RunnerError> {
        for concept in &generated.concepts {
            if self.stores.concept_store.get(concept.id)?.is_none() {
                self.stores.concept_store.insert(concept)?;
            }
        }
        let exercise = Exercise::from_generated(session.id, generated.exercise);
        self.stores.exercise_store.insert(&exercise)?;
        for concept in &generated.concepts {
            self.stores
                .exercise_concept_store
                .insert(&ExerciseConcept {
                    exercise_id: exercise.id,
                    concept_id: concept.id,
                    is_primary: true,
                })?;
        }
        tracing::info!(exercise_id = %exercise.id, kind = %exercise.kind, "Generated exercise saved");
        Ok((exercise, generated.concepts))
    }

    async fn prepare_answer(
        &self,
        user: &User,
        session: &Session,
        exercise: &Exercise,
        definition: &crate::types::ExerciseDefinition,
        context: &ExerciseRequest,
    ) -> Result<PreparedAnswer, RunnerError> {
        if let Some(answer) = &exercise.answer {
            tracing::info!(exercise_id = %exercise.id, "Reusing saved answer");
            return Ok(PreparedAnswer {
                answer: Some(answer.clone()),
                plan: None,
            });
        }
        let simulation = self
            .simulator
            .answer(&SimulationRequest {
                user_id: user.id,
                source_language: &user.source_language_id,
                target_language: &session.target_language_id,
                exercise,
                definition,
                learner_evidence: &context.learner_evidence,
            })
            .await?;
        if let Some(answer) = &simulation.answer {
            self.stores.exercise_store.set_answer(exercise.id, answer)?;
        }
        tracing::debug!(exercise_id = %exercise.id, plan = %serde_json::to_string(&simulation.plan)?, "Simulation plan");
        Ok(PreparedAnswer {
            answer: simulation.answer,
            plan: Some(simulation.plan),
        })
    }

    fn complete(
        &self,
        exercise: &Exercise,
        answer: PreparedAnswer,
        evaluation: &mut EvaluationResult,
    ) -> Result<(), RunnerError> {
        if let Some(plan) = &answer.plan {
            plan.validate_results(&evaluation.concept_results)?;
        }
        let plan = answer.plan.as_ref().map(serde_json::to_value).transpose()?;
        for evidence in &mut evaluation.evidence {
            evidence.detail["simulation_plan"] = plan.clone().unwrap_or(serde_json::Value::Null);
            evidence.detail["resumed_saved_answer"] =
                serde_json::Value::Bool(answer.plan.is_none());
        }
        self.stores.exercise_store.complete_with_evidence(
            exercise.id,
            answer.answer.as_ref(),
            &evaluation.verdict,
            &evaluation.evidence,
        )?;
        tracing::info!(exercise_id = %exercise.id, kind = %exercise.kind, evidence = evaluation.evidence.len(), "Exercise completed");
        Ok(())
    }
}
