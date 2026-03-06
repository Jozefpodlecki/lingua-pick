INSERT INTO script
VALUES
(uuidv7(), CURRENT_TIMESTAMP, 'Latin'),
(uuidv7(), CURRENT_TIMESTAMP, 'Cyrillic'),
(uuidv7(), CURRENT_TIMESTAMP, 'Arabic'),
(uuidv7(), CURRENT_TIMESTAMP, 'Greek'),
(uuidv7(), CURRENT_TIMESTAMP, 'Hebrew'),
(uuidv7(), CURRENT_TIMESTAMP, 'Devanagari'),
(uuidv7(), CURRENT_TIMESTAMP, 'Bengali'),
(uuidv7(), CURRENT_TIMESTAMP, 'Gurmukhi'),
(uuidv7(), CURRENT_TIMESTAMP, 'Gujarati'),
(uuidv7(), CURRENT_TIMESTAMP, 'Oriya'),
(uuidv7(), CURRENT_TIMESTAMP, 'Tamil'),
(uuidv7(), CURRENT_TIMESTAMP, 'Telugu'),
(uuidv7(), CURRENT_TIMESTAMP, 'Kannada'),
(uuidv7(), CURRENT_TIMESTAMP, 'Malayalam'),
(uuidv7(), CURRENT_TIMESTAMP, 'Sinhala'),
(uuidv7(), CURRENT_TIMESTAMP, 'Thai'),
(uuidv7(), CURRENT_TIMESTAMP, 'Lao'),
(uuidv7(), CURRENT_TIMESTAMP, 'Burmese'),
(uuidv7(), CURRENT_TIMESTAMP, 'Han'),
(uuidv7(), CURRENT_TIMESTAMP, 'Hangul'),
(uuidv7(), CURRENT_TIMESTAMP, 'Hiragana'),
(uuidv7(), CURRENT_TIMESTAMP, 'Katakana'),
(uuidv7(), CURRENT_TIMESTAMP, 'Yi'),
(uuidv7(), CURRENT_TIMESTAMP, 'Armenian'),
(uuidv7(), CURRENT_TIMESTAMP, 'Georgian'),
(uuidv7(), CURRENT_TIMESTAMP, 'Mongolian'),
(uuidv7(), CURRENT_TIMESTAMP, 'Thaana'),
(uuidv7(), CURRENT_TIMESTAMP, 'Ethiopic'),
(uuidv7(), CURRENT_TIMESTAMP, 'Canadian Aboriginal');

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