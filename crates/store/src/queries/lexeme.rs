pub const GET_LEXEME_BY_ID: &str = r#"
SELECT
    lex.*
FROM lexeme lex
WHERE lex.id = ?
"#;
