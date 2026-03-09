pub const GET_EXERCISE_BY_ID: &str = r#"
SELECT
    exc.*
FROM exercise exc
WHERE exc.id = ?
"#;

pub const INSERT_EXERCISE: &str = r#"
INSERT INTO exercise
(
    type_id,
    session_id,
    created_on,
    completed_on,
    data,
    result,
    verdict
)
VALUES
(?, ?, ?, ?, ?, ?, ?)
RETURNING id
"#;

pub const GET_EXERCISE_TYPES: &str = r#"
WITH allowed AS (
    SELECT
        let.exercise_type_id
    FROM language_exercise_type let
    WHERE let.language_id = ?
)
SELECT
    ext.*
FROM exercise_type ext
WHERE ext.id IN (SELECT alo.exercise_type_id FROM allowed alo)
    OR NOT EXISTS (SELECT 1 FROM allowed);
"#;

pub const UPDATE_EXERCISE_SET_DATA: &str = r#"
UPDATE exercise
SET data = ?
WHERE id = ?
"#;

pub const UPDATE_EXERCISE_SET_RESULT: &str = r#"
UPDATE exercise
SET result = ?
WHERE id = ?
"#;

pub const UPDATE_EXERCISE_SET_VERDICT: &str = r#"
UPDATE exercise
SET verdict = ?
WHERE id = ?
"#;

pub const UPDATE_EXERCISE_SET_COMPLETED: &str = r#"
UPDATE exercise
SET completed_on = ?
WHERE id = ?
"#;
