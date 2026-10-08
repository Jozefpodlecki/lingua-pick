# Coding standards

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
