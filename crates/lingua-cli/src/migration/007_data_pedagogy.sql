INSERT INTO skill (id, name, description) VALUES
    ('script_reading', 'Script reading', 'Represent a written character or text in Latin script under an explicit romanization system.'),
    ('word_recognition', 'Word recognition', 'Associate a target-language word with its English meaning.'),
    ('word_recall', 'Word recall', 'Retrieve a familiar word without seeing its translation.'),
    ('basic_grammar', 'Basic grammar', 'Apply a simple grammatical relationship using familiar vocabulary.'),
    ('sentence_comprehension', 'Sentence comprehension', 'Understand the meaning of a short sentence.'),
    ('sentence_production', 'Sentence production', 'Construct a short sentence with familiar words and grammar.');

INSERT INTO skill_dependency (skill_id, prerequisite_id) VALUES
    ('word_recall', 'word_recognition'),
    ('basic_grammar', 'word_recognition'),
    ('sentence_comprehension', 'basic_grammar'),
    ('sentence_production', 'sentence_comprehension'),
    ('sentence_production', 'word_recall');

INSERT INTO learning_stage (code, sequence, name, generation_instructions, evaluation_instructions) VALUES
    ('words', 1, 'Word recognition',
     'For a learner with unknown knowledge, use match_words. Generate three to five common, concrete words from one familiar topic and their unambiguous English meanings. Keep the set small, repeat unfamiliar words, and introduce few new words at a time. Avoid grammar questions and sentence translation. Remain at this stage while recognition evidence is insufficient.',
     'Evaluate each word association separately. Record recognition evidence. Distinguish a correct independent association from a hinted or revealed answer. Matching does not establish recall, grammatical knowledge, or sentence production.'),
    ('grammar', 2, 'Basic grammar',
     'Introduce one grammatical relationship at a time using vocabulary the learner has previously recognized. Explain a new rule briefly before testing it. Keep word matching available for unfamiliar vocabulary. Do not assume that recognition proves recall.',
     'Evaluate the targeted grammatical relationship separately from vocabulary mistakes. Record the concept responsible for each error. Do not mark all concepts incorrect because one part of an answer is wrong.'),
    ('sentences', 3, 'Short sentences',
     'Use short sentences built from familiar vocabulary and introduced grammar. Start with comprehension and scaffolded completion before independent production. Introduce one new difficulty at a time. Revisit weak vocabulary or grammar when necessary.',
     'Evaluate meaning and the targeted structure separately. Accept valid equivalent English translations. Distinguish comprehension evidence from target-language production evidence.');

INSERT INTO learning_stage_dependency (stage_code, prerequisite_code) VALUES
    ('grammar', 'words'),
    ('sentences', 'grammar');

INSERT INTO learning_stage_skill (stage_code, skill_id, evidence_mode) VALUES
    ('words', 'word_recognition', 'recognition'),
    ('grammar', 'basic_grammar', 'production'),
    ('sentences', 'sentence_comprehension', 'comprehension'),
    ('sentences', 'sentence_production', 'production');

INSERT INTO learning_stage_exercise (stage_code, definition_id, selection_instructions) VALUES
    ('words', (SELECT id FROM exercise_definition WHERE kind = 'match_words'), 'Use matching exclusively for a new learner. Repetition of this exercise type is expected.'),
    ('grammar', (SELECT id FROM exercise_definition WHERE kind = 'match_words'), 'Review unfamiliar or weak words before using them in a grammar exercise.'),
    ('grammar', (SELECT id FROM exercise_definition WHERE kind = 'fill_blank'), 'Use one blank to isolate the newly introduced grammatical relationship.'),
    ('sentences', (SELECT id FROM exercise_definition WHERE kind = 'fill_blank'), 'Use scaffolded sentence completion before free production.'),
    ('sentences', (SELECT id FROM exercise_definition WHERE kind = 'translate_sentence'), 'Start by translating a short target-language sentence into English.');

INSERT INTO topic (id, name, description) VALUES
    ('everyday_objects', 'Everyday objects', 'Common concrete objects in familiar surroundings.'),
    ('animals', 'Animals', 'Common animals with clearly distinguishable names.'),
    ('food_and_drink', 'Food and drink', 'Common foods and drinks.'),
    ('people_and_family', 'People and family', 'Common family relationships and people.'),
    ('daily_actions', 'Daily actions', 'Frequent everyday actions.'),
    ('places', 'Places', 'Common locations and destinations.');

INSERT INTO topic_stage (topic_id, stage_code, generation_instructions) VALUES
    ('everyday_objects', 'words', 'Choose common concrete nouns with distinct meanings.'),
    ('animals', 'words', 'Choose familiar animals; avoid rare species and ambiguous labels.'),
    ('food_and_drink', 'words', 'Choose common items; distinguish region-specific meanings.'),
    ('people_and_family', 'words', 'Use relationships whose meanings are clear without complex context.'),
    ('daily_actions', 'grammar', 'Use frequent verbs with familiar nouns; introduce one form at a time.'),
    ('places', 'sentences', 'Use familiar places in short sentences with already introduced constructions.');

INSERT INTO teaching_guideline (code, category, generation_instructions, evaluation_instructions) VALUES
    ('unknown_knowledge', 'vocabulary', 'Treat absent learner evidence as unknown. Start with introductory word matching, not a grammar test.', 'Do not infer an incorrect answer or a proficiency level from missing evidence.'),
    ('controlled_novelty', 'vocabulary', 'Mix a small amount of new material with previously encountered material. Keep topic and difficulty changes limited.', 'Attribute evidence to the specific assessed concepts.'),
    ('meaning_before_form', 'grammar', 'Establish vocabulary meanings before asking the learner to manipulate grammatical forms.', 'Keep recognition, recall, and grammatical performance separate.'),
    ('scaffolding', 'sentence_construction', 'Progress from recognition to guided completion, comprehension, and independent production as evidence supports it.', 'Record assistance; a revealed answer is not independent success.'),
    ('regional_consistency', 'meaning_in_context', 'Generate natural vocabulary and usage for the exact selected regional variety. English remains the source language.', 'Judge correctness against the selected variety and accept valid equivalent meanings.'),
    ('ungraded_exposure', 'reading', 'Present introductory explanations or bilingual dialogue as exposure when no answer is assessed.', 'Completion and exposure must not produce correct-answer evidence.'),
    ('targeted_review', 'grammar', 'Use recent mistakes to revisit specific concepts with simpler examples. Keep previously successful material available for review.', 'A single success does not establish mastery; retain the history of assessed attempts.');
