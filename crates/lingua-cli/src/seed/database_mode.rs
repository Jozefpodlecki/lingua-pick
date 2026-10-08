use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DatabaseMode {
    #[default]
    OpenOrCreate,
    ExistingOnly,
    Recreate,
    InMemory,
}

impl fmt::Display for DatabaseMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::OpenOrCreate => "open_or_create",
            Self::ExistingOnly => "existing_only",
            Self::Recreate => "recreate",
            Self::InMemory => "in_memory",
        })
    }
}
