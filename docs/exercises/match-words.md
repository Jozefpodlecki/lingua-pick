# Match words

## Purpose and status

Practice associations between target vocabulary and English meanings. The JSON proposal exists; the core variant, matching answer type, and UI are not implemented.

## Content

Require a bounded list of pairs. Each pair contains a unique stable ID, a target word or phrase, and an English meaning. Three pairs are the initial example; the request controls pair count.

The [AI contract](../ai-interaction.md) specifies the proposed payload. A word category constrains pair content without changing the interaction type.

## Interaction

Shuffle the two display columns independently. Select a target item and an English item to attempt a match. Keep original pair identities hidden from visible labels. Successful local matches stay paired; incorrect attempts release the selection and provide brief feedback.

The learner explicitly submits after completing all pairs. Do not consume one session exercise for each local pair attempt. Avoid a drag-only UI; both columns must support keyboard selection.

## Grading and validation

The proposed submitted answer maps each target item to an English item. The core should require every expected item exactly once and grade the relationships by stable identity. Incorrect attempts may be recorded for future scoring, but their scoring impact is undecided.

Reject blank text, duplicate pair IDs, duplicate-looking items that make the intended matching ambiguous, missing items, repeated item use, and count/vocabulary violations. Do not infer correctness solely from array positions after shuffling.

Word meaning depends on context. Include contextual phrases where a bare word has competing meanings; a well-formed pair list alone does not establish linguistic correctness.
