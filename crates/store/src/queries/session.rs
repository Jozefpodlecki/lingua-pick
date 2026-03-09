pub const INSERT_SESSION: &str = r#"
INSERT INTO session
VALUES
(?, ?, ?, ?, ?, ?, ?, ?)
"#;

pub const GET_SESSION_BY_ID: &str = r#"
SELECT
    *
FROM session
WHERE id = ?
"#;

pub const INSERT_SESSION_USER: &str = r#"
INSERT INTO session_user
VALUES
(?, ?, ?)
"#;

pub const UPDATE_SESSION_SET_EXERCISE: &str = r#"
UPDATE session
SET current_exercise_id = ?
WHERE id = ?
"#;

