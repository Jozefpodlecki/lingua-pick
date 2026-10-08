use duckdb::OptionalExt;

use crate::{LinguaStore, Result, StoreError, TargetStats};

pub struct StatsRepository<'a> {
    store: &'a LinguaStore,
}

impl<'a> StatsRepository<'a> {
    pub(crate) const fn new(store: &'a LinguaStore) -> Self {
        Self { store }
    }

    pub fn for_target(&self, target_language_id: &str) -> Result<Option<TargetStats>> {
        if target_language_id.trim().is_empty() {
            return Err(StoreError::InvalidRecord(String::from(
                "Target language identifier must not be blank.",
            )));
        }

        let connection = self.store.connection()?;
        let raw = connection
            .query_row(
                r#"
                SELECT
                    target_language_id,
                    sessions_started,
                    sessions_completed,
                    answers_recorded,
                    graded_answers,
                    correct_answers,
                    accuracy
                FROM target_stats
                WHERE target_language_id = ?
                "#,
                [target_language_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, i64>(2)?,
                        row.get::<_, i64>(3)?,
                        row.get::<_, i64>(4)?,
                        row.get::<_, i64>(5)?,
                        row.get::<_, Option<f64>>(6)?,
                    ))
                },
            )
            .optional()?;

        raw.map(|raw| {
            Ok(TargetStats {
                target_language_id: raw.0,
                sessions_started: to_u64(raw.1)?,
                sessions_completed: to_u64(raw.2)?,
                answers_recorded: to_u64(raw.3)?,
                graded_answers: to_u64(raw.4)?,
                correct_answers: to_u64(raw.5)?,
                accuracy: raw.6,
            })
        })
        .transpose()
    }
}

fn to_u64(value: i64) -> Result<u64> {
    u64::try_from(value).map_err(|_| {
        StoreError::InvalidRecord(String::from("Stored statistic cannot be negative."))
    })
}
