# Runtime environments

## Implemented environments

| Runtime | Detection | Exercise source |
| --- | --- | --- |
| Tauri desktop | Injected `window.__TAURI__.core.isTauri()` returns true | Native LM Studio prompt command; exercise generation is not connected yet |
| Local web | No Tauri bridge; loopback or localhost hostname | Target-specific sample JSON |
| GitHub Pages | No Tauri bridge; hostname ends in `.github.io` | Target-specific sample JSON |
| Other hosted web | No Tauri bridge; other hostname | Target-specific sample JSON |

`lingua-api` detects the environment at startup, and the application stores it in `AppContext`. Tauri takes precedence over hostname, so desktop development using a localhost frontend is still desktop. Custom-domain GitHub Pages sites classify as hosted web and retain the same sample-only behavior. Hostname detection is not a security boundary.

The shared footer displays the resulting mode as Tauri desktop, local web, GitHub Pages, or hosted web. Its `data-runtime` attribute uses the stable machine-readable name already exposed by the layout.

Detection returns `RuntimeError`, which implements `Display` and `core::error::Error`. It distinguishes field inspection, missing runtime check, failed invocation, invalid check result, and hostname access errors. `AppContextError` preserves it as a typed source rather than converting it to a string.

The existing `withGlobalTauri: true` setting exposes the public API needed for detection. [Tauri core API](https://v2.tauri.app/reference/javascript/api/namespacecore/#istauri).

No browser environment calls LM Studio. Being detected as Tauri does not establish that an LLM is running or that a model is loaded. The desktop runtime registers a `send_prompt` command, and the `lingua-api` adapter invokes it through `window.__TAURI__.core.invoke`. The exercise loader is not connected to that command yet, so desktop exercise generation still reports an explicit not-connected error.

The command accepts the shared `PromptRequest` contract, validates prompt limits, sends a non-streaming request to `http://127.0.0.1:1234/v1/chat/completions`, and returns `PromptResponse`. The managed native HTTP client has a 120-second total timeout, rejects redirects, and limits responses to 2 MiB. Command failures use a serializable error code, message, and optional HTTP status. Endpoint settings, authentication, streaming, and model discovery are not implemented.

During Tauri setup, the native store opens `lingua-pick.duckdb` under the application data directory, validates its migration history, applies pending migrations, and becomes managed application state. Setup fails if the directory, database, or migrations cannot be initialized. No frontend storage command uses this state yet; see [native storage](storage.md).

## Desktop window

`crates/lingua-app/tauri.conf.json` declares `maximized: true` for the initial window. The dev URL is `http://localhost:1420/`; the packaged frontend comes from `../../web/dist`. Maximization uses the documented Tauri 2 window configuration. [Configuration reference](https://v2.tauri.app/reference/config/#windowconfig).

The Tauri crate is user-created and part of the workspace. Maximization is configured; verifying its appearance requires launching the desktop application.

## Sample exercises

Store fixtures in root-level `assets/samples/sample-<target-id>.json`. Use exact catalogue IDs, including variety and script subtags. Each file is a JSON array of core `Exercise` objects, not a model response envelope.

Initial fixtures:

- `sample-pt-BR.json`
- `sample-pt-PT.json`
- `sample-zh-Hans-CN.json`

These are small authored demonstrations, not full courses. Targets without fixtures show an unavailable-sample error rather than using another language's content.

Trunk copies the samples directory into `dist/samples` using the `copy-dir` link in `web/index.html`. The service requests `samples/sample-<target-id>.json` relative to the document's configured base URL. This preserves GitHub Pages repository subpaths such as `/lingua-pick/`.

`web/src/services/exercises.rs` validates safe identifiers before constructing paths, checks HTTP results, parses JSON, and constructs a core `Session`. Core validation rejects mismatched targets, duplicate exercise IDs, and invalid choice sets. The learning component ignores asynchronous responses after unmounting; target changes create a fresh keyed component.

`LearningSession` provides single-choice selection, explicit checking, feedback, continuation, and a score summary. Progress is in memory; saving session outcomes is still pending. Translation and matching samples cannot be accepted until their core variants and UI are implemented.

## Validation

From `web/`, build with Trunk and verify the output contains `samples/`. Test browser routing under a non-root public URL as well as local hosting. Native tests cover environment classification, sample parsing, and path safety; they do not replace desktop/browser interaction checks.
