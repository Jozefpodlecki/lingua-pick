# UI specification

## Status

The picker, shared top bar, selected-language persistence, and placeholder learning route are implemented. Language imagery, exercise screens, generation controls, feedback screens, and session summaries below are requirements or proposals, not working features.

## Home and language switching

- One heading: `What would you like to learn?`.
- Search with an accessible name and a short placeholder. No repeated action descriptions, visible search label, keyboard tutorial, or result count.
- At most four card suggestions. No internal results scrollbar.
- Search names, native names, regions, identifiers, and aliases. Preserve keyboard selection and empty-result feedback.
- Cards identify the language and variety clearly. Region/culture wallpaper imagery is planned; flags are not the language identifier.
- The top bar shows the current target and a short `Change` action. Keep it available on learning screens.
- Changing language returns to the picker without clearing the existing choice. Choosing another target must preserve any future progress belonging to the old target.

## Proposed learning screen

Show the exercise prompt, its answer controls, and only the instruction needed to understand the task. Do not repeat that English is the source language. Display session progress when sessions become available.

Use a shared exercise frame and separate components for [single choice](exercises/single-choice.md), [translation](exercises/translate-sentence.md), and [matching](exercises/match-words.md). The frame owns common layout and feedback; each interaction owns its draft and input controls. The core owns answer validation and session progression.

Proposed lifecycle:

```text
loading → answering → checking → feedback → next exercise → summary
                ↘ validation error → answering
loading → generation error → retry or leave
```

The current core advances immediately when a valid answer is submitted. The UI should retain the answered exercise and outcome during feedback, then reveal `Session::current()` when the learner continues. Do not submit a second answer just to advance past feedback.

Prevent duplicate submissions while checking. Invalid input is distinct from an incorrect answer: an unknown choice or malformed response must not consume an exercise.

## Translation controls

Offer `Type` and `Word bank` modes on the same exercise. Switching modes preserves separate drafts and does not regenerate content. Explicit submission records an answer.

The typed mode uses a labeled text input or textarea. Support the learner's keyboard and input method; do not submit while composing Chinese or other input-method text.

The bank mode shows available tokens and the assembled answer. Tokens can be selected and removed with keyboard or pointer. Track token instances by ID, including repeated words. Display spacing according to the answer language, not a universal space-between-tokens rule.

Translation direction determines prompt and answer language. Chinese-to-English uses English answer controls; English-to-Chinese uses Chinese answer controls.

## Matching controls

Show independently shuffled target and English columns. Selecting one item from each column attempts a pair. Indicate selected, matched, and incorrect states with more than color alone. Matching is also keyboard-operable; dragging is not required.

Pair completion and failed attempts remain interaction-local until the learner explicitly submits the complete matching answer. The core integration and attempt-scoring policy are pending; see [matching](exercises/match-words.md).

## Errors, accessibility, and presentation

- Keep user-facing errors short and actionable. Keep API details in diagnostics.
- Use semantic elements, accessible names, visible focus, and announced status changes.
- Use `data-*` attributes for presentation state while retaining appropriate ARIA semantics.
- Preserve legible text, regional labels, and adequate contrast over future images.
- Let right-to-left text follow its own direction; do not mirror unrelated English controls automatically.
- Support small screens without forcing horizontal scrolling. A small device may still need ordinary page scrolling.
- On target changes, cancel or ignore old generation requests and discard the old exercise draft. Saving or resuming an unfinished session remains undecided.
