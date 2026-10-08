
use lingua_ai::ToolCall;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    store::StoreError, tools::ToolError, types::Concept,
};

pub struct ToolOutput {
    pub value: Value,
    pub concepts: Vec<Concept>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchArgs {
    pub query: String,
    pub skill_id: Option<String>,

    #[serde(default = "default_limit")]
    pub limit: u32,

    #[serde(default)]
    pub offset: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptIdsArgs {
    pub concept_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PageArgs {
    #[serde(default = "default_limit_usize")]
    pub limit: usize,

    #[serde(default)]
    pub offset: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecentArgs {
    #[serde(default = "default_recent_limit")]
    pub limit: usize,
}

fn default_limit() -> u32 {
    10
}

fn default_limit_usize() -> usize {
    10
}

fn default_recent_limit() -> usize {
    5
}

pub enum ToolRequest {
    SearchConcepts(SearchArgs),
    GetConcepts(ConceptIdsArgs),
    GetLearnerEvidence(ConceptIdsArgs),
    ListSkills(PageArgs),
    ListTopics(PageArgs),
    GetTeachingContext,
    GetRecentExercises(RecentArgs),
}

impl ToolRequest {
    pub fn parse(call: ToolCall) -> Result<Self, ToolError> {
        let input = call.input;

        match call.tool_name.as_str() {
            "search_concepts" => Ok(Self::SearchConcepts(
                serde_json::from_value(input)?
            )),

            "get_concepts" => Ok(Self::GetConcepts(
                serde_json::from_value(input)?
            )),

            "get_learner_evidence" => Ok(Self::GetLearnerEvidence(
                serde_json::from_value(input)?
            )),

            "list_skills" => Ok(Self::ListSkills(
                serde_json::from_value(input)?
            )),

            "list_topics" => Ok(Self::ListTopics(
                serde_json::from_value(input)?
            )),

            "get_teaching_context" => {
                let _: EmptyArgs = serde_json::from_value(input)?;
                Ok(Self::GetTeachingContext)
            }

            "get_recent_exercises" => Ok(Self::GetRecentExercises(
                serde_json::from_value(input)?
            )),

            _ => Err(ToolError::Invalid("unknown tool name".into())),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EmptyArgs {}

pub trait ToolHandler: Clone + Send + 'static {
    fn execute(
        &self,
        request: ToolRequest,
    ) -> Result<ToolOutput, ToolError>;
}
