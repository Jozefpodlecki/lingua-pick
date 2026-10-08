INSERT INTO exercise_definition (
    id, kind, name, description, category, instructions, schema, answer_schema, verdict_schema
) VALUES
(
    uuidv7(),
    'match_words',
    'Match words',
    'Associate target words with English meanings.',
    'vocabulary',
    'Generate three to five common words from one familiar topic. Use unambiguous English meanings and distinct labels. Repeat this interaction for unknown learners. Each concept_id identifies one association; generate content without requiring a seeded dictionary. Presentation columns are shuffled by the application.',
    '{"type": "object", "additionalProperties": false, "required": ["pairs"], "properties": {"pairs": {"type": "array", "minItems": 3, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "target", "english"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "target": {"type": "string", "minLength": 1}, "english": {"type": "string", "minLength": 1}}}}}}',
    '{"type": "object", "additionalProperties": false, "required": ["matches"], "properties": {"matches": {"type": "array", "minItems": 3, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "english"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "english": {"type": "string", "minLength": 1}}}}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'fill_blank',
    'Fill the blank',
    'Complete one missing grammatical form.',
    'grammar',
    'Create exactly one blank in a short sentence using familiar words. Supply accepted completions. Target one introduced grammatical concept; explain a new rule before testing it.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'translate_sentence',
    'Translate a sentence',
    'Translate a target-language sentence into English.',
    'sentence_construction',
    'Generate a short target-language sentence using familiar vocabulary and introduced grammar. Supply valid English reference translations. Accept meaning-preserving equivalents rather than requiring an exact reference string.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'single_choice',
    'Choose the meaning',
    'Recognize one target word or phrase.',
    'vocabulary',
    'Ask for the English meaning of a target word. Use plausible distinct English distractors; exactly one choice must fit. Give every choice a unique stable id, and set correct_choice_id to exactly one existing id. The application shuffles choices.',
    '{"type": "object", "additionalProperties": false, "required": ["prompt", "choices", "correct_choice_id", "concept_ids"], "properties": {"prompt": {"type": "string", "minLength": 1}, "choices": {"type": "array", "minItems": 2, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "correct_choice_id": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["choice_id"], "properties": {"choice_id": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'choose_word',
    'Choose the target word',
    'Recognize the target word for an English meaning.',
    'vocabulary',
    'Give an English meaning and target-language choices. Exactly one choice must fit the intended sense. Give every choice a unique stable id, and set correct_choice_id to exactly one existing id. The application shuffles choices.',
    '{"type": "object", "additionalProperties": false, "required": ["prompt", "choices", "correct_choice_id", "concept_ids"], "properties": {"prompt": {"type": "string", "minLength": 1}, "choices": {"type": "array", "minItems": 2, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "correct_choice_id": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["choice_id"], "properties": {"choice_id": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'meaning_in_context',
    'Choose the contextual meaning',
    'Interpret a word within a short context.',
    'meaning_in_context',
    'Include the target word in a short sentence and ask for its English meaning in that context. Avoid multiple defensible choices. Give every choice a unique stable id, and set correct_choice_id to exactly one existing id. The application shuffles choices.',
    '{"type": "object", "additionalProperties": false, "required": ["prompt", "choices", "correct_choice_id", "concept_ids"], "properties": {"prompt": {"type": "string", "minLength": 1}, "choices": {"type": "array", "minItems": 2, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "correct_choice_id": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["choice_id"], "properties": {"choice_id": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'choose_synonym',
    'Choose a synonym',
    'Recognize a contextually equivalent expression.',
    'synonyms',
    'Provide enough context to identify one synonymous target expression. Distractors must differ in meaning in that context. Give every choice a unique stable id, and set correct_choice_id to exactly one existing id. The application shuffles choices.',
    '{"type": "object", "additionalProperties": false, "required": ["prompt", "choices", "correct_choice_id", "concept_ids"], "properties": {"prompt": {"type": "string", "minLength": 1}, "choices": {"type": "array", "minItems": 2, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "correct_choice_id": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["choice_id"], "properties": {"choice_id": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'choose_antonym',
    'Choose an opposite',
    'Recognize a contextually opposite expression.',
    'antonyms',
    'Use a familiar word in context and offer one clear opposite with distinct distractors. Give every choice a unique stable id, and set correct_choice_id to exactly one existing id. The application shuffles choices.',
    '{"type": "object", "additionalProperties": false, "required": ["prompt", "choices", "correct_choice_id", "concept_ids"], "properties": {"prompt": {"type": "string", "minLength": 1}, "choices": {"type": "array", "minItems": 2, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "correct_choice_id": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["choice_id"], "properties": {"choice_id": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'choose_register',
    'Choose the appropriate register',
    'Select language suited to a social situation.',
    'register',
    'Describe the relationship and situation in English. Offer target-language expressions with one appropriate register; explain the distinction in feedback. Give every choice a unique stable id, and set correct_choice_id to exactly one existing id. The application shuffles choices.',
    '{"type": "object", "additionalProperties": false, "required": ["prompt", "choices", "correct_choice_id", "concept_ids"], "properties": {"prompt": {"type": "string", "minLength": 1}, "choices": {"type": "array", "minItems": 2, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "correct_choice_id": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["choice_id"], "properties": {"choice_id": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'choose_collocation',
    'Choose a collocation',
    'Recognize words that naturally occur together.',
    'collocations',
    'Provide a short contextual expression with one missing word and choices. Use natural combinations for the selected regional variety. Give every choice a unique stable id, and set correct_choice_id to exactly one existing id. The application shuffles choices.',
    '{"type": "object", "additionalProperties": false, "required": ["prompt", "choices", "correct_choice_id", "concept_ids"], "properties": {"prompt": {"type": "string", "minLength": 1}, "choices": {"type": "array", "minItems": 2, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "correct_choice_id": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["choice_id"], "properties": {"choice_id": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'choose_idiom_meaning',
    'Choose an idiom meaning',
    'Recognize the intended meaning of a contextual idiom.',
    'idioms',
    'Use a common idiom in a clear context after sufficient basic comprehension. Ask for its English meaning; avoid literal distractors that also fit the context. Give every choice a unique stable id, and set correct_choice_id to exactly one existing id. The application shuffles choices.',
    '{"type": "object", "additionalProperties": false, "required": ["prompt", "choices", "correct_choice_id", "concept_ids"], "properties": {"prompt": {"type": "string", "minLength": 1}, "choices": {"type": "array", "minItems": 2, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "correct_choice_id": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["choice_id"], "properties": {"choice_id": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'reading_question',
    'Answer a reading question',
    'Identify information in a short written passage.',
    'reading_comprehension',
    'Include a short passage in the prompt and a question with one supported answer. Require no external facts; keep vocabulary appropriate to learner evidence. Give every choice a unique stable id, and set correct_choice_id to exactly one existing id. The application shuffles choices.',
    '{"type": "object", "additionalProperties": false, "required": ["prompt", "choices", "correct_choice_id", "concept_ids"], "properties": {"prompt": {"type": "string", "minLength": 1}, "choices": {"type": "array", "minItems": 2, "maxItems": 5, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "correct_choice_id": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["choice_id"], "properties": {"choice_id": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'recall_word',
    'Recall a word',
    'Produce one contextually constrained word or form.',
    'vocabulary',
    'Give an English meaning and enough context to retrieve one target word. Do not show the target answer or choices. Include accepted answers; the learner submits the missing form only.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'complete_spelling',
    'Complete the spelling',
    'Produce one contextually constrained word or form.',
    'spelling',
    'Mask part of one familiar target word and give its English meaning. Ask for the complete word. State which written variants are acceptable. Include accepted answers; the learner submits the missing form only.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'conjugate_verb',
    'Conjugate a verb',
    'Produce one contextually constrained word or form.',
    'conjugation',
    'Provide the verb base form and one sentence context. Specify only introduced inflectional features actually marked by this target, such as tense, aspect, mood, voice, person or number. Do not invent person agreement or tense contrasts absent from the language. Ask for one contextually constrained form. Include accepted answers; the learner submits the missing form only.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'choose_article',
    'Supply an article',
    'Produce one contextually constrained word or form.',
    'articles',
    'Use one article blank in a short familiar noun phrase or sentence. Provide enough context to distinguish definiteness and noun features. Include accepted answers; the learner submits the missing form only.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'choose_preposition',
    'Supply a preposition',
    'Produce one contextually constrained word or form.',
    'prepositions',
    'Use one preposition blank with enough context to isolate the intended relationship. Accept valid regional alternatives. Include accepted answers; the learner submits the missing form only.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'choose_pronoun',
    'Supply a pronoun',
    'Produce one contextually constrained word or form.',
    'pronouns',
    'Use one pronoun blank with explicit referent, person and number where needed. Avoid ambiguous omitted-subject contexts. Include accepted answers; the learner submits the missing form only.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'complete_agreement',
    'Complete agreement',
    'Produce one contextually constrained word or form.',
    'agreement',
    'Use one blank testing gender, number or person agreement. Keep all other grammatical features familiar. Include accepted answers; the learner submits the missing form only.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'form_plural',
    'Form a plural',
    'Produce one contextually constrained word or form.',
    'pluralization',
    'Request the plural form of a familiar singular noun in a short context. Use target-specific rules and accepted regional forms. Include accepted answers; the learner submits the missing form only.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'express_possession',
    'Express possession',
    'Produce one contextually constrained word or form.',
    'possessives',
    'Complete one possessive form with clear possessor and possessed noun. State context needed to disambiguate person and number. Include accepted answers; the learner submits the missing form only.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'negate_sentence',
    'Negate a sentence',
    'Transform a short sentence under an explicit constraint.',
    'negation',
    'Transform a short affirmative sentence into a negative one while preserving tense, participants and core meaning.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "transformation", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "transformation": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'form_question',
    'Form a question',
    'Transform a short sentence under an explicit constraint.',
    'questions',
    'Transform a short statement into a specified question. State the question focus and preserve the remaining meaning.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "transformation", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "transformation": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'change_tense',
    'Change the tense',
    'Transform a short sentence under an explicit constraint.',
    'tense',
    'Rewrite a short sentence in one specified introduced tense. Preserve participants and adapt time expressions only when instructed.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "transformation", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "transformation": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'make_comparison',
    'Make a comparison',
    'Transform a short sentence under an explicit constraint.',
    'comparatives',
    'Rewrite supplied information as a comparative or superlative sentence. State exactly which relationship must be expressed.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "transformation", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "transformation": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'complete_conditional',
    'Complete a conditional',
    'Transform a short sentence under an explicit constraint.',
    'conditionals',
    'Rewrite a short scenario as a specified conditional construction. Use an introduced conditional pattern and clear hypothetical or factual context.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "transformation", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "transformation": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'correct_sentence',
    'Correct a sentence',
    'Transform a short sentence under an explicit constraint.',
    'grammar',
    'Provide a sentence with one intentional error in an introduced rule. Ask for the corrected full sentence; avoid introducing additional errors.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "transformation", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "transformation": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'rewrite_register',
    'Rewrite for register',
    'Transform a short sentence under an explicit constraint.',
    'register',
    'Rewrite a short sentence for a specified audience and situation, preserving meaning. Supply representative acceptable versions and accept valid alternatives.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "transformation", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "transformation": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'punctuate_sentence',
    'Punctuate a sentence',
    'Transform a short sentence under an explicit constraint.',
    'punctuation',
    'Supply a short sentence with omitted punctuation. Ask for the punctuated sentence without changing words. Follow conventions of the exact target.',
    '{"type": "object", "additionalProperties": false, "required": ["sentence", "transformation", "accepted_answers", "concept_ids"], "properties": {"sentence": {"type": "string", "minLength": 1}, "transformation": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'order_words',
    'Order words',
    'Arrange tokens into a grammatical sentence.',
    'word_order',
    'Generate a short sentence split into uniquely identified tokens, including duplicate words as separate token identities. Supply acceptable token-id orders. The application shuffles tokens. Accept valid alternative word orders.',
    '{"type": "object", "additionalProperties": false, "required": ["instruction", "tokens", "accepted_orders", "concept_ids"], "properties": {"instruction": {"type": "string", "minLength": 1}, "tokens": {"type": "array", "minItems": 3, "maxItems": 12, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "accepted_orders": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "array", "minItems": 3, "maxItems": 12, "items": {"type": "string", "minLength": 1}}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["token_ids"], "properties": {"token_ids": {"type": "array", "minItems": 3, "maxItems": 12, "items": {"type": "string", "minLength": 1}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'build_sentence',
    'Build a sentence',
    'Build a target sentence from a word bank.',
    'sentence_construction',
    'Provide an English meaning and target-language tokens with unique identities. Include sufficient tokens for each accepted order; identify any distractors through the accepted orders. The application shuffles tokens. Use familiar words and introduced structures.',
    '{"type": "object", "additionalProperties": false, "required": ["english", "tokens", "accepted_orders", "concept_ids"], "properties": {"english": {"type": "string", "minLength": 1}, "tokens": {"type": "array", "minItems": 3, "maxItems": 15, "items": {"type": "object", "additionalProperties": false, "required": ["id", "text"], "properties": {"id": {"type": "string", "minLength": 1}, "text": {"type": "string", "minLength": 1}}}}, "accepted_orders": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "array", "minItems": 2, "maxItems": 12, "items": {"type": "string", "minLength": 1}}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["token_ids"], "properties": {"token_ids": {"type": "array", "minItems": 2, "maxItems": 12, "items": {"type": "string", "minLength": 1}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'translate_to_target',
    'Translate into the target language',
    'Produce a target-language sentence from English.',
    'translation',
    'Give a short English sentence using meanings the learner has encountered. Supply target-language reference answers for the exact regional variety. Accept valid equivalent translations and evaluate vocabulary and grammar separately.',
    '{"type": "object", "additionalProperties": false, "required": ["english", "accepted_answers", "concept_ids"], "properties": {"english": {"type": "string", "minLength": 1}, "accepted_answers": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'short_response',
    'Write a short response',
    'Produce a brief response to a constrained situation.',
    'writing',
    'Describe a familiar situation and a single communicative goal in English. Request one or two target-language sentences. Provide English evaluation criteria and representative reference responses. Accept alternative phrasing that achieves the goal.',
    '{"type": "object", "additionalProperties": false, "required": ["situation", "goal", "evaluation_criteria", "reference_responses", "concept_ids"], "properties": {"situation": {"type": "string", "minLength": 1}, "goal": {"type": "string", "minLength": 1}, "evaluation_criteria": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "reference_responses": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "minLength": 1}}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}',
    '{"type": "object", "additionalProperties": false, "required": ["text"], "properties": {"text": {"type": "string", "minLength": 1}}}',
    '{"type": "object", "additionalProperties": false, "required": ["concept_results", "feedback"], "properties": {"concept_results": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "object", "additionalProperties": false, "required": ["concept_id", "correct"], "properties": {"concept_id": {"type": "string", "format": "uuid"}, "correct": {"type": "boolean"}}}}, "feedback": {"type": "string"}}}'
),
(
    uuidv7(),
    'dialogue',
    'Read a bilingual dialogue',
    'Encounter a short bilingual dialogue without grading.',
    'reading',
    'Generate two to six short turns with consistent speakers, familiar content, target-language text and English translations. The learner advances with Continue. Completion records exposure only; do not infer comprehension or correctness.',
    '{"type": "object", "additionalProperties": false, "required": ["turns"], "properties": {"turns": {"type": "array", "minItems": 2, "maxItems": 6, "items": {"type": "object", "additionalProperties": false, "required": ["speaker", "target", "english", "concept_ids"], "properties": {"speaker": {"type": "string", "minLength": 1}, "target": {"type": "string", "minLength": 1}, "english": {"type": "string", "minLength": 1}, "concept_ids": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}}}}',
    NULL,
    '{"type": "object", "additionalProperties": false, "required": ["completed", "concept_exposures"], "properties": {"completed": {"type": "boolean"}, "concept_exposures": {"type": "array", "minItems": 1, "maxItems": 10, "items": {"type": "string", "format": "uuid"}, "uniqueItems": true}}}'
);

