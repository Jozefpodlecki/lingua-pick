# Roadmap

This roadmap records milestones, not delivery dates. Checkboxes describe implementation status; detailed requirements remain in the linked specifications. Track concrete work in [tasks](tasks.md).

## 1. Application foundation

- [x] Rust/Yew application shell with shared layout and hash routing.
- [x] Searchable picker limited to four card suggestions.
- [x] Root-level JSON catalogue with 100 learning options, including varieties.
- [x] Shared selected target, browser persistence, and top-bar switching.
- [x] Shared `no_std`/`alloc` core with language IDs, single-choice exercises, and basic sessions.
- [x] Contributor standards, architecture, UI, exercise, and AI interaction documentation.
- [x] User-added Tauri application crate, maximized-window configuration, and frontend runtime detection.
- [x] Target-specific sample loading for web-only runs.
- [x] HTTP catalogue loading with explicit loading, error, and retry states.
- [x] Reusable shell/exercise component crates and a separate browser/Tauri API bridge.
- [x] Native DuckDB store foundation with transactional migrations and session/statistics schema.
- [ ] Language-card imagery and browser interaction verification.

Outcome: choose and change a target reliably. Three targets support sample sessions; catalogue entries do not imply full courses exist.

The user-added Tauri crate in `crates/lingua-app` is part of the workspace. Native generation and packaged desktop verification remain pending.

## 2. First playable learning session

- [ ] Choose the first target and vocabulary categories to implement.
- [ ] Add its separate course crate with validated vocabulary and sample exercises.
- [x] Render single-choice exercises with selection, explicit submission, and feedback for browser samples.
- [x] Connect the existing core session to the learning screen for browser samples.
- [x] Show a completed-session summary.
- [x] Enforce and display ten exercises per session.

Outcome: complete a short authored session without needing an AI server. Follow [learning core](learning-core.md), [UI](ui.md), and [single choice](exercises/single-choice.md).

## 3. Translation and matching

- [ ] Resolve grading, normalization, and matching-attempt policy.
- [ ] Extend core exercise and answer models for translation and matching.
- [ ] Implement translation in both directions with typing and word-bank modes.
- [ ] Implement matching with independently shuffled columns.
- [ ] Integrate both formats with sessions and feedback.

Outcome: three usable exercise formats. Follow [exercise specifications](exercises/README.md) and [AI interaction requirements](ai-interaction.md).

## 4. LM Studio generation

- [x] Add a provider-neutral AI facade and validated generation request boundary.
- [x] Add the native LM Studio prompt client, Tauri command, and `lingua-api` invoke adapter.
- [ ] Add versioned JSON schemas and generation request types.
- [ ] Provide selected target, available/selected categories, vocabulary, and level constraints.
- [ ] Add connection settings and a browser generation service.
- [ ] Validate generated exercises before admitting them into a session.
- [ ] Handle generation failures, cancellation, and late responses after target changes.
- [ ] Verify the actual model locally and test deployed-origin access separately.

Outcome: generate exercises from LM Studio while retaining authored sessions when generation is unavailable. Follow [integration proposal](llm-integration.md) and [AI contract](ai-interaction.md).

## 5. Progress and repeat learning

- [ ] Start from unknown knowledge and distinguish exposure, activity completion, practice, and assessment evidence.
- [ ] Add Continue-only bilingual dialogue with optional volume/microphone practice; see [dialogue](exercises/dialogue.md).

- [ ] Define progress records per target and a versioned persistence format.
- [ ] Save completed-session outcomes and restore progress after reload.
- [ ] Decide unfinished-session resume behavior and repeat/review policy.
- [ ] Surface useful learning progress without unnecessary UI copy.

Outcome: switching languages preserves each target's learning history. Scheduling, streaks, and spaced repetition are not yet agreed features.

## 6. Course coverage and deployment

- [ ] Establish supported formats, scripts, categories, and curriculum metadata per course.
- [ ] Expand beyond the first target with a separate crate per learning option.
- [ ] Expand the catalogue toward 400 options as meaningful target coverage develops.
- [ ] Add sourced/licensed imagery and review script/accessibility behavior.
- [ ] Enable meaningful CI checks and verify GitHub Pages deployment.

Outcome: broader usable course coverage, not merely a longer picker. The [coverage table](exercise-coverage.md) is a proposal until courses and renderers exist.

## Open decisions

- First implemented target and initial vocabulary categories.
- Translation grading for valid answers absent from the reference list.
- Whether incorrect matching attempts affect the session score.
- Scope of English-source/English-target catalogue options.
- Whether and how unfinished sessions resume after switching or reloading.
- Model choice and generation limits after testing available local hardware.
- Native generation settings and packaged desktop verification.

Update this file when a milestone changes. Do not mark a milestone complete because its documentation or a lower-level model exists.
