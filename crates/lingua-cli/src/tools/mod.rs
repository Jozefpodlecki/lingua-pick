use std::collections::BTreeMap;

use lingua_ai::{Tool, ToolCall};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    seed::SeedResult,
    types::{Concept, ExerciseRequest, GenerationResult},
};

mod context;
mod definitions;

const MAX_RESULT_BYTES: usize = 64 * 1024;

pub struct GenerationTools {
    context: context::Context,
    definitions: Vec<Tool>,
    validators: BTreeMap<String, jsonschema::Validator>,
    retrieved: BTreeMap<Uuid, Concept>,
}

pub struct ToolOutput {
    pub value: Value,
    pub is_error: bool,
}

impl GenerationTools {
    pub fn new(
        stores: &SeedResult,
        request: &ExerciseRequest,
        session_id: Uuid,
    ) -> Result<Self, String> {
        let definitions = definitions::definitions();
        let mut validators = BTreeMap::new();
        for tool in &definitions {
            if let Tool::Function(function) = tool {
                let validator = jsonschema::draft202012::options()
                    .should_validate_formats(true)
                    .build(&function.input_schema)
                    .map_err(|error| error.to_string())?;
                validators.insert(function.name.clone(), validator);
            }
        }
        Ok(Self {
            context: context::Context::new(stores, request, session_id),
            definitions,
            validators,
            retrieved: BTreeMap::new(),
        })
    }

    pub fn definitions(&self) -> Vec<Tool> {
        self.definitions.clone()
    }

    pub fn retrieved_concepts(&self) -> Vec<Concept> {
        self.retrieved.values().cloned().collect()
    }

    pub async fn execute(&mut self, call: ToolCall) -> ToolOutput {
        match self.execute_checked(call).await {
            Ok(value) => ToolOutput {
                value,
                is_error: false,
            },
            Err(error) => ToolOutput {
                value: json!({"error":error}),
                is_error: true,
            },
        }
    }

    async fn execute_checked(&mut self, call: ToolCall) -> Result<Value, String> {
        if call.invalid == Some(true) || call.provider_executed == Some(true) {
            return Err("invalid or provider-executed store tool call".into());
        }
        let validator = self
            .validators
            .get(&call.tool_name)
            .ok_or_else(|| "unknown tool name".to_string())?;
        let errors: Vec<_> = validator
            .iter_errors(&call.input)
            .take(3)
            .map(|error| error.to_string())
            .collect();
        if !errors.is_empty() {
            return Err(errors.join("; "));
        }
        let context = self.context.clone();
        let output = tokio::task::spawn_blocking(move || context.execute(call))
            .await
            .map_err(|error| error.to_string())?
            .map_err(|error| error.to_string())?;
        if serde_json::to_vec(&output.value)
            .map_err(|error| error.to_string())?
            .len()
            > MAX_RESULT_BYTES
        {
            return Err("tool result exceeds 64 KiB; request fewer items".into());
        }
        for concept in output.concepts {
            self.retrieved.insert(concept.id, concept);
        }
        Ok(output.value)
    }

    pub async fn validate_new_codes(&self, result: &GenerationResult) -> Result<(), String> {
        let context = self.context.clone();
        let codes: Vec<_> = result
            .concepts
            .iter()
            .filter(|concept| !self.retrieved.contains_key(&concept.id))
            .map(|concept| concept.code.clone())
            .collect();
        tokio::task::spawn_blocking(move || {
            for code in codes {
                if context
                    .code_exists(&code)
                    .map_err(|error| error.to_string())?
                {
                    return Err(format!("new concept code already exists in target: {code}"));
                }
            }
            Ok(())
        })
        .await
        .map_err(|error| error.to_string())?
    }
}
