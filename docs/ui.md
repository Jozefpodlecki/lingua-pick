# UI specification

## Status

The picker, shared top bar and footer, selected-language persistence, and sample single-choice sessions with feedback and summary are implemented for web-only runs. Desktop generation is not connected. Language imagery, translation/matching screens, and generation controls remain requirements or proposals.

## Home and language switching

- One heading: `What would you like to learn?`.
- Search with an accessible name and a short placeholder. No repeated action descriptions, visible search label, keyboard tutorial, or result count.
- At most four card suggestions. No internal results scrollbar.
- Search names, native names, regions, identifiers, and aliases. Preserve keyboard selection and empty-result feedback.
- Cards identify the language and variety clearly. Region/culture wallpaper imagery is planned; flags are not the language identifier.
- The top bar shows the current target and a short `Change` action. Keep it available outside an active session.
- Changing language returns to the picker without clearing the existing choice. Choosing another target must preserve any future progress belonging to the old target.

## Language dashboard

Selecting a language opens `#/learn`, with three action cards: **Run exercise**, **Analyze stats**, and **Exercise history**. Cards use the existing icon library and contain no redundant descriptive text.

- Run exercise opens `#/learn/exercise` and starts the runtime-appropriate session loader.
- Analyze stats opens `#/learn/stats`.
- Exercise history opens `#/learn/history`.

Stats and history retain the shared top bar and offer a back link to the dashboard. The exercise destination enters the focused session layout. Stats and history currently show explicit not-recorded/not-saved states; session outcomes are not persisted yet. Do not show invented metrics or imply that completed sessions were saved.

## Shared footer

Every routed page outside an active session shows a footer with `Jozef Podlecki © 2026`, a link to the GitHub repository, and the detected runtime mode. The visible modes are Tauri desktop, local web, GitHub Pages, and hosted web. The footer reads the runtime already stored in application context and does not run a second environment check.

## Learning screen

An active session uses a focused full-screen layout without the shared top bar, footer, or target-language title. Show the exercise prompt, its answer controls, and only the instruction needed to understand the task. Do not repeat the selected target or that English is the source language. Error and completion states may offer a direct return to the dashboard.

Use the top area normally occupied by navigation for session progress. Show the current position and percentage in text, expose progress semantics to assistive technology, and use color as an additional cue. The implemented completion stages use muted rose through 33%, amber through 66%, and emerald from 67% onward. These colors describe session completion rather than answer correctness.

Use a shared exercise frame and separate components for [single choice](exercises/single-choice.md), [translation](exercises/translate-sentence.md), and [matching](exercises/match-words.md). Reusable exercise presentation belongs in `lingua-web-exercise`; route and provider orchestration remains in the application. The frame owns common layout and feedback; each interaction owns its draft and input controls. The core owns answer validation and session progression.

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

## Dialogue controls

Show target-language text above its English translation, with volume and microphone widgets nearby and a Continue action. Listening and speaking are optional; no assessed answer is required to advance. See [dialogue](exercises/dialogue.md) for the full behavior and [progress](progress.md) for completion versus knowledge evidence.

## Errors, accessibility, and presentation

- Keep user-facing errors short and actionable. Keep API details in diagnostics.
- Use semantic elements, accessible names, visible focus, and announced status changes.
- Use `data-*` attributes for presentation state while retaining appropriate ARIA semantics.
- Preserve legible text, regional labels, and adequate contrast over future images.
- Let right-to-left text follow its own direction; do not mirror unrelated English controls automatically.
- Support small screens without forcing horizontal scrolling. A small device may still need ordinary page scrolling.
- On target changes, cancel or ignore old generation requests and discard the old exercise draft. Saving or resuming an unfinished session remains undecided.
