# Language features

## Status and scope

The experimental `lingua-cli` harness has language-feature tables, SQL seed data, typed models, and read stores. UI badge rendering and generator context integration remain pending. No build or SQL execution was performed for this change under the current session restrictions.

All schema is in `001_initial.sql`. `003_data_language_feature.sql` seeds 12 definitions, 41 allowed values, and 52 assignments: article and noun-gender metadata for the 14 English targets, definite articles for the seven Portuguese targets, and articles and noun gender for Polish. Other features and targets are unassigned until reviewed. Regional English assignments describe standard written English; Portuguese assignments describe the shared existence of definite articles, not equivalent regional usage.

## Storage contract

- `language_feature` defines a stable ID, English display name, precise scope description, group code, and display order. Initial groups are grammar, syntax, and phonology.
- `language_feature_value` defines allowed codes, badge labels, and explanations for each feature. Its composite key ensures a value belongs to its feature.
- `language_feature_assignment` assigns one value per feature per exact target, with optional explanatory notes and an original short example. Foreign keys reject unknown targets and feature/value combinations.

Missing assignments mean unknown. An absent value is an explicit linguistic assertion. The UI should not turn missing rows into negative badges. Script badges can use the existing script associations rather than duplicate them here. A trait's presence neither enables an exercise automatically nor establishes learner knowledge.

The catalogue covers definite and indefinite articles, noun gender, noun cases, plural formation, verb conjugation, grammatical tense, adjective agreement, basic word order, word-order flexibility, lexical tone, and word stress. Word order and flexibility are separate features so a target can be both subject-verb-object and flexible. Noun gender concerns productive noun-class agreement; gendered pronouns alone do not establish that feature. Polish's three principal singular genders are a teaching summary, with masculine subtypes and plural distinctions retained in notes.

## Store access

`SeedResult` exposes `language_feature_store`, `language_feature_value_store`, and `language_feature_assignment_store`. Definitions and values are read from the reference seed; these stores do not add mutation APIs yet.

```rust
let badges = stores.language_feature_store.for_language("pt-BR")?;
```

`for_language` returns `LanguageFeatureBadge` records containing feature identity, name, scope, group and order, value code, badge label, explanation, notes, and example. Results are ordered by display order and feature ID. It returns only assigned features. Assignment reads retain the language and feature keys, and nullable notes/examples remain `Option<String>`.

## Sources and seed interpretation

Seed descriptions, badge labels, notes, and examples are authored for this project rather than copied from reference prose. The initial assertions were checked against:

- [Cambridge: articles](https://dictionary.cambridge.org/grammar/british-grammar/the) for English definite and indefinite articles.
- [Cambridge: nouns and gender](https://dictionary.cambridge.org/us/grammar/british-grammar/nouns-and-gender) for the distinction between ordinary English nouns and lexical gender pairs. The absent noun-gender assignment uses the narrower noun-class agreement definition above.
- [WALS: Polish articles](https://wals.info/valuesets/38A-pol) for the absence of definite and indefinite articles.
- [Oscar Swan: First Year Polish, lesson 1](https://lektorek.org/lektorek/firstyear/lessons/lesson1.pdf) for principal noun genders and agreement.
- [WALS: Portuguese definite articles](https://wals.info/valuesets/37A-por) for the general definite-article assertion. Applying this shared trait to the seven regional targets does not imply that the source independently surveys every variety.

WALS data is licensed under CC BY 4.0; see its linked attribution and references. Further target assignments need equally explicit definitions and reviewed sources.

## Migration numbering

Shared migrations occupy 001-010; 011-099 are reserved for future shared metadata. Concept seeds run from `100_data_concept_pt-BR.sql` through `249_data_concept_sw-TZ.sql`. Registry versions match filename prefixes. The 160 registered migrations intentionally have a gap between 010 and 100; the loader uses registered versions rather than requiring consecutive numbers.

Existing disposable database files are incompatible after this schema and numbering change. Recreate them only after the runner stops. Startup continues to validate existing files without modifying or upgrading them.
