# Learner knowledge and progress

Status: requirements and a partially implemented storage model. Selected-language persistence exists. The native DuckDB schema and repositories exist, but the active session flow does not write outcomes yet, so history and statistics remain empty.

## Initially unknown knowledge

When a learner starts a target, the app has no evidence of what they know. Unknown is different from zero knowledge or an incorrect answer. Do not infer mastery or a curriculum level merely from selecting a language.

Start with introductory content, record encounters and responses, and use accumulated evidence to select appropriate exercises. An initial assessment or self-reported level could be considered later; neither is an agreed feature yet.

## Evidence

| Event | Establishes | Does not establish |
| --- | --- | --- |
| View a dialogue turn | Exposure to content | Independent comprehension |
| Continue through the dialogue | Activity completed | Vocabulary or speaking mastery |
| Play audio | Listening practice attempted | Understanding the recording |
| Capture a microphone attempt | Speaking practice attempted | Correct pronunciation without evaluation |
| Submit a graded answer | Result under that grading policy | Permanent mastery from one result |

Completion, exposure, practice, and assessed performance are separate. Continue-only dialogue belongs in history as an ungraded activity; it must not produce fictitious correct answers or increase assessed accuracy.

## Proposed records

Keep progress independent for each exact target ID, including varieties. Switching languages must not overwrite another target's progress.

- Activity history: stable activity/session ID, target ID, exercise ID and type, content version, timestamps, completion status, and applicable answer/practice events.
- Learning-item evidence: stable vocabulary or skill ID, exposures, last encounter, graded attempts where applicable, and a derived knowledge state.

Exercises should identify the learning items they cover. An exercise ID is not a vocabulary ID. Content versions or snapshots preserve the meaning of old history when course material changes.

Proposed evidence states are `unknown`, `encountered`, and `assessed`. They do not define a mastery score. Mastery thresholds and review scheduling remain undecided.

## Dialogue tracking

Record encountered turns and learning items, completed turns, current position, and whole-dialogue completion. Record playback or microphone practice only when it actually occurs, not merely when an unavailable widget is clicked.

Continue advances one turn. Finishing the last turn completes one dialogue activity, not one correct answer per turn. Prevent duplicate advancement and completion records. Audio replay or speech retries must not duplicate completion.

Leaving midway records an incomplete activity. Resuming automatically remains undecided. Later graded exercises can assess items introduced through dialogue.

## Persistence, stats, and history

Keep storage and timestamp adapters outside the `no_std` core. Tauri uses the migrated DuckDB store described in [native storage](storage.md); browser-only runs require versioned browser-storage records. Local storage remains required at browser startup. Report failed writes rather than implying progress was saved.

History shows actual saved activities, including ungraded dialogues and incomplete activities when recorded. Stats distinguish activity completion from assessed answers. Accuracy includes only applicable graded answers; unknown/unavailable values replace invented metrics.

The current core `Session` advances through graded single-choice submissions only. Dialogue requires an explicit ungraded progression operation, not a fake choice answer. Platform-independent evidence/outcome models should live in the core.

Raw microphone recordings are not necessary for progress tracking. Keep them temporary by default; persistent audio/transcripts need a separate retention decision. Accounts, sync, and shared storage across browser origins are not implemented.

See [dialogue](exercises/dialogue.md) for the exercise behavior.
