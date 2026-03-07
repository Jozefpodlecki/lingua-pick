pub const GET_EXERCISE_BY_ID: &str = r#"
SELECT
    exc.*
FROM exercise exc
WHERE exc.id = ?
"#;

pub const INSERT_EXERCISE: &str = r#"
INSERT INTO exercise
VALUES
(?, ?, ?, ?, ?, ?, ?)
"#;

pub const GET_EXERCISE_TYPES: &str = r#"
WITH allowed AS (
    SELECT
        eat.type_id
    FROM exercise_allowed_type eat
    WHERE eat.source_language_id = ?
        AND eat.target_language_id = ?
)
SELECT
    ext.*
FROM exercise_type ext
WHERE ext.id IN (SELECT alo.type_id FROM allowed alo)
    OR NOT EXISTS (SELECT 1 FROM allowed);
"#;
