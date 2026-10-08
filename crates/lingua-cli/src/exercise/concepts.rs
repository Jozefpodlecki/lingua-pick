use std::collections::BTreeSet;

use serde_json::Value;
use uuid::Uuid;

use super::Result;
use crate::exercise::ContractError;

pub fn concept_ids(value: &Value) -> Result<BTreeSet<Uuid>> {
    let mut ids = BTreeSet::new();
    collect_concepts(value, &mut ids)?;
    Ok(ids)
}

fn collect_concepts(
    value: &Value,
    ids: &mut BTreeSet<Uuid>,
) -> Result<()> {
    match value {
        Value::Object(fields) => {
            for (key, value) in fields {
                match key.as_str() {
                    "concept_id" => {
                        ids.insert(parse_id(value)?);
                    }
                    "concept_ids" => {
                        let items = value.as_array().ok_or_else(|| {
                            ContractError::InvalidFieldType {
                                field: "concept_ids".into(),
                                expected: "an array",
                            }
                        })?;

                        for item in items {
                            ids.insert(parse_id(item)?);
                        }
                    }
                    _ => collect_concepts(value, ids)?,
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_concepts(item, ids)?;
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn parse_id(value: &Value) -> Result<Uuid> {
    let text = value.as_str().ok_or_else(|| {
        ContractError::InvalidFieldType {
            field: "concept_id".into(),
            expected: "a UUID string",
        }
    })?;

    Uuid::parse_str(text).map_err(|_| ContractError::InvalidUuid {
        field: "concept_id".into(),
    })
}