use duckdb_neo::{Parameters, connection::Connection};
use serde::de::DeserializeOwned;

use super::StoreError;

pub(super) fn has_rows(
    connection: &Connection,
    sql: &str,
    parameters: Parameters<'_>,
) -> Result<bool, StoreError> {
    let result = connection.query(sql, parameters)?;
    for chunk in result {
        if chunk?.row_count()? > 0 {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(super) fn read_ids(
    connection: &Connection,
    select: &str,
    parameters: Parameters<'_>,
) -> Result<Vec<uuid::Uuid>, StoreError> {
    #[derive(serde::Deserialize)]
    struct Identity {
        id: uuid::Uuid,
    }
    Ok(read_many::<Identity>(connection, select, parameters)?
        .into_iter()
        .map(|record| record.id)
        .collect())
}

pub(super) fn add_dependency(
    connection: &Connection,
    table: &str,
    subject_column: &str,
    prerequisite_column: &str,
    subject: &str,
    prerequisite: &str,
) -> Result<(), StoreError> {
    if subject == prerequisite {
        return Err(StoreError::InvalidInput(
            "a dependency cannot reference itself",
        ));
    }
    transaction(connection, || {
        let sql = queries::dependency_cycle(table, subject_column, prerequisite_column);

        if has_rows(
            connection,
            &sql,
            Parameters::positional(&[&prerequisite, &subject]),
        )? {
            return Err(StoreError::InvalidInput("dependency would create a cycle"));
        }
        let sql = queries::insert_dependency(table, subject_column, prerequisite_column);

        connection.execute(
            sql.as_str(),
            Parameters::positional(&[&subject, &prerequisite]),
        )?;
        Ok(())
    })
}

pub(super) fn read_many<T: DeserializeOwned>(
    connection: &Connection,
    select: &str,
    parameters: Parameters<'_>,
) -> Result<Vec<T>, StoreError> {
    let sql = queries::json_records(select);
    let result = connection.query(sql.as_str(), parameters)?;
    let mut records = Vec::new();

    for chunk in result {
        let chunk = chunk?;
        let values = chunk.get_vector_at::<String>(0)?;

        for row in 0..chunk.row_count()? {
            let value = values
                .get(row)?
                .ok_or(StoreError::InvalidRecord("null row"))?;
            records.push(serde_json::from_str(value)?);
        }
    }

    Ok(records)
}

pub(super) fn read_one<T: DeserializeOwned>(
    connection: &Connection,
    select: &str,
    parameters: Parameters<'_>,
) -> Result<Option<T>, StoreError> {
    let mut records = read_many(connection, select, parameters)?;

    if records.len() > 1 {
        return Err(StoreError::InvalidRecord("expected at most one row"));
    }

    Ok(records.pop())
}

pub(super) fn changed_one(count: usize) -> Result<(), StoreError> {
    match count {
        1 => Ok(()),
        0 => Err(StoreError::NotFound),
        _ => Err(StoreError::InvalidRecord(
            "expected exactly one changed row",
        )),
    }
}

pub(super) fn transaction<T>(
    connection: &Connection,
    operation: impl FnOnce() -> Result<T, StoreError>,
) -> Result<T, StoreError> {
    connection.execute(queries::BEGIN_TRANSACTION, Parameters::None)?;

    match operation() {
        Ok(value) => {
            if let Err(error) = connection.execute(queries::COMMIT, Parameters::None) {
                connection.execute(queries::ROLLBACK, Parameters::None)?;
                return Err(error.into());
            }

            Ok(value)
        }
        Err(error) => {
            connection.execute(queries::ROLLBACK, Parameters::None)?;
            Err(error)
        }
    }
}

mod queries {
    pub(super) const BEGIN_TRANSACTION: &str = "BEGIN TRANSACTION";
    pub(super) const COMMIT: &str = "COMMIT";
    pub(super) const ROLLBACK: &str = "ROLLBACK";

    pub(super) fn dependency_cycle(
        table: &str,
        subject_column: &str,
        prerequisite_column: &str,
    ) -> String {
        format!(
            r#"
            WITH RECURSIVE ancestors(id) AS
            (
                SELECT
                    {prerequisite_column}
                FROM {table}
                WHERE
                    {subject_column}::VARCHAR = $1

                UNION

                SELECT
                    dependency.{prerequisite_column}
                FROM {table} dependency
                JOIN ancestors ancestor
                    ON dependency.{subject_column} = ancestor.id
            )
            SELECT
                1
            FROM ancestors
            WHERE
                id::VARCHAR = $2
            "#
        )
    }

    pub(super) fn insert_dependency(
        table: &str,
        subject_column: &str,
        prerequisite_column: &str,
    ) -> String {
        format!(
            r#"
            INSERT INTO {table}
            (
                {subject_column},
                {prerequisite_column}
            )
            VALUES
            (
                $1,
                $2
            )
            "#
        )
    }

    pub(super) fn json_records(select: &str) -> String {
        format!(
            r#"
            SELECT
                to_json(record)::VARCHAR
            FROM
            (
                {select}
            ) record
            "#
        )
    }
}
