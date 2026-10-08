use lingua_ai::ToolCall;
use serde::Deserialize;
use serde_json::{Value, json};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    seed::SeedResult,
    store::{ConceptStore, ExerciseStore, LearningEvidenceStore, SkillStore, StoreError},
    types::{Concept, ExerciseRequest, LearningStage, TeachingGuideline, Topic},
};

#[derive(Clone)]
pub(super) struct Context {
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

pub(super) struct Output {
    pub value: Value,
    pub concepts: Vec<Concept>,
}

#[derive(Debug, Error)]
pub(super) enum ToolError {
    #[error("invalid tool arguments: {0}")]
    Arguments(#[from] serde_json::Error),
    #[error("store lookup failed: {0}")]
    Store(#[from] StoreError),
    #[error("{0}")]
    Invalid(&'static str),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Search {
    query: String,
    skill_id: Option<String>,
    #[serde(default = "default_search_limit")]
    limit: u32,
    #[serde(default)]
    offset: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConceptIds {
    concept_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Page {
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default)]
    offset: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Recent {
    #[serde(default = "recent_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    10
}
fn default_search_limit() -> u32 {
    10
}
fn recent_limit() -> usize {
    5
}

impl Context {
    pub fn new(stores: &SeedResult, request: &ExerciseRequest, session_id: Uuid) -> Self {
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

    pub fn execute(&self, call: ToolCall) -> Result<Output, ToolError> {
        match call.tool_name.as_str() {
            "search_concepts" => {
                let args: Search = serde_json::from_value(call.input)?;
                let concepts = self.concepts.search(
                    &self.target,
                    &args.query,
                    args.skill_id.as_deref(),
                    args.limit,
                    args.offset,
                )?;
                let next_offset =
                    (concepts.len() == args.limit as usize).then_some(args.offset + args.limit);
                Ok(Output {
                    value: json!({"concepts":concepts, "next_offset":next_offset}),
                    concepts,
                })
            }
            "get_concepts" | "get_learner_evidence" => {
                let args: ConceptIds = serde_json::from_value(call.input)?;
                let mut concepts = Vec::new();
                let mut records = Vec::new();
                for id in args.concept_ids {
                    let concept = self.target_concept(id)?;
                    let record = if call.tool_name == "get_concepts" {
                        let prerequisites = self.concepts.prerequisite_ids(id)?;
                        if prerequisites.len() > 20 {
                            return Err(ToolError::Invalid(
                                "concept has too many prerequisites for one lookup",
                            ));
                        }
                        json!({"concept":concept, "prerequisite_ids":prerequisites})
                    } else {
                        let evidence =
                            self.evidence
                                .summarize_concept(self.user_id, &self.target, id)?;
                        json!({"concept":concept, "evidence":evidence})
                    };
                    concepts.push(concept);
                    records.push(record);
                }
                Ok(Output {
                    value: json!({"records":records}),
                    concepts,
                })
            }
            "list_skills" => {
                let args: Page = serde_json::from_value(call.input)?;
                let skills = self.skills.list()?;
                let next_offset =
                    (skills.len() > args.offset + args.limit).then_some(args.offset + args.limit);
                let records = skills
                    .into_iter()
                    .skip(args.offset)
                    .take(args.limit)
                    .map(|skill| {
                        let prerequisites = self.skills.prerequisites(&skill.id)?;
                        Ok(json!({"skill":skill, "prerequisites":prerequisites}))
                    })
                    .collect::<Result<Vec<_>, StoreError>>()?;
                Ok(Output {
                    value: json!({"skills":records, "next_offset":next_offset}),
                    concepts: vec![],
                })
            }
            "get_teaching_context" => Ok(Output {
                value: json!({"learning_stage":self.stage, "available_topics":self.topics, "teaching_guidelines":self.guidelines}),
                concepts: vec![],
            }),
            "get_recent_exercises" => {
                let args: Recent = serde_json::from_value(call.input)?;
                let exercises: Vec<_> = self
                    .exercises
                    .list(self.session_id)?
                    .into_iter()
                    .rev()
                    .take(args.limit)
                    .collect();
                Ok(Output {
                    value: json!({"exercises":exercises}),
                    concepts: vec![],
                })
            }
            _ => Err(ToolError::Invalid("unknown tool name")),
        }
    }

    fn target_concept(&self, id: Uuid) -> Result<Concept, ToolError> {
        self.concepts
            .get(id)?
            .filter(|concept| concept.language_id == self.target)
            .ok_or(ToolError::Invalid(
                "concept was not found in the current target",
            ))
    }

    pub fn code_exists(&self, code: &str) -> Result<bool, StoreError> {
        Ok(self.concepts.get_by_code(&self.target, code)?.is_some())
    }
}
