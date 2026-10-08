# Experimental learning data

## Scope and migration layout

`crates/lingua-cli` is a file-backed testing harness with generation, learner simulation, answer evaluation, and transactional completion. Automatic stage advancement remains undecided.

The harness generates exercises, simulates graded answers, and evaluates them through the same configured LM Studio model. Each adapter has a separate prompt and validates its inputs and outputs. The session loop saves outcomes and synthetic concept evidence and reloads that evidence for subsequent generation. A complete live-model session has not yet been verified.

All experimental schema definitions belong in `001_initial.sql`, including `exercise_category`. Data migrations contain the language catalogue (002), language features (003), exercise categories (004), exercise contracts (005), language/exercise availability (006), pedagogical guidance (007), scripts (008), additional topics (009), additional skills (010), and target-specific concepts (100 through 249). This harness is not deployed and its database is disposable: schema and seed migrations may be edited directly, then rerun against a newly created database after the runner stops. This exception does not change the separate desktop store's immutable migration policy.

### Persistent startup

`seed/` separates database opening, initial learner state, saved-state resolution, store construction, and errors. `RunnerConfig` selects `database_mode` independently of `database_path`, which defaults to repository-root `lingua-cli.duckdb`. The existing `seed::open` API retains open-or-create behavior; `seed::open_with_mode` exposes the explicit mode.

| Mode | Startup behavior |
|---|---|
| `OpenOrCreate` (default) | Create and seed a missing file; verify and resume an existing file. |
| `ExistingOnly` | Require an existing file and resolve its saved state; do not create a database, seed content, or upgrade migrations. Session execution still writes progress. |
| `Recreate` | Replace the configured disposable database and its `.wal` sidecar on every startup, then initialize fresh content and learner state. |
| `InMemory` | Create a fresh shared in-memory DuckDB instance for the pool; ignore the configured file path and discard data when its handles close. |

Set the mode in `RunnerConfig::local`, for example `database_mode: DatabaseMode::InMemory`. `Recreate` checks regular-file paths and opens/closes an existing database before removal, allowing DuckDB to reject a lock held by another process or an invalid database file. Do not run recreation concurrently with another runner. No recursive deletion is used; only the configured file and its exact `.wal` sidecar are removed. The mode is logged at startup. Open-or-create remains the default, so adding this option does not automatically reset existing progress.

Fresh file and in-memory modes apply all 160 registered migrations, then insert the learner, statistics, and session transactionally. Existing files only undergo migration ledger validation and saved-state resolution at startup. Missing migrations, changed checksums, mismatched source languages, absent statistics, missing sessions, or ambiguous learner names return errors rather than reseeding. After editing migrations, use `Recreate` on a stopped disposable database or choose `InMemory` for an isolated fresh run. Startup never silently upgrades an incompatible existing file.

Saved-state resolution chooses the newest unfinished session for the requested learner and target, falling back to the newest completed session. A completed session causes the current loop to finish without adding exercises; creating subsequent sessions remains a separate action. The startup resolver does not repair partially initialized files or upgrade older files automatically. The database and its write-ahead log are excluded from version control.

