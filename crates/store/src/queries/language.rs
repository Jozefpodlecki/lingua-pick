pub const GET_LANGUAGES_COUNT: &str = r#"
SELECT
    COUNT(*)
FROM language lan
"#;

pub const GET_LANGUAGE_BY_ISO639_1: &str = r#"
SELECT
    lan.*
FROM language lan
WHERE lan.iso639_1 = ?
"#;

pub const GET_LANGUAGE_BY_ISO639_3: &str = r#"
SELECT
    lan.*
FROM language lan
WHERE lan.iso639_3 = ?
"#;

pub const GET_LANGUAGES: &str = r#"
SELECT
    lan.*
FROM language lan
"#;

pub const GET_ASSETS_BY_LANGUAGE_ID: &str = r#"
SELECT
    las.*
FROM language_assets las
WHERE lan.language_id = ?
"#;
