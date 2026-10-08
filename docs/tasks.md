
## Experimental harness data

- [x] Refine the in-memory schema in 001 for pedagogical stages, skills, prerequisites, guidance, and mode-specific evidence.
- [x] Seed matching-first pedagogical guidance and later grammar/sentence guidance without lexemes.
- [x] Expand the exercise-definition data migration to 35 experimental definitions with generation instructions and JSON contracts, including script transliteration.
- [x] Load the 100-option language catalogue through migration 002 and remove runtime JSON loading from seeding.
- [x] Seed per-target exercise availability for all 100 targets in migration 006, omit same-language matching/translation for English, and intersect availability with teaching-stage definitions.
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
- [x] Clarify the generation envelope with `new_concepts` and reuse/new-slot examples, aggregate concept-resolution diagnostics, and allow twelve tool rounds plus a final turn (live-model verification pending).
- [x] Restore generator prompt, response, and tool argument/result diagnostics; report duplicate payload paths and conflicting items, and distinguish forced-final turn and call limits (live-model verification pending).
- [x] Add generator store tools, bounded AiMux conversation handling, retrieved-concept validation, and tool-aware system instructions (compilation and live verification pending).
- [x] Format new tracing log files as readable multiline text with tool arguments/results and conversation context (execution verification pending).
- [x] Add runner database modes: open-or-create, existing-only, recreate, and shared in-memory storage (runtime verification pending).
- [x] Expand nested JSON and embedded multiline text in logs and use readable formatting for file and console (runtime verification pending).
- [ ] Compile and verify native tool calls with the loaded model after the active runner finishes.
- [x] Extend topic and skill catalogues to 60 entries each in migrations 009/010, with per-topic stage guidance, acyclic skill prerequisites, and 65 exercise/skill mappings. Source references were checked; SQL execution remains pending.
- [x] Name the table `exercise_category` directly in the experimental initial schema and align model, store, and seed names.
- [x] Move store SQL into private queries modules and document readable Rust/SQL formatting conventions.
- [x] Add concept seed files for all 150 experimental catalogue targets, leaving non-pt-BR files empty. The initial word-specific pt-BR content is superseded.
- [ ] Replace the pt-BR lexical seed with broad topic-and-skill objectives and add the topic relationship to schema, models, stores, and generator context.
- [ ] Introduce typed `PracticeFocus` filters for topic, part of speech, content unit, and exercise type; enforce compatible target/stage selections before generation.
- [ ] Connect practice-focus selection to the UI after the harness contract is established.
- [x] Expand the experimental SQL catalogue to 150 options with 50 regional targets, exercise profiles, script associations, and empty concept seed files; frontend catalogue remains unchanged.
- [x] Add language-feature definitions, allowed values, assignments, and typed badge reads; seed 12 features, 41 values, and 52 starter assignments.
- [x] Reserve shared migrations 001-010 and move all target concept seeds to 100-249, updating the registry.
- [ ] Expand reviewed language-feature assignments beyond the starter article and noun-gender metadata.
- [ ] Expose language-feature badges in the UI and include relevant metadata in generation context.
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
