
SET VARIABLE language_id = (SELECT id FROM language WHERE iso639_3 = 'zho');

INSERT INTO lexeme
VALUES
(uuidv7(), getvariable('language_id'), CURRENT_TIMESTAMP, '你好', '你好');

INSERT INTO meaning
VALUES
(uuidv7(), CURRENT_TIMESTAMP, 'A greeting used to say hello in Mandarin');

INSERT INTO lexeme_meaning
VALUES
(
    (SELECT id FROM lexeme WHERE text = '你好' LIMIT 1),
    (SELECT id FROM meaning WHERE description LIKE '%hello%' LIMIT 1)
);

INSERT INTO lexeme_reading
VALUES
(uuidv7(), (SELECT id FROM lexeme WHERE text='你好' LIMIT 1), 'pinyin', 'nǐ hǎo'),