use lingua_ai::{FunctionTool, Tool};
use serde_json::{Value, json};

pub fn definitions() -> Vec<Tool> {
    let ids = json!({"type":"array", "items":{"type":"string", "format":"uuid"}, "minItems":1, "maxItems":20, "uniqueItems":true});
    let limit = json!({"type":"integer", "minimum":1, "maximum":20});
    let offset = json!({"type":"integer", "minimum":0, "maximum":10000});
    vec![
        tool(
            "search_concepts",
            "Search stored concepts for this target before proposing new ones. Empty query browses concepts. Results are not learner mastery. Use offset to page; maximum 20 per page.",
            json!({"query":{"type":"string", "maxLength":256}, "skill_id":{"type":"string", "maxLength":128}, "limit":limit, "offset":offset}),
            &["query"],
        ),
        tool(
            "get_concepts",
            "Read target concepts and prerequisite IDs. Retrieve prerequisite concepts separately if needed.",
            json!({"concept_ids":ids}),
            &["concept_ids"],
        ),
        tool(
            "get_learner_evidence",
            "Read this learner's evidence for target concepts. Empty evidence means unknown knowledge, not failure or mastery.",
            json!({"concept_ids":ids}),
            &["concept_ids"],
        ),
        tool(
            "list_skills",
            "Read valid skills with prerequisite skills. Use the returned skill IDs for new concept metadata.",
            json!({"limit":limit, "offset":offset}),
            &[],
        ),
        tool(
            "get_teaching_context",
            "Read the fixed current stage, its topics, and teaching guidelines. Tools cannot advance the stage or enable other exercise types.",
            json!({}),
            &[],
        ),
        tool(
            "get_recent_exercises",
            "Read up to five recent exercises in this session to avoid repetition. Returns their referenced concepts and evidence separately through concept/evidence tools.",
            json!({"limit":{"type":"integer", "minimum":1, "maximum":5}}),
            &[],
        ),
    ]
}

fn tool(name: &str, description: &str, properties: Value, required: &[&str]) -> Tool {
    FunctionTool::new(
        name,
        json!({
            "type":"object", "properties":properties, "required":required,
            "additionalProperties":false
        }),
    )
    .with_description(description)
    .into()
}
