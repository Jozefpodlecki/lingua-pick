# Project glossary

Use these terms consistently in product requirements, code APIs, UI copy, and decision records. A definition describes the project meaning of a term; it does not imply that the corresponding feature is implemented.

## Language and course terms

**Source language**
: The language the learner already uses to understand instructions, meanings, and translations. English is the fixed source language in Lingua Pick.

**Learning target**
: The exact language or regional variety the learner is studying. A target has a stable identifier such as `pt-BR`, independently stored progress, and its own content boundary. Use *target* as the short form after the meaning is clear.

**Learning option**
: One selectable entry in the language picker. Each option resolves to exactly one learning target. Regional varieties count as separate options.

**Regional variety**
: A region-specific form treated as an independent learning target, such as Brazilian Portuguese and European Portuguese. A variety is not merely a display label attached to a shared course.

**Target identifier**
: The stable machine-readable identity of a learning target. Display names, regions, aliases, and artwork may change without changing this identifier. Persisted selection, content, sessions, history, and progress refer to targets by this identifier.

**Language catalogue**
: The validated list of available learning options and their discovery metadata: target identifier, display name, native name, region, and search aliases. Catalogue membership does not guarantee that a complete course exists.

**Course**
: The target-specific body of vocabulary, learning items, curriculum rules, authored content, and supported exercise types. Each learning target, including each regional variety, has a separate course boundary.

**Target module**
: The Rust crate or equivalent content module that implements one course. It supplies target-specific knowledge while depending on shared domain models.

**Script**
: A writing system identified separately from the language target and exercise type, preferably with an ISO 15924 identifier such as `Latn` or `Hans`. A target may use more than one script.

**Curriculum level**
: A label from a named curriculum framework and optional edition, such as an HSK level. It is metadata supplied by a course and does not by itself prove that content follows that curriculum.

**Word category**
: A stable topic classification for vocabulary, such as `animals` or `food_and_drink`. A category constrains content selection; it is not an exercise type, script, or curriculum level.

**Learning item**
: A stable unit whose encounters and assessed results can contribute to learner progress, such as a vocabulary entry, phrase, grammar skill, or listening distinction. An exercise may cover multiple learning items. An exercise identifier is not a learning-item identifier.

## Learning interaction terms

**Exercise**
: One bounded learner interaction with a stable identifier, target, instruction, prompt, optional metadata, and interaction-specific content. Examples include selecting one meaning, translating one sentence, matching one set of words, or advancing one dialogue unit.

**Exercise type**
: The interaction the learner performs, such as `single_choice`, `translate_sentence`, `match_words`, or `dialogue`. Language, regional variety, script, word category, and curriculum level are independent of exercise type. *Exercise format* and *interaction format* refer to the same concept, but project APIs and new documents should prefer *exercise type*.

**Answer**
: The learner submission for a graded exercise. Its shape depends on the exercise type: a choice identifier, translated text, a word-bank sequence, or a completed mapping.

**Answer outcome**
: The validated result of one submitted answer, including the exercise identity and grading result. Invalid or malformed input is an error and does not produce an incorrect outcome or advance the session.

**Feedback**
: The state shown after a valid submission and before the next exercise. It may communicate correctness and a useful explanation. Feedback is presentation state; recording the answer is what advances the core session.

**Dialogue**
: An ungraded bilingual interaction that displays target-language text above its English translation. The learner may listen or practice speaking and advances with **Continue**. Completion records exposure or practice, not a correct answer.

**Language session (`Session`)**
: One ordered run of exercises for one exact learning target. The current core requires exactly ten exercises. A valid graded answer advances the session whether it is correct or incorrect; invalid input does not. Use *session* as the short form in code and prose.

**Session progress**
: The learner's position within the current language session, such as exercise 4 of 10. It is transient run state and is distinct from long-term learner progress.

**Activity**
: A broader unit recorded in learner history. A completed or incomplete language session can be an activity; an ungraded dialogue can also be an activity. Activity completion does not imply correctness or mastery.

## Progress terms

**Learner progress**
: Persisted, target-specific information derived from real learning events over time. It includes history and evidence; it is not the current position inside a session.

**Evidence**
: A recorded fact about a learner interaction that can inform progress, such as exposure, activity completion, a practice attempt, or an assessed answer. Evidence retains whether it was graded.

**Exposure**
: Evidence that the learner encountered a learning item. Viewing or continuing through content can establish exposure, but not comprehension or mastery.

**Practice**
: Evidence that the learner attempted an optional learning action, such as playing audio or recording speech. Practice is not assessed performance unless an explicit assessment produces a result.

**Assessment**
: An interaction with a defined grading policy that produces evidence about performance. Assessment results are limited to that policy and do not establish permanent mastery from one answer.

**Unknown knowledge**
: The initial state when the application has no evidence about whether the learner knows a learning item. Unknown is distinct from an incorrect answer, zero knowledge, or low mastery.

**Mastery**
: A future derived judgment that would require an agreed evidence model and thresholds. The project does not currently define or calculate mastery.

**History**
: The chronological record of actual activities and relevant events. History may contain graded sessions, ungraded dialogue, incomplete activities, and practice attempts when those events were recorded.

## System terms

**Runtime**
: The environment hosting the Yew application: local browser, GitHub Pages, another hosted browser, or Tauri desktop. Runtime capabilities determine which adapters are available.

**Capability**
: A service available in a runtime, such as static sample loading, AI generation, audio playback, or microphone access. Features should depend on declared capabilities rather than scattered environment checks.

**IPC bridge**
: The inter-process communication boundary between the Yew frontend in the system WebView and native Tauri code. The frontend serializes command arguments and receives a serialized result or error.

**Tauri command**
: A Rust function registered with Tauri's invoke handler and callable through the IPC bridge. `send_prompt` is the native command that communicates with LM Studio; it is not itself an exercise provider.

**Exercise provider**
: An adapter that supplies a complete exercise batch. Browser runtimes use packaged samples; the planned Tauri provider will obtain generated exercises from LM Studio.

**AI facade**
: The provider-neutral boundary in `lingua-ai`. It validates generation requests, delegates to an exercise provider, and constructs a core language session only from a valid complete batch.

**Generated exercise**
: An exercise candidate returned by a model-backed provider. It remains untrusted until request, schema, semantic, target, and session validation succeed.

**Authored exercise**
: Exercise content created and packaged by the project rather than generated at runtime. Authored content still passes the same applicable domain validation before entering a session.

**Sample session**
: A packaged authored language session used when AI generation is unavailable. A sample demonstrates supported behavior; it is not a complete course.

**Persistence adapter**
: The runtime-specific boundary that reads and writes selected-target, progress, or history records. Persisted formats use stable identifiers and explicit versions.

**Schema migration**
: An immutable, ordered database change applied transactionally before the native store becomes available. The migration ledger records its version, name, and checksum.

**Target statistics**
: Aggregates derived from canonical session and exercise outcomes for one exact target identifier. An absent accuracy value means no graded answers were recorded.
