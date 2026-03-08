SET VARIABLE language_id = (SELECT id FROM language WHERE iso639_1 = 'zh');

INSERT INTO language_asset
(
    file_name,
    language_id
)
VALUES
('data/zh/lexemes.json', getvariable('language_id')),
('data/zh/graphemes.json', getvariable('language_id'));