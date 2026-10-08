use std::collections::{BTreeMap, BTreeSet};

use chrono::Utc;
use serde_json::{Value, json};
use uuid::Uuid;

use super::{EvaluationError, EvaluationRequest};
use crate::{
    exercise_contract as contract,
    types::{EvidenceMode, LearningEvidence},
};

pub(super) fn create(
    request: &EvaluationRequest<'_>,
    verdict: &Value,
    results: &BTreeMap<Uuid, bool>,
    ids: &BTreeSet<Uuid>,
    model: &str,
) -> Result<Vec<LearningEvidence>, EvaluationError> {
    let mode = evidence_mode(&request.exercise.kind)?;
    let occurred_on = Utc::now();
    let evaluator = if mode == EvidenceMode::Exposure {
        "application:dialogue-exposure".into()
    } else {
        format!("lmstudio:{model}")
    };
    Ok(ids.iter().map(|id| LearningEvidence {
        id: Uuid::now_v7(),
        user_id: request.user_id,
        target_language_id: request.target_language.into(),
        exercise_id: request.exercise.id,
        concept_id: *id,
        event_key: "synthetic-evaluation-v1".into(),
        evidence_mode: mode,
        correct: results.get(id).copied(),
        assisted: false,
        occurred_on,
        evaluator: evaluator.clone(),
        policy_version: "experimental-v1".into(),
        detail: json!({"synthetic": true, "exercise_kind": request.exercise.kind, "feedback": verdict.get("feedback")}),
    }).collect())
}

fn evidence_mode(kind: &str) -> Result<EvidenceMode, EvaluationError> {
    let mode = match kind {
        "dialogue" => EvidenceMode::Exposure,
        "match_words"
        | "single_choice"
        | "choose_word"
        | "meaning_in_context"
        | "choose_synonym"
        | "choose_antonym"
        | "choose_register"
        | "choose_collocation"
        | "choose_idiom_meaning" => EvidenceMode::Recognition,
        "recall_word" | "complete_spelling" | "transliterate" => EvidenceMode::Recall,
        "reading_question" | "translate_sentence" => EvidenceMode::Comprehension,
        "fill_blank"
        | "conjugate_verb"
        | "choose_article"
        | "choose_preposition"
        | "choose_pronoun"
        | "complete_agreement"
        | "form_plural"
        | "express_possession"
        | "negate_sentence"
        | "form_question"
        | "change_tense"
        | "make_comparison"
        | "complete_conditional"
        | "correct_sentence"
        | "rewrite_register"
        | "punctuate_sentence"
        | "order_words"
        | "build_sentence"
        | "translate_to_target"
        | "short_response" => EvidenceMode::Production,
        _ => return Err(contract::invalid("exercise kind has no evidence policy").into()),
    };
    Ok(mode)
}
