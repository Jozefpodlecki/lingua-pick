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

INSERT INTO skill
(
    id,
    name,
    description
)
VALUES
    ('noun_gender_selection', 'Noun gender selection', 'Identify an introduced grammatical noun class and its relevant agreement pattern where the target has noun gender.'),
    ('noun_case_selection', 'Noun case selection', 'Apply an introduced noun case to an explicit grammatical role where the target marks noun case.'),
    ('determiner_selection', 'Determiner selection', 'Select an introduced demonstrative or other determiner for an explicit referent; article selection remains a separate skill.'),
    ('classifier_selection', 'Classifier selection', 'Use an introduced numeral or noun classifier in a specified construction where applicable.'),
    ('aspect_selection', 'Aspect selection', 'Select an introduced aspect construction from an explicit event viewpoint rather than time reference alone.'),
    ('mood_selection', 'Mood selection', 'Select an introduced grammatical mood for an explicit intention or degree of certainty.'),
    ('politeness_formation', 'Politeness formation', 'Produce an introduced polite construction for a specified relationship and request, where applicable.'),
    ('verb_valency', 'Verb valency', 'Use an introduced verb with its required complements while preserving stated participant roles.'),
    ('reflexive_formation', 'Reflexive formation', 'Produce an introduced reflexive construction when the intended action refers back to its subject.'),
    ('passive_formation', 'Passive formation', 'Produce an introduced passive construction preserving the event and participant roles where the target supports it.'),
    ('relative_clause_formation', 'Relative clause formation', 'Construct one introduced relative clause identifying or describing a clear referent.'),
    ('clause_linking', 'Clause linking', 'Connect two familiar clauses using an introduced causal, temporal, or contrastive relationship.'),
    ('quantifier_selection', 'Quantifier selection', 'Select an introduced quantity expression appropriate to a specified noun and intended amount.'),
    ('number_agreement', 'Number agreement', 'Apply an introduced singular, plural, or other number-agreement relationship where applicable.'),
    ('spatial_relation_comprehension', 'Spatial relation comprehension', 'Understand a stated spatial relationship with explicit reference points rather than relying on world knowledge.'),
    ('temporal_relation_comprehension', 'Temporal relation comprehension', 'Understand the order and timing of stated events using introduced temporal expressions.'),
    ('reference_resolution', 'Reference resolution', 'Identify the referent of a pronoun or referring expression from sufficient supplied context.'),
    ('detail_extraction', 'Detail extraction', 'Find a requested explicitly stated fact in a short supplied passage or message.'),
    ('main_idea_identification', 'Main idea identification', 'Identify the central meaning of a short supplied text without substituting a minor detail.'),
    ('inference_from_context', 'Inference from context', 'Infer an intended meaning supported by the supplied text; do not require unstated cultural or specialist knowledge.'),
    ('paraphrase_comprehension', 'Paraphrase comprehension', 'Recognize a restatement preserving the original proposition and relevant nuance.'),
    ('paraphrase_production', 'Paraphrase production', 'Restate a short supplied proposition in the target language without changing its meaning.'),
    ('summarization', 'Summarization', 'Produce a brief target-language summary of a supplied text retaining its central meaning and key constraints.'),
    ('request_formation', 'Request formation', 'Produce a clear request for an explicitly stated need and social situation.'),
    ('invitation_response', 'Invitation response', 'Accept, decline, or qualify a supplied invitation with an appropriate short response.'),
    ('clarification_requests', 'Clarification requests', 'Ask for repetition, explanation, or disambiguation of an explicitly unclear message.'),
    ('opinion_expression', 'Opinion expression', 'Express a specified viewpoint with an introduced stance construction; correctness does not depend on agreeing with the viewpoint.'),
    ('narrative_sequencing', 'Narrative sequencing', 'Describe a short ordered sequence of supplied events using introduced linking expressions.'),
    ('discourse_cohesion', 'Discourse cohesion', 'Connect a short message using consistent references and introduced cohesive expressions.'),
    ('orthographic_conventions', 'Orthographic conventions', 'Apply introduced capitalization, diacritic, spacing, or script-form conventions to supplied text; this is not handwriting assessment.');

INSERT INTO skill_dependency
(
    skill_id,
    prerequisite_id
)
VALUES
    ('noun_gender_selection', 'grammatical_agreement'),
    ('noun_case_selection', 'basic_grammar'),
    ('determiner_selection', 'basic_grammar'),
    ('classifier_selection', 'basic_grammar'),
    ('aspect_selection', 'tense_selection'),
    ('mood_selection', 'verb_conjugation'),
    ('politeness_formation', 'register_selection'),
    ('verb_valency', 'basic_grammar'),
    ('reflexive_formation', 'pronoun_selection'),
    ('passive_formation', 'sentence_production'),
    ('relative_clause_formation', 'sentence_production'),
    ('clause_linking', 'sentence_production'),
    ('quantifier_selection', 'basic_grammar'),
    ('number_agreement', 'grammatical_agreement'),
    ('spatial_relation_comprehension', 'sentence_comprehension'),
    ('temporal_relation_comprehension', 'sentence_comprehension'),
    ('reference_resolution', 'sentence_comprehension'),
    ('detail_extraction', 'sentence_comprehension'),
    ('main_idea_identification', 'sentence_comprehension'),
    ('inference_from_context', 'detail_extraction'),
    ('paraphrase_comprehension', 'sentence_comprehension'),
    ('paraphrase_production', 'sentence_production'),
    ('summarization', 'main_idea_identification'),
    ('request_formation', 'sentence_production'),
    ('invitation_response', 'sentence_production'),
    ('clarification_requests', 'request_formation'),
    ('opinion_expression', 'sentence_production'),
    ('narrative_sequencing', 'temporal_relation_comprehension'),
    ('discourse_cohesion', 'clause_linking'),
    ('orthographic_conventions', 'spelling_accuracy'),
    ('summarization', 'sentence_production'),
    ('narrative_sequencing', 'sentence_production'),
    ('discourse_cohesion', 'reference_resolution');

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
        ('complete_agreement', 'noun_gender_selection'),
        ('fill_blank', 'noun_case_selection'),
        ('fill_blank', 'determiner_selection'),
        ('fill_blank', 'classifier_selection'),
        ('fill_blank', 'aspect_selection'),
        ('fill_blank', 'mood_selection'),
        ('short_response', 'politeness_formation'),
        ('fill_blank', 'verb_valency'),
        ('fill_blank', 'reflexive_formation'),
        ('build_sentence', 'passive_formation'),
        ('build_sentence', 'relative_clause_formation'),
        ('build_sentence', 'clause_linking'),
        ('fill_blank', 'quantifier_selection'),
        ('complete_agreement', 'number_agreement'),
        ('reading_question', 'spatial_relation_comprehension'),
        ('reading_question', 'temporal_relation_comprehension'),
        ('reading_question', 'reference_resolution'),
        ('reading_question', 'detail_extraction'),
        ('reading_question', 'main_idea_identification'),
        ('reading_question', 'inference_from_context'),
        ('reading_question', 'paraphrase_comprehension'),
        ('short_response', 'paraphrase_production'),
        ('short_response', 'summarization'),
        ('short_response', 'request_formation'),
        ('short_response', 'invitation_response'),
        ('short_response', 'clarification_requests'),
        ('short_response', 'opinion_expression'),
        ('short_response', 'narrative_sequencing'),
        ('short_response', 'discourse_cohesion'),
        ('complete_spelling', 'orthographic_conventions')
) mapping(kind, skill_id)
    ON mapping.kind = definition.kind;
