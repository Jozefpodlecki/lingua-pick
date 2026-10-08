
## Experimental harness data

- [x] Refine the in-memory schema in 001 for pedagogical stages, skills, prerequisites, guidance, and mode-specific evidence.
- [x] Seed matching-first pedagogical guidance and later grammar/sentence guidance without lexemes.
- [x] Expand the exercise-definition data migration to 35 experimental definitions with generation instructions and JSON contracts, including script transliteration.
- [x] Load the 100-option language catalogue through migration 002 and remove runtime JSON loading from seeding.
- [x] Seed per-target exercise availability for all 100 targets in migration 005, omit same-language matching/translation for English, and intersect availability with teaching-stage definitions.
- [x] Seed script mappings for every catalogue target so available transliteration definitions have script context.
- [ ] Define English-target introductory progression without same-language word matching.
- [x] Add script reference tables, initial Japanese/Chinese/Latin associations, typed stores, and available scripts in generation requests.
- [x] Make the harness compile with explicit generation, simulation, and evaluation stubs and exercise persistence APIs.
- [ ] Define advancement policy against actual learning evidence.
- [x] Implement typed pedagogical, concept, turn, and evidence repositories and correct existing stores.
- [x] Commit evidence/completion/session advancement atomically with derived counters and target ownership checks.
- [x] Build experimental generation context from learner state and pedagogical repositories.
- [x] Implement one-exercise LM Studio generation with JSON Schema validation, reference checks, and application-owned concept IDs.
- [x] Implement learner simulation with configurable mistake percentages, independent answer evaluation, and atomic outcome/evidence completion.
- [x] Remove the obsolete generator implementation.
- [x] Split seeding into `seed/`, persist the harness database to a file, and resolve existing stores/state without reseeding.
- [x] Move the exercise loop into a runner with configuration, context loading, persistence, and pending-exercise/saved-answer reuse.
- [x] Add tracing-based file logging for exact AI prompts/responses, call durations, validation errors, and session progress.
- [x] Add generator store tools, bounded AiMux conversation handling, retrieved-concept validation, and tool-aware system instructions (compilation and live verification pending).
- [x] Format new tracing log files as readable multiline text with tool arguments/results and conversation context (execution verification pending).
- [ ] Compile and verify native tool calls with the loaded model after the active runner finishes.
- [x] Extend topic and skill catalogues in separate data migrations, with stage guidance, prerequisites, and exercise/skill mappings.
- [x] Name the table `exercise_category` directly in the experimental initial schema and align model, store, and seed names.
- [x] Move store SQL into private queries modules and document readable Rust/SQL formatting conventions.
- [x] Add concept seed files for all 100 catalogue targets; populate pt-BR with 100 vocabulary/grammar/sentence concepts, language associations, and prerequisites while leaving other target files empty.
- [ ] Verify the revised migrations and recreate the disposable harness database after the active runner finishes.
- [ ] Verify a complete live-model session and review generated/simulated/evaluated linguistic content.

See [experimental learning data](experimental-learning-data.md). The runtime loop is implemented; complete live-model validation and stage advancement remain pending.

1. Proper architecture

At the start of the app we initialize necessary things

app needs to check local storage whether selected language exist, if not we have to reroute to home page

LanguagePicker should be able to distingush whether we currently have an active language and give different propmt than
"What would you like to learn?

2. components shouldnt do a lot of logic aside from render, that's why we have context, providers reductible etc
3. tauri setup should be in separate module lib.rs should be slim
