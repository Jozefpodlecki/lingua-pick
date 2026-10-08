# Dialogue

## Purpose and status

Introduce language through a conversation with English translations. The learner only needs to click Continue to advance; no typed answer, word selection, or speech submission is required.

This is a documented requirement. The core variant, renderer, playback, and microphone practice are not implemented.

## Content

A dialogue is an ordered list of turns bound to the selected target variety. Each turn requires a stable ID, target-language text, corresponding English translation, and a speaker identity when relevant.

Optional proposed metadata includes readings, audio references, and stable vocabulary/skill IDs for [progress tracking](../progress.md). Text generation alone does not supply playable audio.

## Screen structure

```text
Speaker, when relevant

Target-language text
English translation

[Volume]  [Microphone]

[Continue]
```

The target text appears above English. Keep both readable and visually distinct. No repeated language labels are needed on every turn. The volume and microphone widgets sit near the text; their exact placement is flexible.

Use accessible names and keyboard-operable controls for icon widgets, with visible playback/recording states. Continue remains available without using either widget.

## Continue

Advance one turn per activation. After the final turn, complete the dialogue and show completion feedback or the next session step. Prevent repeated activation from skipping turns or recording completion twice.

This is an ungraded activity. Do not count turns as correct answers or submit fake choice IDs to the existing session model.

## Volume

Play the current turn in the target language. Show loading/playing state and allow playback to stop. Stop old audio on activity exit or target change. English playback is not required.

The source is undecided: authored audio or a configured synthesis provider could supply it. If unavailable, show the widget as unavailable with an accessible explanation; do not claim playback occurred.

## Microphone

Offer optional speaking practice for the current turn. Capture begins only after explicit user action and any required platform permission. Show recording state and stop/cancel controls. Stop capture on exit or target change.

Recording and replaying an attempt is a possible first implementation. Transcription, pronunciation assessment, and scoring are not agreed features. Capturing audio alone never establishes correctness.

Unavailable hardware or denied permission must not block Continue. Runtime-specific audio capabilities need explicit implementation; sample mode does not imply a speech evaluator exists. Recordings are temporary by default.

## Validation and progress

Require a nonempty ordered turn list, unique turn IDs, nonblank target/English text, and valid target/content references. Course review checks aligned translations and suitable difficulty; JSON validation does not prove linguistic correctness.

Record exposure, turn/activity completion, and real optional practice events separately from mastery. See [progress](../progress.md).

If AI dialogue generation is added, extend the prompt, schema, core models, and validation together. The existing AI contract does not yet define a dialogue response schema.
