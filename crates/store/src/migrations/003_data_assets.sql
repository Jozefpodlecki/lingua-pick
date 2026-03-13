SET VARIABLE language_id = (SELECT id FROM language WHERE iso639_1 = 'aa');

INSERT INTO language_asset
(
    file_name,
    file_size,
    file_hash,
    language_id
)
VALUES
('data/aa/background.jpg', 1024, '', getvariable('language_id')),
('data/aa/lexemes.json', 1024, '', getvariable('language_id')),
('data/aa/graphemes.json', 1024, '', getvariable('language_id'));

SET VARIABLE language_id = (SELECT id FROM language WHERE iso639_1 = 'zh');

INSERT INTO language_asset
(
    file_name,
    file_size,
    file_hash,
    language_id
)
VALUES
('data/zh/lexemes.json', 1024, '', getvariable('language_id')),
('data/zh/graphemes.json', 1024, '', getvariable('language_id'));