-- Root features
INSERT INTO language_feature
(
    name,
    description
)
VALUES
('Word Order', 'Canonical ordering of subject, verb, and object'),
('Grammatical Gender', 'Whether nouns belong to grammatical gender classes or noun classes'),
('Case Alignment', 'Alignment system used in grammatical cases'),
('Number System', 'Number distinctions in nouns/pronouns'),
('Verb Conjugation', 'Verbs inflect for grammatical categories'),
('Verb Aspect', 'Grammatical marking of completed vs ongoing actions'),
('Mood System', 'Grammatical mood distinctions'),
('Tense System', 'Verbal tense distinctions'),
('Pro-drop', 'Whether null subjects are allowed'),
('Morphological Type', 'Morphological typology of the language'),
('Tone System', 'Presence and type of tone in the language'),
('Plural Formation', 'Method used to form plural nouns'),
('Negation', 'Grammatical marking of negation'),
('Politeness/Honorifics', 'Grammatical marking for politeness or social hierarchy'),
('Classifier System', 'Use of numeral or noun classifiers in the language'),
('Evidentiality', 'Grammatical marking of information source'),
('Copula System', 'How languages express predication such as "X is Y"');

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Word Order');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('SOV', 'Subject precedes object, which precedes verb', getvariable('parent_id')),
('SVO', 'Subject precedes verb, which precedes object', getvariable('parent_id')),
('VSO', 'Verb precedes subject, which precedes object', getvariable('parent_id')),
('VOS', 'Verb precedes object, which precedes subject', getvariable('parent_id')),
('OSV', 'Object precedes subject, which precedes verb', getvariable('parent_id')),
('OVS', 'Object precedes verb, which precedes subject', getvariable('parent_id')),
('Order Flexibility', 'How rigidly the language adheres to canonical word order', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Order Flexibility');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Rigid', 'Canonical word order is strictly followed', getvariable('parent_id')),
('Moderately flexible', 'Word order varies depending on discourse, focus, or topicalization', getvariable('parent_id')),
('Highly flexible', 'Constituent order is largely free and determined by pragmatics', getvariable('parent_id')),
('Topicalization', 'Constituent order may change to emphasize topics or contrast', getvariable('parent_id')),
('Question inversion', 'Word order changes in interrogative clauses', getvariable('parent_id')),
('Focus constructions', 'Elements move to prominent positions for contrastive focus', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Grammatical Gender');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('No grammatical gender', 'The language does not classify nouns by grammatical gender', getvariable('parent_id')),
('Masculine-Feminine', 'Nouns are classified as either masculine or feminine', getvariable('parent_id')),
('Masculine-Feminine-Neuter', 'Nouns are classified as masculine, feminine, or neuter', getvariable('parent_id')),
('Noun class system', 'Nouns are divided into multiple noun classes with specific agreement patterns', getvariable('parent_id')),
('Extensive noun class', 'Languages with a large and complex system of noun classes', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Case Alignment');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Core Alignment', 'Primary grammatical alignment pattern of the language', getvariable('parent_id')),
('Split Alignment', 'Alignment varies depending on grammatical or semantic conditions', getvariable('parent_id')),
('Neutral Alignment', 'Alignment distinctions are weak or absent', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Core Alignment');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Nominative-Accusative', 'Subjects of both transitive and intransitive verbs behave similarly', getvariable('parent_id')),
('Ergative-Absolutive', 'Subjects of intransitive verbs behave like objects of transitive verbs', getvariable('parent_id')),
('Active-Stative', 'Subject marking depends on semantic role such as volition or control', getvariable('parent_id')),
('Tripartite', 'Subjects of transitive verbs, subjects of intransitive verbs, and objects all receive different marking', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Split Alignment');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Split Ergative', 'Ergative alignment occurs only in certain grammatical contexts', getvariable('parent_id')),
('Split Accusative', 'Accusative alignment varies depending on tense, aspect, or person', getvariable('parent_id')),
('Fluid-S Alignment', 'Intransitive subjects may take different marking depending on semantic role', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Neutral Alignment');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Direct Alignment', 'Subjects and objects share the same morphological marking', getvariable('parent_id')),
('No overt alignment marking', 'Grammatical relations are expressed primarily by word order or particles', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Number System');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Singular/Plural only', 'Only distinguishes between singular and plural forms', getvariable('parent_id')),
('Dual', 'A special form exists for exactly two entities', getvariable('parent_id')),
('Trial', 'A special form exists for exactly three entities', getvariable('parent_id')),
('Paucal', 'A form exists for a few entities beyond two or three', getvariable('parent_id')),
('Polymorphic/other', 'Number marking varies irregularly or is highly complex', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Verb Conjugation');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Conjugates for person/number/tense', 'Verbs inflect to indicate grammatical person, number, and tense', getvariable('parent_id')),
('Minimal conjugation', 'Verbs show minimal or no inflection', getvariable('parent_id')),
('Polypersonal agreement', 'Verbs agree with multiple arguments (subject, object) simultaneously', getvariable('parent_id')),
('Templatic morphology', 'Verbal inflection is expressed via fixed root templates rather than affixes', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Verb Aspect');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Temporal Aspect', 'Aspect related to the temporal flow of actions', getvariable('parent_id')),
('Modal/Evidential Aspect', 'Aspect related to speaker perspective/modality/evidence', getvariable('parent_id')),
('Habitual/Iterative Aspect', 'Aspect indicating repetition or habitual occurrence', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Temporal Aspect');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Progressive (morphological)', 'Verb has a dedicated morphological form for progressive aspect', getvariable('parent_id')),
('Progressive (periphrastic)', 'Progressive expressed via auxiliary constructions', getvariable('parent_id')),
('Perfective (morphological)', 'Verb has a formal morphological perfective marker', getvariable('parent_id')),
('Perfective (analytic)', 'Perfective expressed via auxiliary or separate lexical means', getvariable('parent_id')),
('Imperfective', 'Verb expresses ongoing or continuous action without perfective contrast', getvariable('parent_id')),
('Aspect unmarked', 'No formal aspect marking in the temporal domain', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Modal/Evidential Aspect');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Evidential', 'Indicates source or reliability of information', getvariable('parent_id')),
('Potential', 'Indicates possibility, ability, or potential event', getvariable('parent_id')),
('Appreciative/Evaluative', 'Speaker’s attitude or evaluation toward the action', getvariable('parent_id')),
('Obligatoriness/Necessitative', 'Indicates requirement or necessity of action', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Habitual/Iterative Aspect');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Habitual (marked)', 'Dedicated morphological marking for habitual action', getvariable('parent_id')),
('Habitual (unmarked)', 'Habitual meaning only via lexical or syntactic means', getvariable('parent_id')),
('Iterative', 'Repeated or frequent occurrence of an action', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Mood System');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Indicative', 'Statements of fact or reality', getvariable('parent_id')),
('Subjunctive', 'Hypothetical or non-real situations', getvariable('parent_id')),
('Imperative', 'Commands or requests', getvariable('parent_id')),
('Optative', 'Wishes or hopes', getvariable('parent_id')),
('Jussive', 'Commands or exhortations in third person', getvariable('parent_id')),
('Conditional', 'Actions dependent on a condition', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Tense System');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Present/Past/Future', 'Standard tense distinctions for present, past, and future', getvariable('parent_id')),
('Absolute tense', 'Tense is fixed relative to the moment of speaking', getvariable('parent_id')),
('Relative tense', 'Tense is marked relative to another event', getvariable('parent_id')),
('No grammatical tense', 'The language does not formally mark tense', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Pro-drop');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Subject Pro-drop', 'Subject omission behaviour in the language', getvariable('parent_id')),
('Object Pro-drop', 'Object omission behaviour in the language', getvariable('parent_id')),
('Discourse Drop', 'Argument omission driven by discourse or topic structure', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Subject Pro-drop');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Canonical subject pro-drop', 'Subjects are routinely omitted due to rich agreement morphology', getvariable('parent_id')),
('Restricted subject pro-drop', 'Subject omission is limited to certain contexts or constructions', getvariable('parent_id')),
('No subject pro-drop', 'Subjects must generally be overtly expressed', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Object Pro-drop');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Object drop allowed', 'Objects may be omitted when recoverable from context', getvariable('parent_id')),
('Object drop disallowed', 'Objects must generally be overtly expressed', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Discourse Drop');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Topic drop', 'Arguments can be omitted when recoverable from discourse context', getvariable('parent_id')),
('No discourse drop', 'Arguments cannot normally be omitted based on discourse context', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Morphological Type');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Isolating', 'Words are mostly uninflected and grammatical relations are expressed via word order', getvariable('parent_id')),
('Agglutinative', 'Words are formed by stringing together morphemes each with a single grammatical meaning', getvariable('parent_id')),
('Fusional', 'Morphemes express multiple grammatical meanings simultaneously', getvariable('parent_id')),
('Polysynthetic', 'Words combine many morphemes, often representing a whole sentence', getvariable('parent_id')),
('Templatic', 'Morphology is based on consonant/vowel templates rather than linear affixes', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Tone System');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Tone Presence', 'Whether lexical tone exists in the language', getvariable('parent_id')),
('Tone Type', 'Structural type of tonal system', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Tone Presence');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('No tone', 'Pitch does not distinguish lexical meaning', getvariable('parent_id')),
('Tonal language', 'Pitch distinguishes lexical or grammatical meaning', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Tone Type');

INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Register tone', 'Tone distinguishes level pitches such as high or low', getvariable('parent_id')),
('Contour tone', 'Tone involves rising or falling pitch patterns', getvariable('parent_id')),
('Mixed tone', 'Language uses both contour and register tones', getvariable('parent_id')),
('Pitch accent', 'Only one syllable per word carries a prominent pitch', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Plural Formation');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Suffix', 'Plural is formed by adding a suffix to the noun', getvariable('parent_id')),
('Prefix', 'Plural is formed by adding a prefix to the noun', getvariable('parent_id')),
('Internal vowel change', 'Plural is formed by changing the vowels inside the noun', getvariable('parent_id')),
('Reduplication', 'Plural is indicated by repeating part or all of the noun', getvariable('parent_id')),
('No plural marking', 'Plural is not morphologically marked', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Negation');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Negation Marker Type', 'Morphological form used to express negation', getvariable('parent_id')),
('Negation Position', 'Position of the negation marker relative to the verb', getvariable('parent_id')),
('Negation Strategy', 'Overall structural pattern of negation', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Negation Position');

INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Preverbal negation', 'Negative marker appears before the verb', getvariable('parent_id')),
('Postverbal negation', 'Negative marker appears after the verb', getvariable('parent_id')),
('Bipartite negation', 'Negation uses two markers surrounding the verb', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Negation Marker Type');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Negation particle', 'A separate particle expresses negation', getvariable('parent_id')),
('Negation affix', 'Negation expressed via prefix or suffix', getvariable('parent_id')),
('Auxiliary negation', 'A special auxiliary verb expresses negation', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Negation Strategy');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Single negation', 'Only one negative marker appears in the clause', getvariable('parent_id')),
('Double negation', 'Multiple negative markers appear in the clause', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Politeness/Honorifics');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Morphological Honorifics', 'Politeness or social hierarchy is marked through grammatical morphology', getvariable('parent_id')),
('Lexical Honorifics', 'Different vocabulary items are used depending on social hierarchy or politeness', getvariable('parent_id')),
('Pragmatic Politeness', 'Politeness is expressed through pragmatic or discourse strategies rather than grammar', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Morphological Honorifics');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Honorific verb forms', 'Verbs change form depending on the social status of speaker, listener, or subject', getvariable('parent_id')),
('Politeness verb endings', 'Dedicated verb endings indicate politeness level', getvariable('parent_id')),
('Honorific pronouns', 'Different pronouns are used depending on social hierarchy', getvariable('parent_id')),
('Honorific affixes', 'Prefixes or suffixes attached to words express respect or humility', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Lexical Honorifics');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Honorific titles', 'Special titles or address forms are used to mark respect', getvariable('parent_id')),
('Humble verb forms', 'Special verbs are used to lower the speaker’s status relative to the listener', getvariable('parent_id')),
('Respectful verb forms', 'Special verbs are used to elevate the status of the listener or subject', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Pragmatic Politeness');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Indirect requests', 'Politeness expressed through indirect phrasing of requests or commands', getvariable('parent_id')),
('Deferential phrasing', 'Politeness conveyed through softened or deferential sentence constructions', getvariable('parent_id')),
('Avoidance strategies', 'Certain words or direct forms are avoided when speaking to higher-status individuals', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Classifier System');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('No classifiers', 'Language does not use numeral classifiers', getvariable('parent_id')),
('Optional classifiers', 'Classifier usage is optional', getvariable('parent_id')),
('Obligatory classifiers', 'Numerals require classifiers when modifying nouns', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Evidentiality');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Direct evidential', 'Speaker directly witnessed the event', getvariable('parent_id')),
('Inferential evidential', 'Speaker infers the event from evidence', getvariable('parent_id')),
('Reportative evidential', 'Information reported by others', getvariable('parent_id')),
('Hearsay evidential', 'Information heard but not verified', getvariable('parent_id'));

SET VARIABLE parent_id = (SELECT id FROM language_feature WHERE name = 'Copula System');
INSERT INTO language_feature
(
    name,
    description,
    parent_id
)
VALUES
('Overt copula', 'A verb equivalent to "be" is used', getvariable('parent_id')),
('Zero copula', 'Copula verb omitted in some contexts', getvariable('parent_id')),
('Mixed copula system', 'Copula appears only in certain tenses or constructions', getvariable('parent_id'));