Migration 008 supplies 27 script identifiers and 154 associations covering all 150 experimental targets. `script` stores four-letter ISO 15924 identifiers and names; `language_script` allows multiple scripts per exact target. Japanese is associated with `Hira`, `Kana`, and `Hani`; the Chinese targets use `Hans` or `Hant` according to their catalogue identity. Arabic, Persian, Urdu, and Pashto use `Arab`. Serbian and Uzbek have Latin and Cyrillic associations. Other entries use the selected modern writing system; these seed choices do not catalogue every historical or alternative orthography. An empty association list never implies Latin. Identifiers follow the [ISO 15924 code list](https://www.unicode.org/iso15924/iso15924-codes.html).

`ScriptStore` and `LanguageScriptStore` provide typed reads and association writes and are available through `SeedResult`. Each generation request includes the selected target's available scripts. Scripts describe writing systems, not vocabulary, pronunciation systems, or exercise availability.

Chinese words may contain multiple characters and remain ordinary Unicode text in existing vocabulary and sentence payloads. Do not restrict word fields to one character. Script metadata distinguishes simplified and traditional writing; it does not determine a character's pronunciation. Mandarin Pinyin readings require separate romanization and tone policy, and readings with multiple possibilities require context. Script-reading exercises and pronunciation grading are not implemented by adding these reference tables.

Migration 002 inserts 150 experimental learning options in one SQL statement: the original 100 entries from `assets/languages.json` plus 50 regional targets. This embeds the catalogue in the harness binary; seeding no longer reads JSON at runtime. The shared frontend catalogue still contains 100 options and has not been expanded by this harness change.

UUID primary keys default to DuckDB's `uuidv7()`. Migration 005 generates definition IDs with that function; migrations 006 and 007 resolve references through unique exercise kinds instead of fixed UUID literals. IDs remain consistent in a saved file, while newly initialized files receive new IDs. Explicit application-generated IDs remain supported by existing repositories.

Migration 005 seeds 35 exercise definitions. Each has generation instructions and payload, answer, and verdict contracts; ungraded dialogue has no answer schema. These are experimental definitions, not implemented frontend interactions. Migration 006 supplies the experimental target availability registry, and migration 007 continues to limit the introductory stage to matching.

The catalogue includes:

- Vocabulary: matching, meaning selection, target-word selection, contextual meaning, synonyms, antonyms, recall, spelling completion, collocations, and idiom meaning.
- Grammar: blank completion, conjugation, articles, prepositions, pronouns, agreement, plurals, possession, negation, questions, tense changes, comparisons, conditionals, and error correction.
- Sentences and writing: translation in both directions, token ordering, word-bank construction, short responses, register selection and rewriting, and punctuation.
- Reading: passage questions and ungraded bilingual dialogue.
- Script reading: `transliterate` presents one character or short text and requests its Latin-script representation. Its payload contains `text`, `script_id`, `romanization_system`, `accepted_answers`, `case_sensitive`, and `concept_ids`, with optional disambiguating `context`. The answer is `{ "text": "..." }`. The generator rejects source scripts absent from the selected target's script mappings. Migration 007 supplies a separate `script_reading` skill. Migration 006 makes the definition available to targets with a seeded non-Latin script; a teaching-stage association remains undecided.

For hiragana and katakana, begin with a single kana and permit Latin answers longer than one letter. Arabic generation must distinguish consonant transliteration from a vocalized reading and supply vowel marks or context when needed. The named romanization system controls accepted answers; case sensitivity is explicit. Chinese readings require sufficient context and an explicit tone convention. Evaluation uses the accepted representations and case policy; frontend presentation remains pending. None of these answers establish knowledge of a word's English meaning.

Choices require unique IDs and exactly one referenced correct choice. Token exercises require valid accepted orders using available token IDs. Matching requires distinct labels and complete one-to-one submissions. These semantic rules need runtime validation beyond the stored structural JSON schemas. Open-response references are examples, not an exhaustive list of valid answers. Dialogue completion records exposure and never graded correctness.

## Pedagogical model

### Language availability

`006_data_language_exercise.sql` seeds 4,960 associations across all 150 exact target IDs. Every target has an explicit profile. The 50 additional regional targets initially inherit exercise availability and script associations from a related existing target; these are provisional profiles and do not establish linguistic equivalence or complete dialect coverage. A shared list of 28 exercise kinds covers vocabulary, general grammar, comprehension, and writing; seven additional kinds are selected individually through named profile columns: verb conjugation, articles, prepositions, agreement, plural formation, tense changes, and transliteration. Adding a new exercise definition does not automatically enable it everywhere. Missing referenced kinds fail the migration rather than silently producing an incomplete mapping.

Profiles are conservative content-authoring choices for the current exercise instructions, not an exhaustive linguistic classification. For example, Japanese has verb-form, tense, and script-reading exercises, while Mandarin has preposition and script-reading exercises without generic verb conjugation or obligatory noun pluralization. Postpositional languages use general blank completion rather than an exercise explicitly asking for a preposition. Conjugation instructions refer to a verb base form and only features actually marked by the selected target. The distinction between tense and aspect must be preserved; [WALS](https://wals.info/chapter/69) describes different strategies for marking them. Profiles remain editable as target-specific teaching requirements are reviewed.

English target varieties omit `match_words`, both translation definitions, and bilingual `dialogue`; grammar, reading, and writing remain available. English-target introductory progression needs a separate product decision because the current words stage exclusively permits matching. The harness reports an unavailable stage explicitly for those targets.

Language availability and teaching-stage suitability are independent. The harness intersects the selected target's mappings with the current stage's definitions before generation. Seeding no longer inserts matching availability for only the selected target. All target mappings come from migrations.

### Learning stages

Learning stages progress from word recognition to basic grammar and short sentences. New learners receive only `match_words`, with small sets of common words from familiar topics. Repeat matching while recognition evidence is insufficient. Later stages retain vocabulary review and introduce one grammar relationship at a time using familiar words, followed by scaffolded sentence comprehension and production.

Stage instructions guide generation and evaluation. Stage dependencies describe progression; stage skills identify evidence modes; stage exercise associations specify allowed interactions. Skill prerequisites describe capabilities independently of interaction type. Topics constrain generated subject matter. Teaching guidelines cover unknown knowledge, controlled novelty, scaffolding, regional consistency, ungraded exposure, and targeted review.

The current pt-BR concept seed includes individual vocabulary entries alongside grammar and sentence concepts. This granularity has been rejected and awaits replacement with broad topic-and-skill learning objectives, as described in the [accepted design](#accepted-concept-and-practice-focus-design). AI generates concrete exercise content at runtime; seed membership never establishes learner knowledge.

## Learner evidence

Evidence records the learner, exact target, exercise, concept, event identity, evidence mode, assistance, timestamp, evaluator, and evaluation policy version. Recognition, recall, production, and comprehension remain separate. Exposure and practice cannot have correctness. Unique exercise/concept/event keys prevent duplicated records.

The evidence view derives attempts, correct results, assessed session counts, accuracy, and last encounter by mode. Assisted attempts do not count as independent assessed success. Missing evidence means unknown knowledge. Legacy mastery/confidence fields are nullable and have no scoring rule.

Advancement thresholds remain undecided; no fixed attempt or accuracy threshold is seeded. Stage dependencies describe teaching order rather than automatically calculating eligibility. The future request builder must combine pedagogical guidance, topics, allowed interactions, recent history, and learner evidence. Consecutive matching exercises are appropriate when matching is the only suitable interaction.

Repository operations validate evidence ownership and target and support atomic completion as described below. The generator validates candidate payloads before persistence; storing a JSON schema alone does not validate content.

## Generator contract

The experimental generator uses the existing `lingua-ai::LlmClient` and loaded model. One request generates one exercise, independently of the frontend's proposed ten-exercise batch contract. The request includes source and target IDs, target statistics, teaching-stage instructions, topics, available skills and scripts, teaching guidelines, known concepts, mode-specific evidence, and up to five recent exercises from the current session. The harness currently selects the introductory words stage; automatic advancement remains undecided.

The fixed system prompt requires an envelope with exactly `kind`, `data`, and `new_concepts`. `kind` must identify an available definition. `data` must satisfy that definition's payload schema. `new_concepts` describes only newly created learning concepts through `id`, `code`, `name`, `description`, `skill_id`, and `topic_id`; it is empty when only known concepts are used. It does not include metadata for existing seeded concepts; reuse their IDs after retrieving them through tools.

The application allocates ten UUIDv7 concept slots before each call. The model may use those slots for new concepts or reuse known IDs. Unknown IDs, duplicate concept codes, unavailable skills, unused descriptions, and undescribed new references are rejected. The application supplies the target identity and resolves the selected definition's database ID. Exercise and session identity never come from the model. `GenerationResult` returns the candidate exercise and its referenced concept records; the harness stores new concepts and links them to the exercise before simulation.

The `jsonschema` dependency validates Draft 2020-12 payload contracts with UUID format checks enabled and network reference fetching disabled. Additional relationship validation rejects ambiguous matching labels, duplicate pair concepts, missing correct choices, invalid token references, repeated token use, and incomplete word orders. Payload strings must be nonblank and at most 4096 characters; completed responses are limited to 128 KiB. Compilation of the payload schemas and request validation occur before contacting the model. Calls do not retry or silently repair malformed output.

The harness uses `LlmClient::generate_streamed`, collecting the stream into a complete string before JSON parsing. This avoids the adapter's 30-second non-streaming exchange limit. Streaming generation has a 4096-token output cap, a 30-minute total timeout, a 10-minute first-chunk timeout, and a 5-minute idle timeout. Existing non-streaming callers retain their original method.

These checks establish structural and reference validity, not linguistic accuracy. Correct translations, regional naturalness, topic alignment, and teaching quality still require review. Generation does not invent proficiency or mastery. Transport, JSON, schema, and relationship failures remain distinct errors. The obsolete generator implementation has been removed.

The response field is `new_concepts`; the former `concepts` field is rejected. This changes only the model-response envelope, not persisted exercise or concept records. The prompt includes separate existing-ID and new-slot response patterns and forbids copying retrieved concepts into new slots. Concept resolution collects up to sixteen diagnostics before returning an error, including metadata paths, IDs, codes, rule names, and whether a code belongs to a retrieved concept. Unknown payload references include their exact paths. Multiple problems such as an unused new ID and a copied existing code are reported together; invalid candidates are never returned or automatically repaired.

## Generator store tools

The generator now uses AiMux function definitions and streamed structured responses to run a bounded conversation. The initial model request contains exercise contracts, source/target, the fixed stage, skills, topics, guidelines, scripts, learner statistics, and application-allocated concept slots. Growing concept, evidence, and recent-exercise arrays are omitted from model context by sending them empty; the system instructions explicitly distinguish this from an empty database. The runner still loads the full internal request for validation and simulation; avoiding those bulk internal reads is a later optimization.

Six read-only tools are registered in `src/tools/`: `search_concepts`, `get_concepts`, `get_learner_evidence`, `list_skills`, `get_teaching_context`, and `get_recent_exercises`. The runner supplies learner, exact target, session, and teaching context. Model arguments cannot change these identities, write records, or advance the stage. Concept search uses parameterized SQL and paginated literal text matching, optionally filtered by skill. Concept/evidence lookups reject IDs outside the current target. Evidence is queried for the captured learner and target. Teaching context is the current store-loaded snapshot; recent exercises cover the captured session.

Tool arguments pass JSON Schema validation and typed decoding. Calls run sequentially through `spawn_blocking`, keeping DuckDB reads off the async executor. Concept and skill pages contain at most 20 items, recent history at most five, and each successful tool result must fit within 64 KiB. Lookup and argument errors return structured tool results so the model can correct them. The loop allows thirteen model turns and twelve calls, counting failed lookups; at most twelve turns can request tools. Oversized call batches, duplicate call IDs, incomplete responses, and calls after tools are disabled stop generation. The last turn, or the turn following the twelfth call, requests the final exercise with tools disabled. The whole conversation shares a 30-minute timeout budget, including store calls.

Assistant response messages and matching tool-result messages are preserved between turns. Only existing concepts whose metadata was actually returned by successful tools can be referenced in the final exercise. New concepts still use application-allocated slots and valid skill IDs; their codes are checked against the target's database before persistence to reject duplicates outside the retrieved subset. The final response remains exactly one validated exercise JSON envelope. The generator prompt explains tool use, evidence semantics, prerequisites, pagination, duplicate avoidance, and final output requirements. Simulator and evaluator continue to use their supplied context without tools.

Compilation and a live capability check against the loaded model are pending: the user requested source changes without building or running tests while an existing runner remains active.

## Topic, skill, and exercise-category catalogues

Migration 009 adds 54 topics to the six introductory topics in 007, giving 60 in total. The expansion includes lifestyle, politics, civic participation, law, buildings, construction, agriculture, environmental subjects, culture, digital life, housing, finance, accessibility, and other everyday or specialist contexts. Each additional topic has a stage association and generation guidance that limits novelty and distinguishes word matching from later grammar and sentence tasks. These records describe learning content without seeding lexemes or learner mastery. Existing stage rules still restrict beginners to `match_words`.

Migration 010 adds 54 skills to the original six, giving 60 in total. Fine-grained skills cover contextual meanings, spelling, vocabulary relationships, grammatical constructions, register, word order, punctuation, and revision. Prerequisites express pedagogical relationships without defining advancement thresholds. The migration supplies 65 definition/skill associations across all 35 exercise definitions using their stable kinds. New objectives cover noun classes and cases, determiners, classifiers, aspect, mood, valency, clause construction, text interpretation, paraphrasing, summaries, requests, and discourse cohesion. One definition may support several skills; an individual exercise should assess only its selected objectives. Structural skills apply only where the target has the relevant construction. These mappings do not override target availability, introductory matching, or stage restrictions. Sound discrimination is prospective metadata requiring actual audio; no listening interaction is implemented merely by adding its skill record. Definitions can expose their associations through `ExerciseDefinitionStore::skills`.

`exercise_category` groups definitions and teaching guidelines by pedagogical purpose, independently of topics and skills. Its Rust names are `ExerciseCategory`, `ExerciseCategoryStore`, and `SeedResult::exercise_category_store`. The existing `category` fields in definition and guideline records retain their shape and reference the renamed table.

## Language features

Migration 003 supplies the feature catalogue, allowed badge values, and initial language assignments. Schema remains in migration 001. Typed read stores expose definitions, values, assignments, and joined badge data through `SeedResult`. See [language features](language-features.md) for field meanings, seed coverage, and sources. This metadata is not yet wired into UI rendering or generator requests.

## Target-specific concept seeds

Each of the 150 experimental catalogue targets has a separate `NNN_data_concept_<target-id>.sql` migration, in catalogue order from 100 through 249. `100_data_concept_pt-BR.sql` is populated; the remaining 149 files are intentionally empty placeholders and do not imply learning-content coverage. `migration/registry.rs` lists every file explicitly. Empty files skip SQL parsing/execution but still receive their own version, name, and checksum in the migration ledger. Filling a placeholder changes its checksum, so recreate this disposable database after seed edits.

The current, superseded pt-BR seed contains 100 concepts: 40 word-recognition concepts across animals, food/drink, everyday objects, family, and clothing; 40 separate recall concepts for the same senses; 12 grammar concepts; and eight sentence comprehension/production concepts. Descriptions preserve accents, clarify polysemous words, identify the topic, and constrain assessment. Grammar guidance covers gender, articles, agreement, regular plurals, subject pronouns, selected present-tense constructions, possession, negation, and simple word order. Sentence concepts distinguish understanding a proposition from producing it.

Database defaults allocate UUIDv7 IDs. Stable target-scoped codes identify each seed record; language associations and 74 concept prerequisite links resolve those generated IDs. Recognition and recall keep separate identities, and recall depends on recognition of the same sense. Grammar and sentence concepts reference relevant simpler concepts. Prerequisites are pedagogical relationships, not automatic stage advancement or evidence of proficiency. No learner evidence, exercises, answers, or mastery values are seeded.

The regional pronoun guidance preserves Brazilian variation rather than claiming that tu is absent; see the University of Texas at Austin's [Brazilpod usage notes](https://cob.coerll.utexas.edu/brazilpod/cob/wp-content/uploads/cob_32.pdf). The seed is a starter selection whose linguistic content remains subject to review, not an exhaustive regional curriculum. Grammar and sentence concepts in the database do not override the introductory stage's matching-only exercise restriction.

The registry, populated seed, and placeholders were prepared without building, running unit tests, or opening the active database. Migration execution and live generation using seeded IDs remain pending.

## Learner simulation and evaluation

`ExerciseSimulator` takes the exercise, its definition, source and target IDs, and recorded learner evidence. `MistakeDistribution` supplies a default percentage and exercise-kind overrides; rates must be integers from 0 through 100. The harness currently configures 25% by default, 30% for `match_words`, and 35% for `transliterate`. These are adjustable experimental settings, not estimates of a real learner's accuracy. For example:

```rust
let distribution = MistakeDistribution::new(25)?
    .with_rate("match_words", 30)?
    .with_rate("transliterate", 35)?;
let simulator = ExerciseSimulator::new(model, distribution);
```

Application-side sampling uses operating-system randomness through `getrandom`. For ordinary graded exercises, the percentage is the probability of an incorrect whole answer. For matching, it is the expected percentage of incorrect associations. Small matching sets cannot realize every percentage exactly, and a complete one-to-one matching cannot have exactly one incorrect pair. The sampler rounds stochastically and replaces a one-mistake count with either zero or two, preserving the expected rate. Thus 0% requests all-correct answers and 100% requests all-incorrect answers; intermediate rates are distributions over repeated exercises, not quotas within one session.

The simulator receives the sampled plan and reference content so it can generate plausible controlled errors. Learner evidence informs familiarity and mistake style; missing evidence does not establish mastery. The prompt requests one answer satisfying the definition's answer schema. Application validation requires complete one-to-one matching, existing choice IDs, valid nonrepeated token IDs, and nonblank typed answers. Objective answers must realize the sampled plan. For open answers, the independently generated evaluator verdict is checked against the plan before completion. A mismatch returns an error rather than silently changing the sample or saving a different distribution. There are no automatic retries or repairs.

`ExerciseEvaluator` receives the exercise, definition, submitted answer, exact target, referenced concepts, stage evaluation instructions, and teaching guidelines. It does not receive the simulator's sampled plan. Graded responses must satisfy the verdict schema, contain exactly one result per exercise concept, and provide nonblank English feedback. Matching results are checked per pair. Choice and token results must agree with the explicit answer key, and transliteration must agree with the declared accepted representations and case policy. Typed open answers are judged by the model, allowing valid equivalents; their linguistic accuracy still requires review.

The application assigns evidence modes: recognition for matching and vocabulary choices, recall for word/spelling/transliteration responses, comprehension for reading questions and target-to-English translation, and production for grammar and target-language construction. IDs, ownership, timestamps, event identity, evaluator identity, and policy version are application-owned. Evidence is explicitly tagged `synthetic` and retains the simulation plan. It does not calculate mastery. New exercise kinds require an evidence policy before they can complete.

Dialogue simulation treats all supplied turns as encountered and returns no graded answer. The evaluator records completion and the referenced concepts as exposure locally, without a model call or correctness flags. The harness commits verdicts, synthetic evidence, snapshots, and session progress through `complete_with_evidence`, then reloads learner evidence for the next request. This is a separate file-backed experiment; it does not connect simulation or grading to frontend sessions.

## Accepted concept and practice-focus design

This design is agreed but not yet implemented in the schema, stores, seeds, generation request, or UI. The current word-specific pt-BR seed must be replaced.

Topics describe subjects such as food and drink, numbers, animals, lifestyle, clothing, politics, location, health and medicine, law, buildings, and construction. Skills describe what the learner does. A target-specific concept combines a subject and an ability: recognizing common food vocabulary, recalling animal vocabulary, or understanding questions about feeling unwell. Add an explicit topic relationship to concepts rather than encoding the topic only in descriptive text. Concrete lexemes and sentences are generated exercise content. Broad concept evidence cannot establish knowledge of each individual word.

Represent practice selection with a typed `PracticeFocus` in the generation request. Topic, part of speech, content unit, and exercise type are independent filters: a learner can request everyday verbs, animal adjectives, or health-related sentences. The runner should intersect these filters with target availability and teaching-stage constraints before generation; the prompt then carries the selected focus. A conflict with the stage or available exercise definitions needs an explicit outcome rather than silently ignoring the selection. UI wiring remains separate future work.

## Runner and file logging

Generation diagnostics record the initial user prompt and system instructions, each model turn's raw text before validation, and tool arguments and results at DEBUG level. Forced-final INFO events identify `turn_limit` or `tool_call_limit`, the current turn, calls used, and both configured limits. The forced-final instruction says no further tools are available without claiming that all permitted calls were consumed. The conversation allows at most twelve tool calls across twelve tool rounds, followed by a thirteenth final turn. Reaching the call cap can force the final response earlier.

Duplicate payload values report `payload_validation`, a rule name, both JSON paths, normalized values, and the complete conflicting items. Paths use the generation response's `$.data` wrapper (for example, `$.data.pairs[1].concept_id`); when validating a persisted standalone payload, omit that wrapper to locate the field. The returned error retains both paths and the rule so the runner's final error remains actionable. Existing log files are unchanged; these details appear in subsequent runs.

`Runner` owns the stores and AI adapters and coordinates context loading, exercise selection, simulation, evaluation, and transactional completion. `main.rs` constructs `RunnerConfig`, initializes logging, opens the runner, and invokes it. Model, learner, source/target, stage, database path, log directory, and mistake rates are configured in `RunnerConfig::local`. Default paths resolve relative to the repository root, independently of the working directory.

The runner selects an existing unfinished exercise before generating another. Its payload references must match its persisted concept links; incomplete links return an error. A saved answer is preserved and evaluated rather than simulated again. Its original sampled mistake plan is unavailable after a restart because that plan is persisted only with completed evidence, so resumed saved answers record `resumed_saved_answer: true` and a null plan instead of inventing or resampling one. Fresh simulated answers retain the usual plan validation.

The harness uses `tracing` macros and `tracing-subscriber` for diagnostics. File and console use the readable event formatter. Each execution creates `logs/run-<UTC timestamp>-<run UUID>.log` containing readable text events. Headers include UTC time, severity, and message, followed by module, span context, and named fields. Prompts and returned text use indented multiline blocks instead of escaped JSON strings. JSON objects render as named nested fields and arrays as indexed items; embedded JSON and multiline string content are expanded recursively, including conversation message content. Debug-formatted JSON strings are decoded rather than shown with escaped newlines. This is a readable representation of structured payloads, not a JSON Lines log format. Existing log files are not rewritten. File logging includes application DEBUG events; console logging includes application INFO events. Both include dependency warnings and errors. The subscriber writes directly to a synchronized file writer, so events are readable while the process runs without an asynchronous logging queue. Logging initialization errors stop startup. See the [subscriber file-writer documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/writer/trait.MakeWriter.html).

Run/session spans and AI stage/call spans provide correlation IDs. At DEBUG level, the shared AI-call helper records system instructions and the serialized user prompt before starting a request, followed by complete returned model text before parsing or validation. Generator conversations also record tool definitions, turn numbers, requested calls, arguments, results, and conversation additions rather than repeatedly dumping the entire conversation. INFO events record request starts, durations, response sizes, database opening, exercise selection, completion, and session progress. Tool errors produce WARN events; transport failures, adapter validation failures, and runner errors produce ERROR events. Malformed completed responses remain in the log even when rejected. Formatting JSON for readability can change whitespace but preserves values; malformed text and system instructions retain their content with display indentation. Incomplete streamed text is not exposed by the current buffered client methods.

## Experimental stores

Typed repositories now cover languages, users, target statistics, sessions, exercise definitions, exercises, turns, concepts, concept associations, learner snapshots, skills, topics, stages, and teaching guidelines. Stage and skill prerequisites reject cyclic writes; concept prerequisites also reject mixed targets. Concept insertion creates its language association in one transaction.

All stores are exposed through `seed::SeedResult`. Migrations load the language catalogue and exercise availability for every target; seeding creates only the experimental learner state and session. Generation uses the intersection of target availability and stage-specific definition reads; automatic advancement is still undecided.

Reads use parameterized SQL and a shared DuckDB `to_json` decoder. SQL lives in private `queries` modules at the bottom of store files; static statements use named constants and shared dynamic templates use functions. Repository methods have separated preparation, execution, and validation steps. The multiline SQL layout follows [coding standards](../CODING_STANDARDS.md#sql-and-repositories). UTC timestamps are serialized explicitly, nullable JSON remains nullable, and invalid records return errors instead of panicking. File import reads and parses JSON before executing parameterized inserts transactionally. Updates report missing records. Teaching-stage order is immutable through the update API because changing its indexed order can violate DuckDB foreign-key update limitations; instructions and names remain editable.

`ExerciseStore::complete_with_evidence` validates exercise ownership and linked concepts, commits evidence, refreshes learner snapshots, saves the outcome, and derives session progress in one transaction. Invalid evidence or a full session rolls back the entire operation. Completed outcomes are immutable; repeating the same answer/verdict without an evidence batch succeeds without changing counts. Replaying a completed evidence batch returns an error rather than inserting duplicate events. Evidence can also be appended through its store for individual encounters and practice; use the transactional completion API for completed outcomes.

Exposure and practice remain ungraded, assisted answers do not count as independent assessed success, and aggregate snapshots derive counters without calculating mastery. Ungraded completion does not require an answer. Shared exercise-contract helpers validate payloads, answers, and references across generation, simulation, and evaluation.

## Validation

The `new_concepts` envelope, prompt examples, aggregated concept-resolution diagnostics, and thirteen-turn conversation limit pass `cargo check -p lingua-cli --offline`, targeted Rust formatting, and diff whitespace checks. No unit tests or live-model calls were performed; the database was not opened or recreated.

Generator exchange logging, duplicate-path diagnostics, and forced-final limit reporting pass `cargo check -p lingua-cli --offline` and targeted Rust formatting checks. No unit tests or live-model runs were performed, and the existing database was not opened or recreated. Compilation does not verify captured log output from a live generation.

The revised initial/category migrations, additional topic/skill migrations, category names, and store query organization have been source-reviewed and formatted without compiling, running unit tests, or opening the active database. Static SQL extraction preserved SQL tokens apart from the requested category-table rename and explicitly reviewed aliases. Migration execution and compilation remain pending until the active runner finishes.

The tool conversation and readable formatter have been source-reviewed and formatted only. No build, unit tests, model calls, or database smoke checks were performed for these changes because an existing runner is active. Compilation, formatter execution, and live tool-call compatibility remain pending. The earlier JSON logging implementation passed a direct smoke check and compilation; those results do not validate the new formatter or tool loop.

A direct persistence smoke check created a temporary DuckDB file, closed it, reopened it through `seed::open`, and confirmed identical learner/session IDs with exactly one learner and session retained. Requesting another learner in that existing file returned an error without reseeding. The temporary file and diagnostic example were removed. Persistent startup passes `cargo check -p lingua-cli --offline` and targeted Rust formatting checks. No unit tests were added or run.

Simulator/evaluator integration passes `cargo check -p lingua-cli --offline` and targeted Rust formatting checks. No unit tests were added or run. The live-model limitations described below still apply; compilation does not verify a complete generated/simulated/graded session or the quality of model answers.

The original seven migrations loaded transactionally in an in-memory DuckDB. Direct aggregate queries found 100 mapped targets, 3,297 exercise associations, all 35 exercise kinds represented, no unmapped targets or missing script context, and no English-target matching, translation, or bilingual dialogue associations. The availability changes pass `cargo check -p lingua-cli --offline` and targeted Rust formatting checks. No unit tests were added or run for this work.

The generator compiles with `cargo check -p lingua-cli --offline`, and its changed Rust files pass targeted formatting checks. A live run against the loaded `meta/muse-glimmer` model reached the 180-second streaming timeout without a complete response, so successful generation and persistence have not yet been verified end to end. No unit tests were added or run for the generator work.

From the repository root:

```powershell
python scripts/check_learning_schema.py path/to/duckdb.dll
```

The checker applies the migrations transactionally in memory and checks stage and exercise counts, the pt-BR starter concept seed, matching-only introductory interaction, evidence aggregation, duplicate protection, and ungraded constraints. It does not validate the live-model session loop.

Run native repository tests from the repository root:

```powershell
cargo test -p lingua-cli -- --test-threads=1
```

These cover native adapter round trips, corrupted-record errors, target isolation, nullable evidence, dependency cycles, quoted file paths, atomic completion rollback, duplicate protection, and derived session counts.

The expansion to 150 experimental targets was checked for unique IDs, complete exercise profiles, valid script references, and registered empty concept files using source inspection. No build, unit test, SQL execution, or database reset was performed for this expansion. Earlier runtime checks above describe the original smaller seed.

Database-mode changes and nested log rendering were inspected and formatted without a build, unit test, model call, or database access. `logs/readability-preview.log` is an illustrative rendering of two historical events, not a captured run of the new Rust formatter. Live verification remains pending.
