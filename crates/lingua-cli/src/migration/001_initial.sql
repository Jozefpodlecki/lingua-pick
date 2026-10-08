CREATE TABLE exercise_category (
    name VARCHAR PRIMARY KEY,
    description VARCHAR NOT NULL
);

CREATE TABLE skill (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE topic (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE language (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    region VARCHAR NOT NULL,
    native_name VARCHAR NOT NULL
);

CREATE TABLE language_feature (
    id VARCHAR PRIMARY KEY CHECK (length(trim(id)) > 0),
    name VARCHAR NOT NULL CHECK (length(trim(name)) > 0),
    description VARCHAR NOT NULL CHECK (length(trim(description)) > 0),
    group_code VARCHAR NOT NULL CHECK (length(trim(group_code)) > 0),
    display_order INTEGER NOT NULL DEFAULT 0 CHECK (display_order >= 0)
);

CREATE TABLE language_feature_value (
    feature_id VARCHAR NOT NULL REFERENCES language_feature(id),
    code VARCHAR NOT NULL CHECK (length(trim(code)) > 0),
    badge_label VARCHAR NOT NULL CHECK (length(trim(badge_label)) > 0),
    description VARCHAR NOT NULL CHECK (length(trim(description)) > 0),
    PRIMARY KEY (feature_id, code)
);

CREATE TABLE language_feature_assignment (
    language_id VARCHAR NOT NULL REFERENCES language(id),
    feature_id VARCHAR NOT NULL,
    value_code VARCHAR NOT NULL,
    notes VARCHAR,
    example VARCHAR,
    PRIMARY KEY (language_id, feature_id),
    FOREIGN KEY (feature_id, value_code)
        REFERENCES language_feature_value(feature_id, code)
);

CREATE TABLE script (
    id VARCHAR PRIMARY KEY CHECK (length(id) = 4),
    name VARCHAR NOT NULL CHECK (length(trim(name)) > 0)
);

CREATE TABLE language_script (
    language_id VARCHAR NOT NULL REFERENCES language(id),
    script_id VARCHAR NOT NULL REFERENCES script(id),
    PRIMARY KEY (language_id, script_id)
);

CREATE TABLE concept (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    language_id VARCHAR NOT NULL,
    skill_id VARCHAR NOT NULL,
    topic_id VARCHAR NOT NULL REFERENCES topic(id),
    code VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    UNIQUE (language_id, code),
    UNIQUE (language_id, id),
    FOREIGN KEY (language_id) REFERENCES language(id),
    FOREIGN KEY (skill_id) REFERENCES skill(id)
);

CREATE TABLE language_concept (
    language_id VARCHAR NOT NULL,
    concept_id UUID NOT NULL,
    PRIMARY KEY (language_id, concept_id),
    FOREIGN KEY (language_id) REFERENCES language(id),
    FOREIGN KEY (language_id, concept_id) REFERENCES concept(language_id, id)
);

CREATE TABLE concept_dependency (
    concept_id UUID NOT NULL,
    prerequisite_id UUID NOT NULL,
    PRIMARY KEY (concept_id, prerequisite_id),
    CHECK (concept_id <> prerequisite_id),
    FOREIGN KEY (concept_id) REFERENCES concept(id),
    FOREIGN KEY (prerequisite_id) REFERENCES concept(id)
);

CREATE TABLE user (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    source_language_id VARCHAR NOT NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    username VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL
);

CREATE TABLE user_stats (
    user_id UUID NOT NULL REFERENCES user(id),
    target_language_id VARCHAR NOT NULL REFERENCES language(id),
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    struggling_categories JSON NOT NULL DEFAULT '[]',
    PRIMARY KEY (user_id, target_language_id)
);

CREATE TABLE user_concept (
    user_id UUID NOT NULL,
    target_language_id VARCHAR NOT NULL,
    concept_id UUID NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    correct INTEGER NOT NULL DEFAULT 0 CHECK (correct >= 0 AND correct <= attempts),
    mastery DOUBLE,
    confidence DOUBLE,
    last_seen TIMESTAMPTZ,
    updated_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (user_id, target_language_id, concept_id),
    FOREIGN KEY (user_id, target_language_id) REFERENCES user_stats(user_id, target_language_id),
    FOREIGN KEY (target_language_id, concept_id) REFERENCES language_concept(language_id, concept_id)
);

CREATE TABLE session (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL,
    target_language_id VARCHAR NOT NULL,
    last_exercise_id UUID NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    exercise_count USMALLINT NOT NULL CHECK (exercise_count <= max_exercise_count),
    max_exercise_count USMALLINT NOT NULL CHECK (max_exercise_count > 0),
    FOREIGN KEY (user_id, target_language_id) REFERENCES user_stats(user_id, target_language_id)
);
   
CREATE TABLE exercise_definition (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    kind VARCHAR NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    category VARCHAR NOT NULL REFERENCES exercise_category(name),
    instructions VARCHAR NOT NULL,
    schema JSON NOT NULL,
    answer_schema JSON,
    verdict_schema JSON,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    UNIQUE (id, kind)
);

CREATE TABLE exercise (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    definition_id UUID NOT NULL REFERENCES exercise_definition(id),
    session_id UUID NOT NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    kind VARCHAR NOT NULL,
    payload JSON NOT NULL,
    answer JSON NULL,
    verdict JSON NULL,
    answered_on TIMESTAMPTZ NULL,
    interaction_mode VARCHAR NOT NULL DEFAULT 'single_turn' CHECK (interaction_mode IN ('single_turn', 'multi_turn')),
    FOREIGN KEY (definition_id, kind) REFERENCES exercise_definition(id, kind),
    FOREIGN KEY (session_id) REFERENCES session(id)
);

CREATE TABLE exercise_turn (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    exercise_id UUID NOT NULL,
    turn_number USMALLINT NOT NULL CHECK (turn_number > 0),
    role VARCHAR NOT NULL CHECK (role IN ('system', 'assistant', 'user')),
    content VARCHAR NOT NULL,
    evaluation JSON,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    UNIQUE (exercise_id, turn_number),
    FOREIGN KEY (exercise_id) REFERENCES exercise(id)
);

CREATE TABLE exercise_definition_skill (
    definition_id UUID NOT NULL,
    skill_id VARCHAR NOT NULL,
    PRIMARY KEY (definition_id, skill_id),
    FOREIGN KEY (definition_id) REFERENCES exercise_definition(id),
    FOREIGN KEY (skill_id) REFERENCES skill(id)
);

CREATE TABLE exercise_concept (
    exercise_id UUID NOT NULL,
    concept_id UUID NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (exercise_id, concept_id),
    FOREIGN KEY (exercise_id) REFERENCES exercise(id),
    FOREIGN KEY (concept_id) REFERENCES concept(id)
);

CREATE TABLE language_exercise (
    language_id VARCHAR NOT NULL,
    definition_id UUID NOT NULL,
    PRIMARY KEY (language_id, definition_id),
    FOREIGN KEY (language_id) REFERENCES language(id),
    FOREIGN KEY (definition_id) REFERENCES exercise_definition(id)
);

CREATE TABLE learning_evidence (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL,
    target_language_id VARCHAR NOT NULL,
    exercise_id UUID NOT NULL REFERENCES exercise(id),
    concept_id UUID NOT NULL,
    event_key VARCHAR NOT NULL,
    evidence_mode VARCHAR NOT NULL CHECK (evidence_mode IN ('recognition', 'recall', 'production', 'comprehension', 'exposure', 'practice')),
    correct BOOLEAN,
    assisted BOOLEAN NOT NULL DEFAULT FALSE,
    occurred_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    evaluator VARCHAR NOT NULL,
    policy_version VARCHAR NOT NULL,
    detail JSON NOT NULL DEFAULT '{}',
    UNIQUE (exercise_id, concept_id, event_key),
    FOREIGN KEY (user_id, target_language_id) REFERENCES user_stats(user_id, target_language_id),
    FOREIGN KEY (target_language_id, concept_id) REFERENCES language_concept(language_id, concept_id),
    CHECK ((evidence_mode IN ('exposure', 'practice') AND correct IS NULL)
        OR (evidence_mode IN ('recognition', 'recall', 'production', 'comprehension') AND correct IS NOT NULL))
);

CREATE VIEW learner_concept_evidence AS
SELECT e.user_id, e.target_language_id, e.concept_id, e.evidence_mode,
    count(*) AS encounters,
    count(*) FILTER (WHERE e.correct IS NOT NULL AND NOT e.assisted) AS assessed_attempts,
    count(*) FILTER (WHERE e.correct = TRUE AND NOT e.assisted) AS correct_attempts,
    count(DISTINCT x.session_id) FILTER (WHERE e.correct IS NOT NULL AND NOT e.assisted) AS assessed_sessions,
    max(e.occurred_on) AS last_seen,
    avg(CASE WHEN NOT e.assisted THEN CAST(e.correct AS INTEGER) END) AS accuracy
FROM learning_evidence e
JOIN exercise x ON x.id = e.exercise_id
GROUP BY e.user_id, e.target_language_id, e.concept_id, e.evidence_mode;

CREATE TABLE learning_stage (
    code VARCHAR PRIMARY KEY,
    sequence USMALLINT NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    generation_instructions VARCHAR NOT NULL,
    evaluation_instructions VARCHAR NOT NULL
);

CREATE TABLE learning_stage_dependency (
    stage_code VARCHAR NOT NULL REFERENCES learning_stage(code),
    prerequisite_code VARCHAR NOT NULL REFERENCES learning_stage(code),
    PRIMARY KEY (stage_code, prerequisite_code),
    CHECK (stage_code <> prerequisite_code)
);

CREATE TABLE learning_stage_skill (
    stage_code VARCHAR NOT NULL REFERENCES learning_stage(code),
    skill_id VARCHAR NOT NULL REFERENCES skill(id),
    evidence_mode VARCHAR NOT NULL CHECK (evidence_mode IN ('recognition', 'recall', 'production', 'comprehension')),
    PRIMARY KEY (stage_code, skill_id, evidence_mode)
);

CREATE TABLE learning_stage_exercise (
    stage_code VARCHAR NOT NULL REFERENCES learning_stage(code),
    definition_id UUID NOT NULL REFERENCES exercise_definition(id),
    selection_instructions VARCHAR NOT NULL,
    PRIMARY KEY (stage_code, definition_id)
);

CREATE TABLE teaching_guideline (
    code VARCHAR PRIMARY KEY,
    category VARCHAR NOT NULL REFERENCES exercise_category(name),
    generation_instructions VARCHAR NOT NULL,
    evaluation_instructions VARCHAR NOT NULL
);

CREATE TABLE skill_dependency (
    skill_id VARCHAR NOT NULL REFERENCES skill(id),
    prerequisite_id VARCHAR NOT NULL REFERENCES skill(id),
    PRIMARY KEY (skill_id, prerequisite_id),
    CHECK (skill_id <> prerequisite_id)
);

CREATE TABLE topic_stage (
    topic_id VARCHAR NOT NULL REFERENCES topic(id),
    stage_code VARCHAR NOT NULL REFERENCES learning_stage(code),
    generation_instructions VARCHAR NOT NULL,
    PRIMARY KEY (topic_id, stage_code)
);

