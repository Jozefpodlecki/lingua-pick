use alloc::string::String;
use serde::{Deserialize, Serialize};

/// Stable target identifier, including a regional variety where applicable.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageId(pub String);
