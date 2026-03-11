SET VARIABLE language_id = (SELECT id FROM language WHERE name = 'Afar');
INSERT INTO language_feature_language
(
    language_id,
    feature_id
)
VALUES
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'SOV')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Moderately flexible')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Topicalization')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No grammatical gender')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Nominative-Accusative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Singular/Plural only')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Dual')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Conjugates for person/number/tense')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Temporal Aspect')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Progressive (morphological)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Progressive (periphrastic)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Perfective (morphological)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Perfective (analytic)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Imperfective')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Habitual (marked)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Indicative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Present/Past/Future')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Subject Pro-drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Canonical subject pro-drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Object Pro-drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Object drop allowed')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Discourse Drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Topic drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Isolating')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Tonal language')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Suffix')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Preverbal negation')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Morphological Honorifics')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Honorific verb forms')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Lexical Honorifics')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Humble verb forms')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Pragmatic Politeness')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Indirect requests')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No classifiers')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Direct evidential')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Overt copula'));

SET VARIABLE language_id = (SELECT id FROM language WHERE name = 'Abkhazian');
INSERT INTO language_feature_language
(
    language_id,
    feature_id
)
VALUES
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'SOV')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Rigid')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No grammatical gender')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Ergative-Absolutive')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Singular/Plural only')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Conjugates for person/number/tense')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Temporal Aspect')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Perfective (morphological)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Imperfective')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Indicative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Present/Past/Future')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Subject Pro-drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Canonical subject pro-drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Object Pro-drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Object drop allowed')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Discourse Drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Topic drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Agglutinative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Tonal language')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Suffix')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Preverbal negation')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Morphological Honorifics')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Honorific verb forms')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Lexical Honorifics')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Humble verb forms')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Pragmatic Politeness')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Indirect requests')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No classifiers')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Direct evidential')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Overt copula'));

SET VARIABLE language_id = (SELECT id FROM language WHERE name = 'English');
INSERT INTO language_feature_language
(
    language_id,
    feature_id
)
VALUES
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'SVO')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Rigid')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No grammatical gender')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Nominative-Accusative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Singular/Plural only')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Conjugates for person/number/tense')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Temporal Aspect')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Progressive (periphrastic)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Perfective (analytic)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Imperfective')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Indicative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Present/Past/Future')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Restricted subject pro-drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Object drop disallowed')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No discourse drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Fusional')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No tone')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Suffix')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Negation particle')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Single negation')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No classifiers')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Direct evidential')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Overt copula'));

SET VARIABLE language_id = (SELECT id FROM language WHERE name = 'Chhattisgarhi');
INSERT INTO language_feature_language
(
    language_id,
    feature_id
)
VALUES
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'SOV')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Moderately flexible')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Masculine-Feminine')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Nominative-Accusative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Singular/Plural only')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Conjugates for person/number/tense')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Progressive (periphrastic)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Perfective (analytic)')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Imperfective')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Indicative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Present/Past/Future')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Restricted subject pro-drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No discourse drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Agglutinative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No tone')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Suffix')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Negation particle')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Single negation')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No classifiers')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Direct evidential')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Overt copula'));

SET VARIABLE language_id = (SELECT id FROM language WHERE name = 'Chinese');
INSERT INTO language_feature_language
(
    language_id,
    feature_id
)
VALUES
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'SVO')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Moderately flexible')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Topicalization')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Question inversion')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No grammatical gender')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Nominative-Accusative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Singular/Plural only')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Minimal conjugation')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Temporal Aspect')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Aspect unmarked')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Indicative')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Present/Past/Future')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No subject pro-drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'No discourse drop')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Isolating')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Tonal language')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Register tone')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Suffix')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Preverbal negation')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Obligatory classifiers')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Direct evidential')),
(getvariable('language_id'), (SELECT id FROM language_feature WHERE name = 'Overt copula'));