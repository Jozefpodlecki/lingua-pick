
pub const INSERT_MIGRATION: &str = r#"
INSERT INTO migrations
VALUES
(?, ?, ?)
"#;

pub const CHECK_MIGRATIONS_TABLE: &str = r#"
SELECT
    COUNT(*) > 0
FROM information_schema.tables
WHERE table_name = 'migrations'"#;

pub const GET_MIGRATION_BY_FILE_NAME: &str = r#"
SELECT
    id
FROM migrations
WHERE file_name = ?"#;

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

pub const INSERT_USER: &str = r#"
INSERT INTO user
VALUES
(?, ?, ?, ?)
"#;

pub const GET_USER_BY_USERNAME: &str = r#"
SELECT
    usr.id,
    usr.created_on,
    usr.username,
    usr.password_hash
FROM user usr
WHERE usr.username = ?
"#;

pub const INSERT_USER_PROFILE: &str = r#"
INSERT INTO user_profile
VALUES
(?, ?, ?, ?, ?, ?, ?)
"#;

pub const GET_ACTIVE_USER_PROFILE_BY_USER: &str = r#"
SELECT
    upr.*
FROM user_profile upr
WHERE upr.user_id = ?
    AND upr.is_active = true
"#;

pub const GET_USER_PROFILE_BY_USER: &str = r#"
SELECT
    upr.*
FROM user_profile upr
WHERE upr.user_id = ?
"#;

pub const UPDATE_USER_PROFILE_SET_ACTIVE: &str = r#"
UPDATE user_profile
SET is_active = true
WHERE id = ?
"#;


pub const UPDATE_METADATA: &str = r#"
INSERT INTO metadata
(id, version, updated_on)
VALUES
(TRUE, ?, ?)
    ON CONFLICT (id)
    DO UPDATE SET
        version = EXCLUDED.version,
        updated_on = EXCLUDED.updated_on;"#;


pub const INSERT_WOTD: &str = r#"
INSERT INTO word_of_the_day
VALUES
(?, ?, ?)
"#;

pub const GET_WOTD_BY_ID: &str = r#"
SELECT
    wotd.*
FROM word_of_the_day wotd
WHERE id = ?"#;