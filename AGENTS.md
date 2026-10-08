# Lingua Pick

## Project context

- A personal language-learning web app built with Rust and Yew.
- Run locally or deploy as a static site through GitHub Pages.
- English is always the source language.
- The long-term target is 400 learning options, including regional varieties. Each variety counts toward the 400.
- Brazilian Portuguese and European Portuguese are distinct learning options.

## Project structure

- The root `Cargo.toml` defines the Rust workspace.
- `web/` contains the Yew frontend, with pages, components, routing, state, and browser storage under `web/src/`.
- `web/styles/` contains the frontend styles.
- `crates/lingua-app/` is the user-added Tauri 2 desktop application. Its initial window is configured to open maximized.
- `assets/` contains shared data and assets, including the language catalogue in `assets/languages.json`.
- `crates/` is the intended location for language crates. Each learning option, including each regional variety, must have its own separate crate.
- `crates/lingua-core/` contains shared learning models and session logic, using `no_std` with `alloc`. Read [learning core](docs/learning-core.md) before changing these models.
- `crates/lingua-store/` contains native DuckDB persistence, transactional migrations, session history, and derived statistics. Read [native storage](docs/storage.md) before changing its schema or repositories.
- `crates/lingua-ai/` contains the provider-neutral exercise-generation request and facade. Transport implementations belong in runtime adapters, not in the core or frontend components.
- `crates/lingua-api/` contains browser runtime detection and the typed Tauri invocation bridge.
- `crates/lingua-web-core/` contains reusable application-shell components that do not depend on app routing or state.
- `crates/lingua-web-exercise/` contains reusable exercise presentation components. Keep route and exercise-provider orchestration in `web/` until those dependencies have stable interfaces.
- `docs/` is the intended location for detailed product requirements, architecture, and decisions. Keep agent instructions here concise and refer to relevant documents as they are added.

## Established product requirements

- Start with a home screen where the learner selects a target language.
- Use a polished autocomplete search with card-style results rather than a dropdown language selector.
- Use wallpaper-style imagery relevant to the language's region or culture rather than flags.
- Clearly distinguish regional varieties in the language picker.
- Persist the selected language and learning progress in browser storage.
- Show the selected target in a shared top bar outside an active session and let the learner return to the picker to change it.
- During an active session, hide the shared top bar and footer and use the top area for session progress.
- After selecting a target, open its dashboard with Run exercise, Analyze stats, and Exercise history actions.
- Learner knowledge starts unknown. Follow [progress requirements](docs/progress.md); ungraded dialogue completion records exposure, not mastery or correct-answer accuracy.

## Development guidance

- Before making changes, read [coding standards](docs/coding-standards.md), [documentation standards](docs/documentation-standards.md), and the [project glossary](docs/glossary.md). Use [the documentation index](docs/README.md) to find additional relevant guidance.
- Inspect the current repository structure and manifests before making changes.
- Consult [roadmap](docs/roadmap.md) and [tasks](docs/tasks.md) for work status and dependencies. Update relevant items when implementing them; documentation alone does not complete a feature.
- For selection, navigation, or persistence changes, read [application architecture](docs/architecture.md). Use the shared `LearningContext` for the active target.
- For platform capabilities or exercise loading, read [runtime environments](docs/runtime.md). Web-only runs use target-specific sample JSON; native generation belongs behind the Tauri bridge.
- For database or history changes, read [native storage](docs/storage.md). Never edit an applied migration; append a new version.
- For UI changes, read [UI specification](docs/ui.md). For learning features, read [exercise specifications](docs/exercises/README.md) and [target coverage](docs/exercise-coverage.md). Implemented behavior and proposed features are explicitly distinguished in these documents.
- Use the existing Rust and Yew stack and check `web/Cargo.toml` for available libraries before introducing dependencies.
- Reuse existing routing and storage utilities where appropriate.
- Keep features compatible with local execution and static hosting on GitHub Pages.
- Treat product and architecture details not documented here or in `docs/` as undecided; do not present assumptions as established requirements.
