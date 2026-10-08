# Coding standards

This is the authoritative contributor guide. It includes the conventions agreed during the experimental harness work. Accepted model changes are identified below; their implementation status belongs in [tasks](docs/tasks.md).

## Scope and structure

- Build on the existing Rust/Yew application and dependencies. Check the manifests before adding a library, and explain why any new dependency is needed.
- Keep changes focused on the requested work. Preserve unrelated user changes and avoid incidental repository-wide formatting or refactoring.
- Keep the frontend in `web/`. Put reusable language data and behavior in `crates/`, with a separate crate for each learning option, including regional varieties.
- Share common types and behavior when useful rather than duplicating them across language crates. Keep shared learning code independent of Yew and browser APIs where practical.
- Put app-shell primitives that do not depend on routes or app state in `lingua-web-core`, and exercise presentation primitives in `lingua-web-exercise`. Keep page composition, routing, and provider orchestration in `web/`. Put runtime detection and Tauri invocation in `lingua-api` rather than coupling components to the bridge.
- English is the source language. Treat each regional variety as a distinct target option; never identify an option only by its display name or image.
- Do not pre-create hundreds of empty crates. Add language crates as their content or functionality is implemented.

## Rust

- Use `rustfmt` formatting. Use `snake_case` for new module files and functions, `PascalCase` for types, and `SCREAMING_SNAKE_CASE` for constants.
- Prefer explicit domain types and enums over loosely structured strings or booleans when they prevent invalid states.
- Return `Result` for recoverable errors and use `Option` for missing values. Avoid `unwrap()` and `expect()` on browser storage, user input, or other operations that can fail during normal use.
- Follow the existing `no_std`/`alloc` setup where applicable. Do not add nightly features or unsafe code without a concrete need and an explanation.
- Keep public APIs small. Prefer clear names and structure over code comments, including documentation comments. Do not add comments by default; reserve them for essential constraints that cannot be expressed clearly in code. Put broader explanations in `docs/`.
- Avoid compressed code. Separate logical steps with blank lines, especially validation, state preparation, callback setup, and the final return or render. Use readable multiline blocks rather than packing conditions or method bodies onto one line. Keep related lines together and follow `rustfmt`.

## SQL and repositories

- Keep repository methods focused on parameter preparation, query execution, and domain validation. Put static SQL in a private `mod queries` at the bottom of each store file, using named `pub(super)` constants. Keep dynamic SQL construction in named functions in the same module.
- Separate Rust methods and logical steps with blank lines. Put constructors first, use explicit model imports, and avoid compressing query calls and parameter lists into one long line.
- Write SQL as multiline raw strings. Put `SELECT` on its own line with one selected expression per line. Put `FROM` and each `JOIN` on separate lines; indent join predicates under the join. Put `WHERE` on its own line and separate conditions across lines. Expand subqueries using the same layout.
- For `INSERT`, put the table name, parenthesized column list, and `VALUES` or `SELECT` on separate lines. List columns and value parameters vertically. Use explicit insert columns and selected expressions when record decoding depends on their shape; aliases should clarify the source of fields.
- Parameterize values. Dynamic identifiers may come only from application-owned repository definitions. Keep transaction boundaries and ownership checks intact when moving SQL.
- Desktop store migrations are immutable once applied. The experimental `lingua-cli` harness uses a disposable database: keep schema in 001 and edit its migrations directly when needed, then recreate the database after the runner stops. Existing files still undergo checksum validation; source edits do not upgrade them automatically.

Use this layout for queries:

```sql
INSERT INTO exercise_concept
(
    exercise_id,
    concept_id
)
VALUES
(
    $1,
    $2
);

SELECT
    con.id,
    con.code,
    sk.code AS skill_code
FROM concept con
JOIN skill sk
    ON sk.id = con.skill_id
WHERE
    con.language_id = $1
    AND sk.code = $2;
```

## Experimental harness structure

