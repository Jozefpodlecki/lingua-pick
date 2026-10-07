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
- `assets/` contains shared data and assets, including the language catalogue in `assets/languages.json`.
- `crates/` is the intended location for language crates. Each learning option, including each regional variety, must have its own separate crate.
- `crates/lingua-core/` contains shared learning models and session logic, using `no_std` with `alloc`. Read [learning core](docs/learning-core.md) before changing these models.
- `docs/` is the intended location for detailed product requirements, architecture, and decisions. Keep agent instructions here concise and refer to relevant documents as they are added.

## Established product requirements

- Start with a home screen where the learner selects a target language.
- Use a polished autocomplete search with card-style results rather than a dropdown language selector.
- Use wallpaper-style imagery relevant to the language's region or culture rather than flags.
- Clearly distinguish regional varieties in the language picker.
- Persist the selected language and learning progress in browser storage.
- Show the selected target in a shared top bar and let the learner return to the picker to change it.

## Development guidance

- Before making changes, read [coding standards](docs/coding-standards.md) and [documentation standards](docs/documentation-standards.md). Use [the documentation index](docs/README.md) to find additional relevant guidance.
- Inspect the current repository structure and manifests before making changes.
- Consult [roadmap](docs/roadmap.md) and [tasks](docs/tasks.md) for work status and dependencies. Update relevant items when implementing them; documentation alone does not complete a feature.
- For selection, navigation, or persistence changes, read [application architecture](docs/architecture.md). Use the shared `LearningContext` for the active target.
- For UI changes, read [UI specification](docs/ui.md). For learning features, read [exercise specifications](docs/exercises/README.md) and [target coverage](docs/exercise-coverage.md). Implemented behavior and proposed features are explicitly distinguished in these documents.
- Use the existing Rust and Yew stack and check `web/Cargo.toml` for available libraries before introducing dependencies.
- Reuse existing routing and storage utilities where appropriate.
- Keep features compatible with local execution and static hosting on GitHub Pages.
- Treat product and architecture details not documented here or in `docs/` as undecided; do not present assumptions as established requirements.
