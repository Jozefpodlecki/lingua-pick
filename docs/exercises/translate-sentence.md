# Translate sentence

## Purpose and status

Practice understanding and producing a sentence. The interaction and JSON contract are documented requirements; neither the core variant nor the exercise UI exists yet.

## Content and direction

Require a sentence, direction, accepted answers, word-bank tokens, a canonical token sequence, joining policy, and optional learner explanation. The request binds the exercise to the selected target variety.

| Direction | Sentence | Answer |
| --- | --- | --- |
| `target_to_english` | Target language | English |
| `english_to_target` | English | Target language |

The [AI contract](../ai-interaction.md#translation-directions-and-input-modes) is authoritative for the proposed payload and examples; avoid duplicating the JSON structure here.

## Interaction

Every exercise supports typing and word-bank modes. Keep separate drafts when switching. The learner submits explicitly; empty input or unfinished input-method composition must not submit.

Shuffle the bank independently of its canonical sequence. Learners select token instances in order and can remove them. Duplicate words need distinct token IDs. Punctuation and joining must produce a supported answer without guessing spaces; Chinese word or phrase tokens can join without spaces.

## Grading

Both modes produce text for the same grading policy. An accepted normalized answer succeeds. A different canonical token order can still succeed if it reconstructs an accepted answer.

The exact normalization and treatment of unlisted but valid translations remain undecided. Do not remove accents indiscriminately, equate simplified/traditional forms automatically, or present reference matching as complete linguistic assessment. Model-assisted evaluation, if added, is a separate feature with an uncertainty policy.

## Validation and feedback

Validate direction, answer-language constraints, nonblank content, bank IDs, canonical references, and a reconstructable accepted answer. Use course vocabulary and script restrictions where supplied. Reject a bank with no solution before showing it.

Proposed feedback displays correctness and a reference translation, with a brief explanation only when useful. Retry/scoring behavior must be specified before connecting this type to `Session`; the current session accepts only choice identifiers.
