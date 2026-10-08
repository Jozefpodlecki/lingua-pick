
use lingua_ai::{FunctionTool, Tool};
use serde_json::{Value, json};

pub fn definitions() -> Vec<Tool> {
    vec![
        tool(
            "search_concepts",
            concat!(
                "Search stored concepts for the target language before ",
                "proposing new ones. An empty query browses concepts. ",
                "Results do not establish learner mastery. ",
                "Use pagination for additional results."
            ),
            with_pagination(json!({
                "query": {
                    "type": "string",
                    "maxLength": 256
                },
                "skill_id": {
                    "type": "string",
                    "maxLength": 128
                }
            })),
            &["query"],
        ),
        tool(
            "get_concepts",
            concat!(
                "Retrieve existing target-language concepts and their ",
                "prerequisite IDs. Retrieve prerequisite concepts ",
                "separately when their metadata is needed."
            ),
            json!({
                "concept_ids": concept_ids_schema()
            }),
            &["concept_ids"],
        ),
        tool(
            "get_learner_evidence",
            concat!(
                "Retrieve this learner's evidence for target concepts. ",
                "Missing evidence means unknown knowledge, ",
                "not failure or mastery."
            ),
            json!({
                "concept_ids": concept_ids_schema()
            }),
            &["concept_ids"],
        ),
        tool(
            "list_skills",
            concat!(
                "List valid skill IDs, names and prerequisite skills. ",
                "Use returned IDs for new concept metadata."
            ),
            pagination(),
            &[],
        ),
        tool(
            "list_topics",
            concat!(
                "List valid topic IDs, names and descriptions. ",
                "Use returned IDs for new concept metadata. ",
                "Consult get_teaching_context to determine ",
                "which topics are appropriate for the current stage."
            ),
            pagination(),
            &[],
        ),
        tool(
            "get_teaching_context",
            concat!(
                "Retrieve the fixed learning stage, eligible topics ",
                "and teaching guidelines. This tool cannot advance ",
                "the stage or enable additional exercise types."
            ),
            json!({}),
            &[],
        ),
        tool(
            "get_recent_exercises",
            concat!(
                "Retrieve recent exercises from this session ",
                "to avoid repetition. Retrieve referenced concept ",
                "metadata and learner evidence through their ",
                "respective tools."
            ),
            json!({
                "limit": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 5
                }
            }),
            &[],
        ),
    ]
}

fn concept_ids_schema() -> Value {
    json!({
        "type": "array",
        "items": {
            "type": "string",
            "format": "uuid"
        },
        "minItems": 1,
        "maxItems": 20,
        "uniqueItems": true
    })
}

fn pagination() -> Value {
    json!({
        "limit": {
            "type": "integer",
            "minimum": 1,
            "maximum": 20
        },
        "offset": {
            "type": "integer",
            "minimum": 0,
            "maximum": 10000
        }
    })
}

fn with_pagination(mut properties: Value) -> Value {
    if let (Some(target), Some(pagination)) = (
        properties.as_object_mut(),
        pagination().as_object(),
    ) {
        target.extend(pagination.clone());
    }

    properties
}

fn tool(
    name: &str,
    description: &str,
    properties: Value,
    required: &[&str],
) -> Tool {
    FunctionTool::new(
        name,
        json!({
            "type": "object",
            "properties": properties,
            "required": required,
            "additionalProperties": false
        }),
    )
    .with_description(description)
    .into()
}
