# Learning core

`crates/lingua-core` is the shared learning library. It uses `#![no_std]` and `extern crate alloc`, with no Yew, browser, storage, filesystem, or clock dependencies. Serde is configured with its default features disabled and `alloc`/`derive` enabled. The host application provides the allocator.

## Model boundaries

| Model | Responsibility |
| --- | --- |
| `LanguageId` | Stable target identifier shared with the frontend catalogue and saved selection |
| `Text` | Text value, optional ISO 15924 script identifier, and optional reading/transliteration |
| `CurriculumLevel` | Framework, optional edition, and level label supplied by course content |
| `Exercise` | Identity, target language, instruction, prompt, optional curriculum level, and interaction content |
| `ExerciseContent` | Interaction format; initially `SingleChoice` |
| `Choice` | Stable answer identifier and display text |
| `Session` | Ten ordered exercises for one target, current exercise, submitted answers, and score |

The exercise classifier describes what the learner does. A single-choice exercise can show Chinese characters, Latin text, or other scripts without becoming a different interaction type. Script metadata belongs to text. Curriculum metadata, including an HSK level, belongs to the exercise. These axes are independent.

HSK labels are course metadata, not a fixed Rust enum or a built-in level validator. A framework edition can distinguish different syllabus versions. Sample level annotations in tests illustrate the model; they are not an authoritative vocabulary syllabus.

## Chinese example

A vocabulary recognition exercise can have:

- Target: `zh-Hans-CN`.
- Instruction: `Choose the meaning`.
- Prompt: `猫`, script `Hans`, optional reading `māo`.
- Curriculum metadata: framework `HSK`, course-provided edition and level.
- Interaction: `SingleChoice`, with three choices such as `cat`, `dog`, and `horse`.
- Correct answer: the stable `cat` choice identifier.

The frontend renders three answer cards for the sample fixture. The core checks the submitted identifier; it does not know about CSS, card layouts, or DOM events. A small Chinese sample exists; a full Chinese course is not implemented.

## Validation and session behavior

`SingleChoice::new` requires at least two nonblank choices with unique identifiers and an existing correct choice. Deserialization uses the same constructor, so saved or authored data cannot bypass these checks. Answering with an unknown identifier returns an error rather than counting as a wrong answer.

`Session::new` requires a nonblank target, exactly ten exercises, unique nonblank exercise identifiers, and matching target identifiers across all exercises. A valid answer is recorded and advances to the next exercise, whether correct or incorrect. Invalid answers leave the session unchanged. Answering after completion returns an error.

Session fields are private; use its methods to read progress and submit answers. Session persistence, spaced repetition, retries, and scheduling are not implemented. Exercise IDs and metadata remain content-author responsibilities; session validation does not validate the entire syllabus or every text field.

## Dependencies and checks

The frontend imports `LanguageId` from the core while retaining catalogue search and UI state in `web/`. Future language crates should depend on the core for common exercise models.

Run from the repository root, using the frontend's pinned toolchain:

```powershell
cargo +nightly-2026-10-01 test -p lingua-core
cargo +nightly-2026-10-01 check -p lingua-core --target wasm32-unknown-unknown
cargo +nightly-2026-10-01 tree -p lingua-core -e features
```

The package-specific dependency tree verifies the core's own features. Cargo feature unification may enable Serde's `std` feature when building the frontend alongside it; the core's standalone WebAssembly check verifies its supported `no_std` configuration.
