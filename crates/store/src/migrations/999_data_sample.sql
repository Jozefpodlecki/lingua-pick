
SET VARIABLE language_id = (SELECT id FROM language WHERE iso639_3 = 'zho');

INSERT INTO lexeme
(
    language_id,
    text,
    normalized
)
VALUES
(getvariable('language_id'), '你好', '你好');

SET VARIABLE lexeme_id = (SELECT id FROM lexeme WHERE text = '你好');

INSERT INTO meaning
(
    description
)
VALUES
('A greeting used to say hello in Mandarin');

SET VARIABLE meaning_id = (SELECT id FROM meaning WHERE description LIKE '%hello%' LIMIT 1);

INSERT INTO lexeme_meaning
(
    lexeme_id,
    meaning_id
)
VALUES
(
    getvariable('lexeme_id'),
    getvariable('meaning_id')
);

INSERT INTO lexeme_reading
(
    lexeme_id,
    system,
    value
)
VALUES
(getvariable('lexeme_id'), 'pinyin', 'nǐ hǎo'),