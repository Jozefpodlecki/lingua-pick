INSERT INTO exercise_type
(
    name,
    description,
    modality
)
VALUES
('translate-sentence', '
This exercise type requires the learner to translate a complete sentence from the source language into the target language. The goal is to evaluate the learner’s ability to correctly map meaning, grammar, and vocabulary between languages while producing a natural sentence in the target language.

The system will provide:

a sentence written in the source language

optional context information (topic, difficulty level, or vocabulary hints)

the expected target language

The learner must produce a translation that preserves the original meaning as accurately as possible while following the grammatical and stylistic conventions of the target language.
', 'writing')