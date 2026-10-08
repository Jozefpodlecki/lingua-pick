INSERT INTO skill
(
    id,
    name,
    description
)
VALUES
    ('sound_discrimination', 'Sound discrimination', 'Distinguish target-language sounds when an exercise supplies actual audio; text exposure alone is not listening evidence.'),
    ('pronunciation_mapping', 'Pronunciation mapping', 'Relate written forms to an explicitly named pronunciation or romanization convention; distinguish this from assessed speech production.'),
    ('spelling_accuracy', 'Spelling accuracy', 'Produce the correct written form of familiar vocabulary, preserving required accents and script distinctions.'),
    ('contextual_vocabulary', 'Contextual vocabulary', 'Identify the intended sense of a familiar word from explicit surrounding context.'),
    ('synonym_recognition', 'Synonym recognition', 'Recognize contextually appropriate similar meanings without assuming synonyms are interchangeable everywhere.'),
    ('antonym_recognition', 'Antonym recognition', 'Recognize a clear opposition between familiar words within a specified meaning.'),
    ('collocation_recognition', 'Collocation recognition', 'Recognize natural combinations of familiar words in the selected regional variety.'),
    ('idiom_comprehension', 'Idiom comprehension', 'Understand the conventional meaning of an introduced idiom in a clear context.'),
    ('register_selection', 'Register selection', 'Select a form appropriate to an explicit audience, relationship, and degree of formality.'),
    ('verb_conjugation', 'Verb conjugation', 'Produce an introduced verb form for an explicit person, number, tense, or mood.'),
    ('tense_selection', 'Tense selection', 'Select an introduced tense or aspect using a clear time reference and intended meaning.'),
    ('article_selection', 'Article selection', 'Apply an introduced article or determiner pattern where the target language uses one.'),
    ('preposition_selection', 'Preposition selection', 'Apply an introduced preposition or equivalent relational form in a clear context.'),
    ('pronoun_selection', 'Pronoun selection', 'Select an introduced pronoun using an explicit referent and appropriate social context.'),
    ('grammatical_agreement', 'Grammatical agreement', 'Apply one introduced agreement relationship while keeping vocabulary and other structures familiar.'),
    ('plural_formation', 'Plural formation', 'Produce an introduced plural or number-marking form where applicable to the target language.'),
    ('possession_expression', 'Possession expression', 'Express an explicit ownership relationship using an introduced target-language construction.'),
    ('negation_formation', 'Negation formation', 'Express a specified negative meaning using one introduced construction.'),
    ('question_formation', 'Question formation', 'Construct an introduced question form without changing the intended referents or meaning.'),
    ('comparison_formation', 'Comparison formation', 'Express an explicit comparison using an introduced comparative or superlative pattern.'),
    ('conditional_formation', 'Conditional formation', 'Complete an introduced conditional relationship with clear intended meaning and familiar vocabulary.'),
    ('word_order', 'Word order', 'Arrange familiar tokens into a grammatical construction while retaining the intended meaning.'),
    ('punctuation', 'Punctuation', 'Apply the target language and script conventions to a familiar sentence without introducing new grammar.'),
    ('sentence_revision', 'Sentence revision', 'Identify and repair a targeted problem in a familiar sentence while preserving meaning.');

INSERT INTO skill_dependency
(
    skill_id,
    prerequisite_id
)
VALUES
    ('pronunciation_mapping', 'script_reading'),
    ('spelling_accuracy', 'word_recall'),
    ('contextual_vocabulary', 'word_recognition'),
    ('synonym_recognition', 'word_recognition'),
    ('antonym_recognition', 'word_recognition'),
    ('collocation_recognition', 'contextual_vocabulary'),
    ('idiom_comprehension', 'sentence_comprehension'),
    ('register_selection', 'contextual_vocabulary'),
    ('verb_conjugation', 'basic_grammar'),
    ('tense_selection', 'basic_grammar'),
    ('article_selection', 'basic_grammar'),
    ('preposition_selection', 'basic_grammar'),
    ('pronoun_selection', 'basic_grammar'),
    ('grammatical_agreement', 'basic_grammar'),
    ('plural_formation', 'basic_grammar'),
    ('possession_expression', 'basic_grammar'),
    ('negation_formation', 'basic_grammar'),
    ('question_formation', 'basic_grammar'),
    ('comparison_formation', 'basic_grammar'),
    ('conditional_formation', 'sentence_comprehension'),
    ('word_order', 'basic_grammar'),
    ('punctuation', 'sentence_comprehension'),
    ('sentence_revision', 'sentence_comprehension');

INSERT INTO exercise_definition_skill
(
    definition_id,
    skill_id
)
SELECT
    definition.id,
    mapping.skill_id
FROM exercise_definition definition
JOIN
(
    VALUES
        ('match_words', 'word_recognition'),
        ('fill_blank', 'basic_grammar'),
        ('translate_sentence', 'sentence_comprehension'),
        ('single_choice', 'word_recognition'),
        ('choose_word', 'word_recognition'),
        ('meaning_in_context', 'contextual_vocabulary'),
        ('choose_synonym', 'synonym_recognition'),
        ('choose_antonym', 'antonym_recognition'),
        ('choose_register', 'register_selection'),
        ('choose_collocation', 'collocation_recognition'),
        ('choose_idiom_meaning', 'idiom_comprehension'),
        ('reading_question', 'sentence_comprehension'),
        ('recall_word', 'word_recall'),
        ('complete_spelling', 'spelling_accuracy'),
        ('conjugate_verb', 'verb_conjugation'),
        ('choose_article', 'article_selection'),
        ('choose_preposition', 'preposition_selection'),
        ('choose_pronoun', 'pronoun_selection'),
        ('complete_agreement', 'grammatical_agreement'),
        ('form_plural', 'plural_formation'),
        ('express_possession', 'possession_expression'),
        ('negate_sentence', 'negation_formation'),
        ('form_question', 'question_formation'),
        ('change_tense', 'tense_selection'),
        ('make_comparison', 'comparison_formation'),
        ('complete_conditional', 'conditional_formation'),
        ('correct_sentence', 'sentence_revision'),
        ('rewrite_register', 'register_selection'),
        ('punctuate_sentence', 'punctuation'),
        ('order_words', 'word_order'),
        ('build_sentence', 'sentence_production'),
        ('translate_to_target', 'sentence_production'),
        ('short_response', 'sentence_production'),
        ('dialogue', 'sentence_comprehension'),
        ('transliterate', 'script_reading')
) mapping(kind, skill_id)
    ON mapping.kind = definition.kind;
