# Single choice

## Purpose and status

Recognize the meaning or correct interpretation of a prompt by selecting one answer. The core implements `ExerciseContent::SingleChoice`; browser sample sessions have answer cards, explicit checking, feedback, and summary. Native generation remains pending.

## Content

An `Exercise` supplies target, instruction, prompt text, optional script/reading, and optional curriculum metadata. `SingleChoice` supplies choices with stable IDs and one correct-choice ID.

Chinese example: show `猫` and three English cards, `cat`, `dog`, and `horse`. Three cards are an example, not a core limit: the constructor requires at least two choices.

## Interaction and grading

Implemented UI: select one card, then explicitly check the answer. A selection alone does not submit it. Selection is distinct from correctness feedback.

The core grades by choice ID. A known incorrect choice produces an incorrect outcome and advances the session. An unknown choice returns an error without advancing. Display feedback using the recorded outcome and proceed to the next exercise only when the learner continues.

## Validation and limitations

Choice IDs must be unique and nonblank, choice text must be nonblank, and the correct ID must exist. Constructor validation also runs during deserialization.

Course authors must avoid ambiguous or duplicate-looking alternatives; the current constructor does not reject duplicate display text or establish semantic correctness. Distractors should be plausible but demonstrably wrong in the supplied context. Script or curriculum labels do not guarantee syllabus alignment.