INSERT INTO exercise_definition (
    id, kind, name, description, category, instructions, schema, answer_schema, verdict_schema
) VALUES (
    uuidv7(),
    'transliterate',
    'Write the Latin-script form',
    'Read a character or short text and type its Latin-script representation.',
    'reading',
    'Show one character or a short text in a script available for the exact target. Ask for its Latin-script representation, not its English meaning or letter name. Set script_id to that source script and explicitly name one romanization_system. For introductory hiragana and katakana prefer a single kana; allow multi-letter Latin answers and later kana combinations. For Arabic distinguish consonant transliteration from a vocalized reading; include vowel marks or context when a full reading would otherwise be ambiguous. For Han characters provide sufficient word or sentence context to select the intended reading; explicitly state the tone convention for Pinyin. List acceptable answers only under the chosen system and specify case sensitivity. Attribute evidence to script_reading concepts, not word meaning or grammatical mastery.',
    '{"type":"object","additionalProperties":false,"required":["text","script_id","romanization_system","accepted_answers","case_sensitive","concept_ids"],"properties":{"text":{"type":"string","minLength":1},"script_id":{"type":"string","pattern":"^[A-Z][a-z]{3}$"},"romanization_system":{"type":"string","minLength":1},"context":{"type":"string","minLength":1},"accepted_answers":{"type":"array","minItems":1,"maxItems":10,"uniqueItems":true,"items":{"type":"string","minLength":1}},"case_sensitive":{"type":"boolean"},"concept_ids":{"type":"array","minItems":1,"maxItems":10,"uniqueItems":true,"items":{"type":"string","format":"uuid"}}}}',
    '{"type":"object","additionalProperties":false,"required":["text"],"properties":{"text":{"type":"string","minLength":1}}}',
    '{"type":"object","additionalProperties":false,"required":["concept_results","feedback"],"properties":{"concept_results":{"type":"array","minItems":1,"maxItems":10,"items":{"type":"object","additionalProperties":false,"required":["concept_id","correct"],"properties":{"concept_id":{"type":"string","format":"uuid"},"correct":{"type":"boolean"}}}},"feedback":{"type":"string"}}}'
);
