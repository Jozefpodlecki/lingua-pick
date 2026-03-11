INSERT INTO exercise_type
(
    name,
    description,
    modality
)
VALUES
(
    'translate-sentence',
'
This exercise type requires the learner to translate a complete sentence from the source language into the target language.
The goal is to evaluate the learner’s ability to correctly map meaning, grammar, and vocabulary between languages while producing a natural sentence in the target language.

The system will provide:
- a sentence written in the source language
- optional context information (topic, difficulty level, or vocabulary hints)
- the expected target language

The learner must produce a translation that preserves the original meaning as accurately as possible while following the grammatical and stylistic conventions of the target language.
',
    'writing'
),
(
    'draw-hanzi',
'
This exercise type requires the learner to draw a Chinese character (Hanzi) directly in the app using touch or mouse input.
The goal is to practice correct stroke order, proportion, and shape.  
The app will provide the target character as a prompt, and the learner must reproduce it by drawing on the screen.
The system may evaluate accuracy of strokes, order, and overall form.
',
    'writing'
),
(
    'match-lexemes',
'
This exercise requires the learner to pair words (lexemes) with their meanings, translations, or categories.
The goal is to test vocabulary recognition, recall, and association skills.  
The user typically selects or drags items in the app to form correct pairs. The system checks correctness automatically.
',
    'interactive'
),
(
    'select-translation',
'
This exercise presents a single lexeme and multiple translation options.
The learner must select the correct translation from the provided choices.
The goal is to test vocabulary recognition and comprehension.
',
    'interactive'
);