INSERT INTO script
(
    name,
    iso15924
)
VALUES
('Latin', 'Latn'),
('Cyrillic', 'Cyrl'),
('Arabic', 'Arab'),
('Greek', 'Grek'),
('Hebrew', 'Hebr'),
('Devanagari', 'Deva'),
('Bengali', 'Beng'),
('Gurmukhi', 'Guru'),
('Gujarati', 'Gujr'),
('Oriya', 'Orya'),
('Tamil', 'Taml'),
('Telugu', 'Telu'),
('Kannada', 'Knda'),
('Malayalam', 'Mlym'),
('Sinhala', 'Sinh'),
('Thai', 'Thai'),
('Lao', 'Laoo'),
('Burmese', 'Mymr'),
('Han', 'Hani'),
('Hangul', 'Hang'),
('Hiragana', 'Hira'),
('Katakana', 'Kana'),
('Yi', 'Yiii'),
('Armenian', 'Armn'),
('Georgian', 'Geor'),
('Mongolian', 'Mong'),
('Thaana', 'Thaa'),
('Ethiopic', 'Ethi'),
('Canadian Aboriginal', 'Cans'),
('Tibetan','Tibt'),
('Khmer','Khmr'),
('Cherokee','Cher'),
('Han Simplified','Hans'),
('Han Traditional','Hant');

INSERT INTO language_script (language_id, script_id, is_primary)

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Latin'
WHERE lan.iso639_1 IN (
'en','fr','de','es','pt','it','nl','sv','da','fi','et','lv','lt',
'ca','eu','ga','gd','gl','cy','af','sq','sw','tl','id','ms','vi',
'pl','cs','sk','sl','hu','ro','hr','bs','no','is','mt','tr','az','uz'
)

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Cyrillic'
WHERE lan.iso639_1 IN (
'ru','uk','bg','sr','mk','kk','ky','tg','mn'
)

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Arabic'
WHERE lan.iso639_1 IN (
'ar','fa','ur','ps','sd','ug','ku'
)

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Greek'
WHERE lan.iso639_1 = 'el'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Hebrew'
WHERE lan.iso639_1 = 'he'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Devanagari'
WHERE lan.iso639_1 IN ('hi','mr','ne','sa')

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Bengali'
WHERE lan.iso639_1 = 'bn'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Gurmukhi'
WHERE lan.iso639_1 = 'pa'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Gujarati'
WHERE lan.iso639_1 = 'gu'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Oriya'
WHERE lan.iso639_1 = 'or'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Tamil'
WHERE lan.iso639_1 = 'ta'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Telugu'
WHERE lan.iso639_1 = 'te'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Kannada'
WHERE lan.iso639_1 = 'kn'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Malayalam'
WHERE lan.iso639_1 = 'ml'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Sinhala'
WHERE lan.iso639_1 = 'si'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Thai'
WHERE lan.iso639_1 = 'th'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Lao'
WHERE lan.iso639_1 = 'lo'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Burmese'
WHERE lan.iso639_1 = 'my'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Hangul'
WHERE lan.iso639_1 = 'ko'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name IN ('Hiragana','Katakana','Han')
WHERE lan.iso639_1 = 'ja'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Armenian'
WHERE lan.iso639_1 = 'hy'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Georgian'
WHERE lan.iso639_1 = 'ka'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Mongolian'
WHERE lan.iso639_1 = 'mn'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Thaana'
WHERE lan.iso639_1 = 'dv'

UNION ALL

SELECT lan.id, scr.id, TRUE
FROM language lan
JOIN script scr ON scr.name = 'Ethiopic'
WHERE lan.iso639_1 IN ('am','ti');

UNION ALL

SELECT lan.id, scr.id, FALSE
FROM language lan
JOIN script scr ON scr.name = 'Latin'
WHERE lan.iso639_1 = 'sr';

UNION ALL

SELECT lan.id, scr.id, FALSE
FROM language lan
JOIN script scr ON scr.name = 'Cyrillic'
WHERE lan.iso639_1 = 'uz';