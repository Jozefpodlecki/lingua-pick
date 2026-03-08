pub const GET_GRAPHEME_BY_ID: &str = r#"
SELECT
    gra.*
FROM grapheme gra
WHERE gra.id = ?
"#;

pub const GET_GRAPHEMES_BY_LANGUAGE_SAMPLE: &str = r#"
SELECT
    gra.id,
    gra.script_id,
    gra.symbol,
    grr.grapheme_id,
    grr.language_id,
    grr.system,
    grr.value
FROM grapheme gra
JOIN grapheme_reading grr
    ON grr.grapheme_id = gra.id
WHERE grr.language_id = ?
USING SAMPLE 10 ROWS;
"#;