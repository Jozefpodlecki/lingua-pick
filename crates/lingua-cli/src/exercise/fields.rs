use serde_json::Value;

use super::Result;
use crate::exercise::ContractError;

pub(super) fn required<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a Value> {
    value.get(field).ok_or_else(|| ContractError::MissingField {
        field: field.into(),
    })
}

pub(super) fn required_array<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a [Value]> {
    required(value, field)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| ContractError::InvalidFieldType {
            field: field.into(),
            expected: "an array",
        })
}

pub(super) fn required_string<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a str> {
    required(value, field)?
        .as_str()
        .ok_or_else(|| ContractError::InvalidFieldType {
            field: field.into(),
            expected: "a string",
        })
}