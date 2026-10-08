CREATE TABLE category (
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

CREATE TABLE concept (
    id UUID PRIMARY KEY,
    language_id VARCHAR NOT NULL,
    skill_id VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    UNIQUE (language_id, code),
    FOREIGN KEY (language_id) REFERENCES language(id),
    FOREIGN KEY (skill_id) REFERENCES skill(id)
);

CREATE TABLE language_concept (
    language_id VARCHAR NOT NULL,
    concept_id VARCHAR NOT NULL,
    PRIMARY KEY (language_id, concept_id),
    FOREIGN KEY (language_id) REFERENCES language(id),
    FOREIGN KEY (concept_id) REFERENCES concept(id)
);

CREATE TABLE concept_dependency (
    concept_id VARCHAR NOT NULL,
    prerequisite_id VARCHAR NOT NULL,
    PRIMARY KEY (concept_id, prerequisite_id),
    FOREIGN KEY (concept_id) REFERENCES concept(id),
    FOREIGN KEY (prerequisite_id) REFERENCES concept(id)
);

CREATE TABLE user (
    id UUID PRIMARY KEY,
    source_language_id VARCHAR NOT NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    username VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL
);

CREATE TABLE user_concept (
    user_id UUID NOT NULL,
    target_language_id VARCHAR NOT NULL,
    concept_id VARCHAR NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0,
    correct INTEGER NOT NULL DEFAULT 0,
    mastery DOUBLE NOT NULL DEFAULT 0.0,
    confidence DOUBLE NOT NULL DEFAULT 0.0,
    last_seen TIMESTAMPTZ,
    updated_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (user_id, target_language_id, concept_id),
    FOREIGN KEY (user_id, target_language_id) REFERENCES user_stats(user_id, target_language_id),
    FOREIGN KEY (concept_id) REFERENCES concept(id)
);

CREATE TABLE user_stats (
    user_id UUID PRIMARY KEY,
    target_language_id VARCHAR NOT NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE TABLE session (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    target_language_id VARCHAR NOT NULL,
    last_exercise_id UUID NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    exercise_count SMALLINT NOT NULL,
    max_exercise_count SMALLINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id)
);
   
CREATE TABLE exercise (
    id UUID PRIMARY KEY,
    definiton_id: UUID NOT NULL,
    session_id UUID NOT NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    kind VARCHAR NOT NULL,
    payload JSON NOT NULL,
    answer JSON NULL,
    verdict JSON NULL,
    answered_on TIMESTAMPTZ NULL,
    interaction_mode VARCHAR NOT NULL DEFAULT 'single_turn';
    FOREIGN KEY (session_id) REFERENCES session(id)
);

CREATE TABLE exercise_definition (
    id UUID PRIMARY KEY,
    kind VARCHAR NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    instructions VARCHAR NOT NULL,
    schema JSON NOT NULL,
    answer_schema JSON,
    verdict_schema JSON,
    created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

CREATE TABLE exercise_turn (
    id UUID PRIMARY KEY,
    exercise_id UUID NOT NULL,
    turn_number USMALLINT NOT NULL,
    role VARCHAR NOT NULL,
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
    concept_id VARCHAR NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (exercise_id, concept_id),
    FOREIGN KEY (exercise_id) REFERENCES exercise(id),
    FOREIGN KEY (concept_id) REFERENCES concept(id)
);

CREATE TABLE language_exercise (
    language_id VARCHAR NOT NULL,
    definiton_id: UUID NOT NULL,
    PRIMARY KEY (language_id, exercise_kind),
    FOREIGN KEY (language_id) REFERENCES language(id),
    FOREIGN KEY (definiton_id) REFERENCES exercise_definition(id)
);