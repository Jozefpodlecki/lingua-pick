
mod definitions;
mod error;
mod handler;
mod store;

use std::collections::BTreeMap;

use lingua_ai::{Tool, ToolCall};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    seed::SeedResult,
    types::{Concept, ExerciseRequest, GenerationResult},
};

pub use error::ToolError;

const MAX_RESULT_BYTES: usize = 64 * 1024;
const MAX_VALIDATION_ERRORS: usize = 3;

type Result<T> = std::result::Result<T, ToolError>;

pub struct GenerationTools {
    handler: store::StoreToolHandler,
    definitions: Vec<Tool>,
    validators: BTreeMap<String, jsonschema::Validator>,
    retrieved: BTreeMap<Uuid, Concept>,
}

pub struct ToolOutput {
    pub value: Value,
    pub is_error: bool,
}

impl ToolOutput {
    fn success(value: Value) -> Self {
        Self {
            value,
            is_error: false,
        }
    }

    fn error(error: ToolError) -> Self {
        Self {
            value: json!({ "error": error.to_string() }),
            is_error: true,
        }
    }
}

impl GenerationTools {
    pub fn new(
        stores: &SeedResult,
        request: &ExerciseRequest,
        session_id: Uuid,
    ) -> Result<Self> {
        let definitions = definitions::definitions();
        let validators = compile_validators(&definitions)?;

        Ok(Self {
            handler: store::StoreToolHandler::new(
                stores,
                request,
                session_id,
            ),
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
            Ok(value) => ToolOutput::success(value),
            Err(error) => ToolOutput::error(error),
        }
    }

    async fn execute_checked(&mut self, call: ToolCall) -> Result<Value> {
        self.validate_call(&call)?;

        let request = handler::ToolRequest::parse(call)?;
        let handler = self.handler.clone();

        let started = std::time::Instant::now();

        let output = tokio::task::spawn_blocking(move || {
            handler::ToolHandler::execute(&handler, request)
        })
        .await??;

        validate_result_size(&output.value)?;

        let concept_count = output.concepts.len();

        for concept in output.concepts {
            self.retrieved.insert(concept.id, concept);
        }

        tracing::debug!(
            elapsed_ms = started.elapsed().as_millis(),
            concepts_retrieved = concept_count,
            result_bytes = output.value.to_string().len(),
            "Tool execution completed"
        );

        Ok(output.value)
    }

    fn validate_call(&self, call: &ToolCall) -> Result<()> {
        if call.invalid == Some(true) {
            return Err(ToolError::InvalidCall);
        }

        if call.provider_executed == Some(true) {
            return Err(ToolError::ProviderExecuted);
        }

        let validator = self
            .validators
            .get(&call.tool_name)
            .ok_or_else(|| {
                ToolError::UnknownTool(call.tool_name.clone())
            })?;

        let errors: Vec<_> = validator
            .iter_errors(&call.input)
            .take(MAX_VALIDATION_ERRORS)
            .map(|error| error.to_string())
            .collect();

        if !errors.is_empty() {
            return Err(ToolError::Validation {
                tool: call.tool_name.clone(),
                message: errors.join("; "),
            });
        }

        Ok(())
    }

    pub async fn validate_new_codes(
        &self,
        result: &GenerationResult,
    ) -> Result<()> {
        let codes: Vec<_> = result
            .concepts
            .iter()
            .filter(|concept| !self.retrieved.contains_key(&concept.id))
            .map(|concept| concept.code.clone())
            .collect();

        if codes.is_empty() {
            return Ok(());
        }

        let handler = self.handler.clone();

        tokio::task::spawn_blocking(move || {
            for code in codes {
                if handler.code_exists(&code)? {
                    return Err(ToolError::DuplicateConceptCode(code));
                }
            }

            Ok(())
        })
        .await?
    }
}

fn compile_validators(
    definitions: &[Tool],
) -> Result<BTreeMap<String, jsonschema::Validator>> {
    let mut validators = BTreeMap::new();

    for definition in definitions {
        let Tool::Function(function) = definition else {
            return Err(ToolError::Definition(
                "generation tools must be function tools".into(),
            ));
        };

        let validator = jsonschema::draft202012::options()
            .should_validate_formats(true)
            .build(&function.input_schema)
            .map_err(|error| ToolError::Schema {
                tool: function.name.clone(),
                message: error.to_string(),
            })?;

        if validators
            .insert(function.name.clone(), validator)
            .is_some()
        {
            return Err(ToolError::Definition(format!(
                "duplicate tool definition: {}",
                function.name
            )));
        }
    }

    Ok(validators)
}

fn validate_result_size(value: &Value) -> Result<()> {
    let size = serde_json::to_vec(value)?.len();

    if size > MAX_RESULT_BYTES {
        return Err(ToolError::ResultTooLarge {
            limit: MAX_RESULT_BYTES,
        });
    }

    Ok(())
}
