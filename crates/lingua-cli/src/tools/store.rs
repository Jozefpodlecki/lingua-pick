
use lingua_ai::ToolCall;
use serde_json::json;
use uuid::Uuid;

use crate::{
    seed::SeedResult, store::{
        ConceptStore,
        ExerciseStore,
        LearningEvidenceStore,
        SkillStore,
        StoreError,
    }, tools::ToolError, types::{
        Concept,
        ExerciseRequest,
        LearningStage,
        TeachingGuideline,
        Topic,
    },
};

use super::handler::{
    ConceptIdsArgs,
    PageArgs,
    RecentArgs,
    SearchArgs,
    ToolHandler,
    ToolOutput,
    ToolRequest,
};

#[derive(Clone)]
pub struct StoreToolHandler {
    user_id: Uuid,
    session_id: Uuid,
    target: String,

    concepts: ConceptStore,
    evidence: LearningEvidenceStore,
    skills: SkillStore,
    exercises: ExerciseStore,

    stage: LearningStage,
    topics: Vec<Topic>,
    guidelines: Vec<TeachingGuideline>,
}

impl StoreToolHandler {
    pub fn new(
        stores: &SeedResult,
        request: &ExerciseRequest,
        session_id: Uuid,
    ) -> Self {
        Self {
            user_id: request.user_stats.user_id,
            session_id,
            target: request.target_language.clone(),

            concepts: stores.concept_store.clone(),
            evidence: stores.learning_evidence_store.clone(),
            skills: stores.skill_store.clone(),
            exercises: stores.exercise_store.clone(),

            stage: request.learning_stage.clone(),
            topics: request.available_topics.clone(),
            guidelines: request.teaching_guidelines.clone(),
        }
    }

    fn search_concepts(
        &self,
        args: SearchArgs,
    ) -> Result<ToolOutput, ToolError> {
        let concepts = self.concepts.search(
            &self.target,
            &args.query,
            args.skill_id.as_deref(),
            args.limit,
            args.offset,
        )?;

        let next_offset = (concepts.len() == args.limit as usize)
            .then_some(args.offset + args.limit);

        Ok(ToolOutput {
            value: json!({
                "concepts": concepts,
                "next_offset": next_offset
            }),
            concepts,
        })
    }

    fn get_concepts(
        &self,
        args: ConceptIdsArgs,
    ) -> Result<ToolOutput, ToolError> {
        let mut concepts = Vec::new();
        let mut records = Vec::new();

        for id in args.concept_ids {
            let concept = self.target_concept(id)?;
            let prerequisites = self.concepts.prerequisite_ids(id)?;

            if prerequisites.len() > 20 {
                return Err(ToolError::Invalid(
                    "concept has too many prerequisites".into()
                ));
            }

            records.push(json!({
                "concept": concept,
                "prerequisite_ids": prerequisites
            }));

            concepts.push(concept);
        }

        Ok(ToolOutput {
            value: json!({ "records": records }),
            concepts,
        })
    }

    fn get_learner_evidence(
        &self,
        args: ConceptIdsArgs,
    ) -> Result<ToolOutput, ToolError> {
        let mut concepts = Vec::new();
        let mut records = Vec::new();

        for id in args.concept_ids {
            let concept = self.target_concept(id)?;

            let evidence = self.evidence.summarize_concept(
                self.user_id,
                &self.target,
                id,
            )?;

            records.push(json!({
                "concept": concept,
                "evidence": evidence
            }));

            concepts.push(concept);
        }

        Ok(ToolOutput {
            value: json!({ "records": records }),
            concepts,
        })
    }

    fn list_skills(
        &self,
        args: PageArgs,
    ) -> Result<ToolOutput, ToolError> {
        let skills = self.skills.list()?;

        let next_offset = (skills.len() > args.offset + args.limit)
            .then_some(args.offset + args.limit);

        let records = skills
            .into_iter()
            .skip(args.offset)
            .take(args.limit)
            .map(|skill| {
                let prerequisites =
                    self.skills.prerequisites(&skill.id)?;

                Ok(json!({
                    "skill": skill,
                    "prerequisites": prerequisites
                }))
            })
            .collect::<Result<Vec<_>, StoreError>>()?;

        Ok(ToolOutput {
            value: json!({
                "skills": records,
                "next_offset": next_offset
            }),
            concepts: vec![],
        })
    }

    fn list_topics(
        &self,
        args: PageArgs,
    ) -> Result<ToolOutput, ToolError> {
        let next_offset = (
            self.topics.len() > args.offset.saturating_add(args.limit)
        )
        .then_some(args.offset.saturating_add(args.limit));

        let topics: Vec<_> = self.topics
            .iter()
            .skip(args.offset)
            .take(args.limit)
            .collect();

        Ok(ToolOutput {
            value: json!({
                "topics": topics,
                "next_offset": next_offset
            }),
            concepts: vec![],
        })
    }

    fn get_teaching_context(&self) -> ToolOutput {
        ToolOutput {
            value: json!({
                "learning_stage": self.stage,
                "available_topics": self.topics,
                "teaching_guidelines": self.guidelines
            }),
            concepts: vec![],
        }
    }

    fn get_recent_exercises(
        &self,
        args: RecentArgs,
    ) -> Result<ToolOutput, ToolError> {
        let exercises: Vec<_> = self.exercises
            .list(self.session_id)?
            .into_iter()
            .rev()
            .take(args.limit)
            .collect();

        Ok(ToolOutput {
            value: json!({ "exercises": exercises }),
            concepts: vec![],
        })
    }

    fn target_concept(
        &self,
        id: Uuid,
    ) -> Result<Concept, ToolError> {
        self.concepts
            .get(id)?
            .filter(|concept| concept.language_id == self.target)
            .ok_or(ToolError::Invalid(
                "concept was not found in the current target".into()
            ))
    }

    pub fn code_exists(
        &self,
        code: &str,
    ) -> Result<bool, StoreError> {
        Ok(self
            .concepts
            .get_by_code(&self.target, code)?
            .is_some())
    }
}

impl ToolHandler for StoreToolHandler {
    fn execute(
        &self,
        request: ToolRequest,
    ) -> Result<ToolOutput, ToolError> {
        match request {
            ToolRequest::SearchConcepts(args) =>
                self.search_concepts(args),

            ToolRequest::GetConcepts(args) =>
                self.get_concepts(args),

            ToolRequest::GetLearnerEvidence(args) =>
                self.get_learner_evidence(args),

            ToolRequest::ListSkills(args) =>
                self.list_skills(args),

            ToolRequest::ListTopics(args) =>
                self.list_topics(args),

            ToolRequest::GetTeachingContext =>
                Ok(self.get_teaching_context()),

            ToolRequest::GetRecentExercises(args) =>
                self.get_recent_exercises(args),
        }
    }
}