- Treat `lingua-cli` as a development harness while the interfaces are being explored. Do not introduce command-line product behavior unless requested. Move reusable behavior into other crates when its boundary is established.
- Keep `main.rs` limited to configuration, logging initialization, runner construction, and invocation. Put session orchestration in `Runner`; keep persistence in stores and model calls in adapters.
- Give generator, simulator, and evaluator their own `prompt.rs` and `error.rs`. Keep `generate` and other entry points readable by extracting cohesive helpers for context, prompt construction, model calls, decoding, and validation.
- Split seeding responsibilities under `seed/`. Use an explicit database mode for open-or-create, existing-only, recreation, or in-memory storage. Seed fresh databases transactionally. For an existing database, validate migration checksums and resolve saved state; do not silently reseed, repair, or upgrade it. Keep all pooled in-memory connections attached to the same database instance. Recreate only the configured disposable file and its WAL, after the runner stops.
- Keep ownership checks and state transitions in application code. Save exercise completion, evidence, and derived progress atomically. Resume pending work without replacing a persisted learner answer.

## Pedagogical data and practice selection

These are accepted modeling rules. The topic relationship and structured practice filters still require implementation; see [experimental learning data](docs/experimental-learning-data.md#accepted-concept-and-practice-focus-design).

- Seed pedagogical metadata that guides generation. Do not attempt to encode a dictionary or every lexeme in concept migrations.
- Keep distinct meanings for topic, skill, concept, and exercise category. A topic is a subject such as `food_and_drink`, animals, clothing, politics, health, law, or construction. A skill is an ability such as recognition, recall, comprehension, or production. A concept describes a target-specific learning objective within that subject and skill.
- Prefer concepts such as recognizing common food vocabulary or understanding questions about feeling unwell. Let the generator choose concrete words and sentences. Success on a broad concept does not prove knowledge of every word in that topic.
- Use `exercise_category` for grouping exercise definitions and teaching guidelines. Keep subject topics separate from exercise categories.
- Keep part of speech, content unit, topic, and exercise type independent. A request for verbs must not be interpreted as a topic or a requirement to produce sentences. Represent the selection in typed `PracticeFocus` data and filter compatible definitions before asking the model to generate.
- Keep schema in experimental migration `001_initial.sql`. Put reference data in separate numbered migrations, ordered after their dependencies. Reserve migrations below 100 for shared metadata and 100 onward for per-target concepts. Use dedicated topic, skill, and per-target concept seed files; an empty target file does not imply course coverage.
- Prefer DuckDB utility functions such as `uuidv7()` for database-generated IDs. Allocate application-owned IDs where needed, including new-concept slots offered to the generator. Models may only reference supplied IDs or explicitly supplied new-ID slots.
- Keep script metadata separate from language identity. Supply the relevant writing system and accepted answer representation for transliteration, including character-to-Latin exercises. Do not assume every script maps a character to one Latin letter.
- Unknown learner knowledge remains unknown. Begin the introductory vocabulary stage with word matching; later grammar and sentence progression needs an explicit evidence-based policy. Keep English-target availability consistent with omitting same-language matching and translation.
- Separate recognition, recall, comprehension, production, and exposure evidence. Ungraded dialogue records exposure without correctness or mastery. Seed membership and generated content are not evidence of learner knowledge.

## AI calls and tools

- Request exactly one final exercise. System instructions must explain the response contract, available tools, and the distinction between intermediate tool calls and the final exercise response.
- Validate typed arguments, JSON contracts, references, ownership, and objective answer invariants in application code. Treat model output as untrusted input; return clear errors rather than silently repairing it.
- Keep store tools read-only and scoped to the runner's learner, session, and target. Do not let the model override identity, write progress, or advance a session.
- Bound tool turns, calls, result sizes, and history. Preserve call IDs and conversation messages when returning tool results. Validate concepts against successfully retrieved context and check new concept codes against stored data.
- Configure generous explicit timeouts for the local model. Bound the complete conversation as well as first-chunk and idle waits; keep these settings in the shared transport layer.
- Keep simulation and evaluation separate. Configure mistake percentages by exercise type and sample the plan in application code. The evaluator must not receive the simulator's intended verdict. Mark simulated learner evidence as synthetic.

## Logging

- Use `tracing` and `tracing-subscriber` with `info!`, `debug!`, `warn!`, and `error!`. A custom formatter may improve readability; use the logging crate for event handling.
- Create a separate timestamped log file for each run. Include run, session, exercise, and model-call correlation where available, plus duration and error context.
- Log exact prompts and complete raw responses before parsing at DEBUG level. Log tool names, arguments, results, and turn context so generation decisions can be followed.
- Render prompts and responses as readable multiline blocks. Expand structured JSON as nested named fields and indexed arrays, including embedded prompt strings; use the same readable event format for file and console. Avoid escaped JSON strings containing entire conversations and repeated full history on every turn.
- Keep console output concise, and retain detailed diagnostics in the file. Do not log credentials or provider authentication material.

## Yew and browser behavior

- Keep pages responsible for composing the screen. Put reusable UI in components with typed properties, and keep learning logic separate from rendering.
- Keep `html!` focused on declarative markup. Do not put multi-statement iterator closures, variable setup, context or navigator cloning, callback construction, or navigation logic inside it. Prepare data and callbacks before the markup. Rust/Yew can support these inline expressions, but this project prohibits that pattern.
- Review repeated markup and UI responsibilities for extraction into components. For example, a language results grid should render a dedicated `LanguageCard` component rather than define each card's markup and selection handler inside an inline `.map()` closure. Keep each component responsible for a coherent piece of UI; do not extract trivial wrappers without a useful purpose.
- Use `data-*` attributes to expose UI state and stable identifiers, such as `data-selected` and `data-language-id`. Prefer styling state through data-attribute variants over embedding conditional class or markup logic throughout the render tree. Keep typed Rust state as the source of truth, and retain semantic HTML and appropriate ARIA attributes; data attributes do not replace accessibility semantics.
- Reuse the existing layout, routing, and storage utilities where appropriate. Improve their error handling when a feature requires it.
- Support static hosting on GitHub Pages. Keep hash routing unless a documented replacement handles static-host navigation correctly.
- Persist selected language and progress in browser storage. Local storage is required: return an initialization error if access fails or it is absent; do not silently fall back to memory. Handle missing or invalid saved data separately. Plan backward compatibility or migration when changing saved data formats.
- Use semantic HTML, accessible labels, visible keyboard focus, and keyboard-operable controls. An autocomplete must support keyboard selection and communicate its results to assistive technology.
- Keep visible UI copy minimal. Avoid instructions that repeat a heading or obvious action, and do not add search help or result counters without a user need. Preserve accessible names when removing visible labels.
- Keep layouts usable on small and large screens. Use the existing Tailwind styling setup and avoid hand-editing generated CSS or Trunk build output.
- Use region- or culture-relevant imagery for language cards rather than flags. Keep the language and variety labels readable independently of the image.

## Validation

Follow explicit session constraints before choosing checks. For the current experimental work, the user has prohibited adding or running unit tests and has prohibited builds while their runner is active. Use documentation, source, formatting, and diff checks where appropriate; wait for those restrictions to be lifted before executing prohibited checks. Do not open, reset, or delete the active runner database.

The commands and testing guidance below apply when those checks are authorized.

Run commands from `web/` to use its pinned toolchain and WebAssembly target configuration:

```powershell
cargo fmt --all -- --check
cargo check --target wasm32-unknown-unknown
trunk build
```

- Use `cargo fmt --all` when formatting is needed, but review the diff and keep unrelated formatting out of the change.
- For frontend changes, use `trunk build` to validate the complete asset pipeline. Use `trunk serve` for local interaction checks; the configured address is `http://localhost:1420`.
- For changes to shared learning logic, add focused tests for meaningful behavior and failure cases. Run those tests on a native target when the crate is browser-independent; the frontend's WebAssembly target cannot run ordinary native tests.
- Write or update unit tests last, after the implementation and structure are settled. Keep inline test modules at the bottom of their source file. This ordering does not waive required validation before completing a change.
- Do not add tests that simply mirror the implementation or require new infrastructure for a trivial reversible change.
- Report which checks ran and any failures or limitations. Existing warnings are not evidence that a change failed, but do not introduce new warnings unnecessarily.

The commands above are contributor guidance; the current CI workflow does not enforce them. Do not claim CI coverage that has not been implemented.
