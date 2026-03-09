pub const GET_LEXEME_BY_ID: &str = r#"
SELECT
    lex.*
FROM lexeme lex
WHERE lex.id = ?
"#;

pub const GET_LEXEME_BY_LANGUAGE_ID: &str = r#"
SELECT
    lex.*
FROM lexeme lex
WHERE lex.language_id = ?
"#;
