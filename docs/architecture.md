# Application architecture

## System boundaries

The current deployment contains a static Yew/WebAssembly frontend and shared `no_std` learning logic. There is no backend, account service, implemented AI client, or implemented learning UI.

| Layer | Owns | Must not own |
| --- | --- | --- |
| `assets/` | Catalogue and future schemas or static media | Secrets or executable UI from model output |
| `crates/lingua-core/` | Shared types, validation, answer grading, session logic | Browser APIs, HTTP, storage, or UI |
| Future target crates | Vocabulary, curriculum, script rules, supported formats for one learning option | Global browser state |
| Web services | Browser persistence and future generation transport | Duplicate core grading rules |
| Yew pages/components | Navigation, presentation, drafts, accessibility, feedback | Authoritative syllabus or model-controlled layout |

Keep generated content separate from learner progress. A future generation response is a candidate exercise until validated; it does not become trusted because it has valid JSON. Bind every asynchronous request to its target and request identity.

For planned learning behavior, use [UI specifications](ui.md), [exercise specifications](exercises/README.md), and [target coverage](exercise-coverage.md). Generation transport and prompt contracts live in [LM Studio integration](llm-integration.md) and [AI interaction](ai-interaction.md).

## Language selection and switching

The app is a static Rust/Yew frontend. English is always the source language. The selected target is shared across pages and remains visible in the top bar.

Implemented flow:

1. The home route (`#/`) shows a combobox and up to four card-style suggestions from 100 target options, without an internal scrollbar. Search matches names, native names, regions, identifiers, and optional aliases. Common Latin accents are folded, query words can span fields, and exact/prefix matches rank first. The input keeps keyboard focus: Up/Down moves through suggestions, Enter chooses the active option (or the first result), Escape closes suggestions, and typing or refocusing reopens them. Clicking a card also chooses it. The search input has an accessible name without a redundant visible label or instruction repeating the heading.
2. Selecting a card updates shared state, attempts to save the selection, and opens `#/learn`.
3. The learning route currently displays the selected language and a placeholder for future lessons.
4. The top bar displays the selection on every page. Its **Change** link returns to the picker without clearing the existing choice. Selecting another card replaces the active target.
5. Reloading restores a valid saved selection. The home screen remains the picker, with the saved card marked as selected.

The catalogue contains 100 learning options, including regional varieties such as Brazilian and European Portuguese. It is not the complete 400-option catalogue and does not contain lesson content. Regions are discovery labels, not claims that a language is exclusive to that region. English remains the source internally; the UI does not repeat that information.

## Responsibilities

| Location | Responsibility |
| --- | --- |
| `web/src/app.rs` | Browser context, initialization of selected language, persistence callback, and context providers |
| `web/src/state/mod.rs` | `LearningContext`: active target, selection callback, and storage warning |
| `assets/languages.json` | Shared catalogue identifiers and display/search metadata |
| `web/src/state/language.rs` | Catalogue types, validation, and search; reexports the core's `LanguageId` |
| `crates/lingua-core/` | Shared target identifiers, exercises, and session logic; see [learning core](learning-core.md) |
| `web/src/storage.rs` | JSON serialization and browser storage access |
| `web/src/components/top_bar.rs` | Current target and link back to the picker |
| `web/src/components/Layout.rs` | Shared top bar, storage warning, and page container |
| `web/src/pages/home.rs` | Composes the layout and language picker |
| `web/src/components/language_picker.rs` | Search, filtered results, and selection/navigation callback |
| `web/src/components/language_card.rs` | One target card, with selection state exposed through data attributes |
| `web/src/pages/learn.rs` | Learning destination; handles visiting without a selected target |
| `web/src/routes.rs` | Hash routes compatible with GitHub Pages |

Keep selection state above the router so navigating between pages does not discard it. Pages and components consume `LearningContext`; they should use its selection callback instead of maintaining separate target choices or writing storage directly.

## Catalogue loading

The catalogue lives in root-level `assets/languages.json`, outside the frontend. Each entry requires `id`, `name`, `region`, and `native_name`, and may include an `aliases` array for alternate search terms. Add learning options here without adding enum variants or hard-coded name matches in Rust. Identifiers are stable strings; keep existing identifiers unchanged to preserve saved selections.

`include_str!` embeds the JSON during compilation. `AppContext::new` parses and validates it once at startup, returning an initialization error for malformed JSON, an empty catalogue, blank fields, identifiers with surrounding whitespace, or duplicate identifiers. This requires rebuilding to publish catalogue changes and avoids a separate network request or deployment-path dependency. `LearningContext` shares the parsed catalogue through `Rc`.

Cards are listbox options. They use `data-language-id` for identity, `data-selected` for the saved target, and `data-active` for the keyboard suggestion. `aria-selected` and the input's `aria-activedescendant` communicate the active suggestion. The picker uses `data-empty` and `data-open` for presentation. Callbacks and result construction are prepared before the main `html!` markup.

## Saved selection

The browser storage key is `lingua-pick.selected-language.v1`. Its JSON value is a stable target identifier, currently `"pt-BR"` or `"pt-PT"`, rather than a display label.

- Missing or malformed data, or an identifier absent from the current catalogue, yields no selection.
- Browser local storage is required at startup. `AppContext::new` returns `StorageAccess` if accessing it throws, or `NoStorage` if it is absent. The app does not start with an in-memory fallback.
- If saving fails, the current selection still changes and a warning explains that reloading may reset it.
- Storage is local to the browser and origin. Local hosting and GitHub Pages do not share saved data.

Progress persistence is not implemented yet. When added, progress must belong to a target identifier so switching languages does not overwrite another target's progress. Changes to saved formats need versioning or migration.

## Planned extensions

The proposed learning flow is: choose target → choose word categories and exercise format → obtain authored or generated content → validate it → construct a session → collect an answer → show feedback → continue → show summary. Only target selection and core single-choice/session logic are implemented.

Future progress records must be keyed by target and use versioned storage. Session serialization, resuming unfinished sessions, grading translation alternatives, and the category-selection UI remain open decisions. Local storage is required at startup; individual write failures are surfaced separately.

The proposed learning flow is: choose target → choose word categories and exercise format → obtain authored or generated content → validate it → construct a session → collect an answer → show feedback → continue → show summary. Only target selection and core single-choice/session logic are implemented.

Future progress records must be keyed by target and use versioned storage. Session serialization, resuming unfinished sessions, grading translation alternatives, and the category-selection UI remain open decisions. Local storage is required at startup; individual write failures are surfaced separately.

- Each learning option, including a regional variety, gets its own crate under `crates/` when learning content is implemented. The JSON catalogue provides picker metadata, not a substitute for those crates.
- Expand the picker toward 400 options and add region/culture imagery. Current cards contain text only; no external image assets are used.
- Add lessons and progress per language. Selecting a language must not imply that its course is already available.
