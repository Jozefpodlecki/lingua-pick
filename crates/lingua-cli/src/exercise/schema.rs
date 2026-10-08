use serde_json::Value;

use super::Result;
use crate::exercise::ContractError;

pub fn compile_schema(schema: &Value) -> Result<jsonschema::Validator> {
    jsonschema::draft202012::options()
        .should_validate_formats(true)
        .build(schema)
        .map_err(|error| ContractError::InvalidSchema {
            message: error.to_string(),
        })
}

pub fn validate_schema(
    validator: &jsonschema::Validator,
    value: &Value,
) -> Result<()> {
    let errors: Vec<_> = validator
        .iter_errors(value)
        .take(8)
        .map(|error| error.to_string())
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ContractError::SchemaViolation { errors })
    }
}

pub fn parse_response(
    response: &str,
    validator: &jsonschema::Validator,
) -> Result<Value> {
    const MAX_RESPONSE_BYTES: usize = 128 * 1024;

    if response.len() > MAX_RESPONSE_BYTES {
        return Err(ContractError::ResponseTooLarge {
            limit: MAX_RESPONSE_BYTES,
        });
    }

    let value = serde_json::from_str(response)?;
    validate_schema(validator, &value)?;

    Ok(value)
}