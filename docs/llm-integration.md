# LM Studio exercise generation

Status: researched proposal. No LM Studio client, translation exercise, or matching exercise is implemented yet. This document assumes the requested application is LM Studio.

See [AI interaction contract](ai-interaction.md) for the proposed system prompt, target-language context, and available/selected word categories.

## API approach

LM Studio supports schema-constrained output on `POST /v1/chat/completions`. Supply `response_format.type = "json_schema"` with a named schema; start with `stream: false`. Parse the HTTP response, then parse the JSON string in `choices[0].message.content`. Structured output support depends on the loaded model, so test the actual model rather than assuming compatibility. [Official structured-output documentation](https://lmstudio.ai/docs/developer/openai-compat/structured-output).

Configure the base URL, model identifier, and optional authentication token outside the learning core. The usual local base URL is `http://localhost:1234/v1`. Discover model identifiers through `GET /v1/models`; the list can include downloaded models eligible for just-in-time loading. [Official models endpoint](https://lmstudio.ai/docs/developer/openai-compat/models).

Run the server from LM Studio's Developer tab. Direct requests from the frontend need CORS enabled because the app and API have different origins. LM Studio also supports requiring a token in the `Authorization` header. [Server settings](https://lmstudio.ai/docs/developer/core/server/settings), [authentication](https://lmstudio.ai/docs/developer/core/authentication).

## Proposed responsibilities

- `lingua-core`: exercise payloads, answer types, validation, and deterministic grading rules. Keep `no_std` with `alloc`; no HTTP or browser dependencies.
- Web generation service: connection settings, request/response structs, prompts, HTTP calls, cancellation, timeouts, and parsing. Existing `gloo-net` facilities are a candidate; inspect the enabled features before implementing.
- Exercise components: translation input and matching cards. Render validated data as text.
- Session integration: accept a validated exercise batch for its requested target and discard late results after switching languages or cancelling a request.

Begin with one exercise per request and one schema per requested format. The app supplies target, translation direction, requested level, allowed vocabulary, and exercise type. The app assigns stable exercise IDs and binds the result to the requested target. Do not let the model select arbitrary target IDs or define executable UI.

## Proposed payloads

These are generation payloads, not JSON accepted by today's `ExerciseContent`, which only supports `single_choice`. Successful validation would convert them into future core variants.

Translation works in both directions and offers typing or a word bank on the same exercise. The complete payload and input behavior are specified in [translation directions and input modes](ai-interaction.md#translation-directions-and-input-modes). The example below shows the translation fields only; the complete generation schema must also require the word bank.

```json
{
  "type": "translate_sentence",
  "data": {
    "direction": "target_to_english",
    "sentence": "我喜欢喝茶。",
    "accepted_answers": ["I like drinking tea.", "I like to drink tea."],
    "explanation": "喜欢 expresses liking something."
  }
}
```

Matching example:

```json
{
  "type": "match_words",
  "data": {
    "pairs": [
      {"id": "pair-1", "target": "猫", "english": "cat"},
      {"id": "pair-2", "target": "狗", "english": "dog"},
      {"id": "pair-3", "target": "马", "english": "horse"}
    ]
  }
}
```

The frontend independently shuffles the matching columns and submits pair identifiers. Validate nonblank values, unique pair IDs, and unambiguous display values before rendering. The core checks matches by identity rather than guessing from translated strings.

Translation needs a deliberate grading policy. A first version could compare normalized text against accepted answers, but that can reject other valid translations. Normalization must preserve meaningful distinctions; never apply blanket accent removal to answer grading. Optional model-assisted grading would be a separate request and should handle uncertainty. Generated answer lists are not authoritative linguistic verification.

## Request contract and validation

Use a JSON Schema with all expected properties required, `additionalProperties: false`, a fixed `type` for each request, and bounded array sizes. Keep the first schema simple and verify which constraints the selected model/runtime supports. Include instructions to return only the requested exercise and use the specified target variety and vocabulary.

The request envelope should contain `model`, `messages`, `stream: false`, a sufficient `max_tokens`, and:

```json
{
  "response_format": {
    "type": "json_schema",
    "json_schema": {
      "name": "translate_sentence_v1",
      "strict": true,
      "schema": {
        "type": "object",
        "additionalProperties": false,
        "required": ["type", "data"],
        "properties": {
          "type": {"type": "string", "enum": ["translate_sentence"]},
          "data": {
            "type": "object",
            "additionalProperties": false,
            "required": ["direction", "sentence", "accepted_answers", "explanation"],
            "properties": {
              "direction": {"type": "string", "enum": ["target_to_english"]},
              "sentence": {"type": "string"},
              "accepted_answers": {
                "type": "array",
                "minItems": 1,
                "maxItems": 5,
                "items": {"type": "string"}
              },
              "explanation": {"type": "string"}
            }
          }
        }
      }
    }
  }
}
```

The snippet is an initial response-format configuration for target-to-English translation, not a complete HTTP request or a schema tested against a local model. It predates the word-bank requirement: extend it with the word-bank fields and constrain direction to the requested value before implementation. Store complete versioned schemas in `assets/schemas/` when implementing and keep them synchronized with core payloads.

Reject HTTP errors, absent content, truncated responses, invalid JSON, unexpected formats, blank fields, excessive lengths, and invalid answer relationships. Limit repair attempts; do not retry forever. Preserve error details for diagnostics while giving the learner a short actionable failure state. Schema conformity does not establish translation accuracy, syllabus alignment, or a correct answer key.

## Hosting

First verify the integration with the locally served app and LM Studio running on the same machine. GitHub Pages hosts the frontend; it does not run LM Studio. A loopback URL refers to the learner's computer, and browser local-network policies or permissions can affect a deployed page's requests. Test deployed access separately rather than claiming that enabling CORS guarantees it. [Browser local-network access](https://developer.mozilla.org/en-US/docs/Web/Security/Defenses/Local_network_access).

Keep tokens out of repository assets and build output. If authentication is used, start with a session-only token setting. Offline authored exercises should remain possible when generation is unavailable.

## Implementation order

1. Add translation and matching payloads, answer types, and validation to the core, deciding translation grading behavior.
2. Add complete JSON schemas and example fixtures.
3. Add a web generation service and connection settings, using non-streaming requests first.
4. Add modular exercise components and connect validated results to sessions.
5. Add focused tests after the implementation settles, then verify with the actual loaded model and the local and deployed frontend origins.
