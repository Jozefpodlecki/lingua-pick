# Tasks

Actionable work for the [roadmap](roadmap.md). Checked items are implemented; unchecked items are pending, not promises of an agreed design. Keep the detailed behavior in the specifications and update task status alongside the implementation.

## Completed foundation

- [x] Add shared layout and top bar.
- [x] Implement searchable language cards and selected-target switching.
- [x] Persist selected target and require local storage at startup.
- [x] Move catalogue to `assets/languages.json` and expand to 100 options.
- [x] Support keyboard suggestions, ranked search, and common accent folding.
- [x] Extract picker/card components and use data attributes for UI state.
- [x] Add `lingua-core` with `no_std` and `alloc`.
- [x] Add validated single-choice content and ordered session logic.
- [x] Document UI, exercise mechanics, target coverage, and AI interaction proposals.

## Next: authored single-choice session

- [ ] Decide the first target and a small set of word categories. Chinese is a discussed example, not an agreed first course.
- [ ] Define the course contract for vocabulary, categories, and supported exercise formats before adding the first target crate.
- [ ] Add that target's crate to the workspace with sample content using core models.
- [ ] Add a reusable exercise frame and single-choice answer-card component.
- [ ] Wire session construction, answer submission, feedback, continuation, and summary to the learning route.
- [ ] Preserve the answered exercise during feedback: the current core advances when an answer is recorded.
- [ ] Validate the completed flow with keyboard and pointer interaction.

Acceptance: a selected supported target can complete an authored session, see feedback and a summary, and change language through the top bar. Unsupported targets must not silently use another course.

## Core exercise extensions

- [ ] Decide translation normalization and handling of alternative valid answers.
- [ ] Decide matching-attempt scoring and final submission semantics.
- [ ] Add translation content with direction, accepted answers, tokens, canonical token sequence, and joining policy.
- [ ] Add matching content with unique pairs and complete answer mappings.
- [ ] Extend session submission beyond single-choice IDs without consuming exercises on malformed input.
- [ ] Add focused validation, grading, and progression tests after implementation settles.

Acceptance: valid answers can be graded for all implemented formats; invalid payloads and submissions return errors without corrupting session progress. See [exercise specifications](exercises/README.md).

## Exercise UI

- [ ] Implement translation typing and word-bank modes with separate preserved drafts.
- [ ] Support both translation directions and answer-language token joining.
- [ ] Implement matching columns with keyboard-operable selection and stable identities.
- [ ] Add accessible feedback, duplicate-submit prevention, and input-method composition handling.
- [ ] Keep visible copy minimal and presentation state in data attributes.

Acceptance: each interaction works on small screens and with keyboard input; switching answer modes does not regenerate or submit the exercise. See [UI](ui.md).

## LM Studio integration

- [ ] Store complete versioned response schemas in `assets/schemas/`, including translation word banks.
- [ ] Implement request context for exact target variety, available/selected categories, constraints, and requested format.
- [ ] Add endpoint/model settings and optional session-only authentication token handling.
- [ ] Add model discovery and non-streaming generation requests outside the core.
- [ ] Parse the API envelope and JSON content, then apply core and request-context validation.
- [ ] Handle unavailable model/server, malformed output, truncation, timeouts, and bounded retries.
- [ ] Prevent stale responses from affecting a changed target or cancelled session.
- [ ] Test with an actual structured-output-capable model and verify CORS/deployed-browser access.

Acceptance: validated generated content can start a session, failures are actionable, and authored content remains usable without LM Studio. See [integration](llm-integration.md) and [AI contract](ai-interaction.md).

## Progress, coverage, and maintenance

- [ ] Define and persist versioned progress records per target.
- [ ] Decide session resume and review behavior before implementing them.
- [ ] Resolve the purpose of regional English targets with English as the source.
- [ ] Add per-course format availability rather than treating the documentation matrix as runtime configuration.
- [ ] Add relevant card images with recorded sources/licenses.
- [ ] Expand target courses and update the catalogue/coverage table together.
- [ ] Enable actual CI validation and verify GitHub Pages routing and asset paths.
- [ ] Clarify the intended scope of `crates/lingua-app` before planning desktop integration work.
- [ ] Verify picker behavior in a browser; resolve repository-wide formatting issues as a separate change.

## Working rule

Implement and settle structure first, write meaningful tests last, then complete required checks before marking work done. Documentation-only tasks require link/content checks rather than an application build. Keep unrelated cleanup out of feature changes.
