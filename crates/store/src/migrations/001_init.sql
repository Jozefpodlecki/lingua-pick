CREATE TABLE metadata (
    id BOOLEAN PRIMARY KEY DEFAULT TRUE,
    version NVARCHAR(10) NOT NULL,
    updated_on TIMESTAMP NOT NULL
);

CREATE TABLE migrations (
    id UUID NOT NULL PRIMARY KEY,
    file_name NVARCHAR(50) NOT NULL,
    applied_on TIMESTAMP NOT NULL
);

CREATE TABLE user (
    id UUID NOT NULL PRIMARY KEY,
    created_on TIMESTAMP NOT NULL,
    username NVARCHAR(50) NOT NULL,
    password_hash BLOB NOT NULL
);

CREATE TABLE user_profile (
    id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL,
    created_on TIMESTAMP NOT NULL,
    updated_on TIMESTAMP NOT NULL,
    is_active BOOL NOT NULL,
    source_language_id UUID NOT NULL,
    target_language_id UUID NOT NULL
);

CREATE TABLE language (
    id UUID NOT NULL PRIMARY KEY,
    created_on TIMESTAMP NOT NULL,
    name NVARCHAR(50) NOT NULL,
    iso639_1 CHAR(2) NULL,
    iso639_3 CHAR(3) NOT NULL,
    parent_id UUID NULL,
    UNIQUE (iso639_3)
);

CREATE TABLE session (
    id UUID NOT NULL PRIMARY KEY,
    user_profile_id UUID NOT NULL,
    created_on TIMESTAMP NOT NULL,
    updated_on TIMESTAMP NOT NULL,
    completed_on TIMESTAMP NULL,
    exercise_count TINYINT NOT NULL,
    total_exercise_count TINYINT NOT NULL,
    current_exercise_id UUID NULL,
    FOREIGN KEY (user_profile_id) REFERENCES user_profile(id)
);

CREATE TABLE session_user (
    session_id UUID NOT NULL,
    user_id UUID NOT NULL,
    PRIMARY KEY(session_id, user_id),
    FOREIGN KEY (session_id) REFERENCES session(id),
    FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE TABLE exercise_type (
    id UUID PRIMARY KEY,
    created_on TIMESTAMP NOT NULL,
    name NVARCHAR(50) NOT NULL,
    description TEXT NULL,
    modality NVARCHAR(20) NOT NULL
);

CREATE TABLE exercise (
    id UUID NOT NULL PRIMARY KEY,
    type_id UUID NOT NULL,
    session_id UUID NOT NULL,
    created_on TIMESTAMP NOT NULL,
    completed_on TIMESTAMP NULL,
    data TEXT NOT NULL,
    result TEXT NULL,
    verdict TEXT NULL,
    FOREIGN KEY (type_id) REFERENCES exercise_type(id),
);

CREATE TABLE exercise_allowed_type (
    source_language_id UUID NOT NULL REFERENCES language(id),
    target_language_id UUID NOT NULL REFERENCES language(id),
    type_id UUID NOT NULL REFERENCES exercise_type(id),
    PRIMARY KEY(source_language_id, target_language_id, type_id)
);

CREATE TABLE script (
    id UUID PRIMARY KEY,
    name NVARCHAR(50) NOT NULL,
    iso15924 CHAR(4) NOT NULL UNIQUE
);

CREATE TABLE language_script (
    language_id UUID NOT NULL,
    script_id UUID NOT NULL,
    is_primary BOOLEAN DEFAULT TRUE,
    PRIMARY KEY (language_id, script_id),
    FOREIGN KEY (language_id) REFERENCES language(id),
    FOREIGN KEY (script_id) REFERENCES script(id)
);

CREATE TABLE character (
    id UUID PRIMARY KEY,
    language_id UUID NOT NULL,
    char NVARCHAR(10) NOT NULL,
    script_id UUID NOT NULL,
    FOREIGN KEY (script_id) REFERENCES script(id),
    UNIQUE(language_id, char)
);

CREATE TABLE character_reading (
    id UUID PRIMARY KEY,
    character_id UUID NOT NULL,
    system NVARCHAR(50) NOT NULL,
    value TEXT NOT NULL,
    UNIQUE(character_id, system, value)
);

CREATE TABLE lexeme (
    id UUID PRIMARY KEY,
    language_id UUID NOT NULL,
    created_on TIMESTAMP NOT NULL,
    text NVARCHAR(100) NOT NULL,
    normalized NVARCHAR(100) NULL
);

CREATE TABLE lexeme_reading (
    id UUID PRIMARY KEY,
    lexeme_id UUID NOT NULL REFERENCES lexeme(id),
    system NVARCHAR(50) NOT NULL,
    value NVARCHAR(50) NOT NULL,
    UNIQUE(lexeme_id, system, value)
);

CREATE TABLE meaning (
    id UUID PRIMARY KEY,
    created_on TIMESTAMP NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE lexeme_meaning (
    lexeme_id UUID NOT NULL REFERENCES lexeme(id),
    meaning_id UUID NOT NULL REFERENCES meaning(id),
    PRIMARY KEY(lexeme_id, meaning_id)
);

CREATE TABLE classification_system (
    id UUID PRIMARY KEY,
    language_id UUID NOT NULL,
    name NVARCHAR(50) NOT NULL
);

CREATE TABLE classification_level (
    id UUID PRIMARY KEY,
    system_id UUID NOT NULL,
    name NVARCHAR(50) NOT NULL,
    difficulty INTEGER NOT NULL
);

CREATE TABLE lexeme_classification (
    lexeme_id UUID NOT NULL REFERENCES lexeme(id),
    level_id UUID NOT NULL REFERENCES classification_level(id),
    PRIMARY KEY(lexeme_id, level_id)
);

CREATE TABLE user_skill_metric (
    id UUID NOT NULL PRIMARY KEY,
    metric TEXT NOT NULL,
    value DOUBLE NOT NULL,
    updated_on TIMESTAMP NOT NULL,
    FOREIGN KEY (id) REFERENCES user_profile(id)
);

CREATE TABLE user_character_stat (
    id UUID NOT NULL REFERENCES user_profile(id),
    character_id UUID NOT NULL,
    times_seen INT NOT NULL DEFAULT 0,
    times_correct INT NOT NULL DEFAULT 0,
    times_pronounced_correct INT NOT NULL DEFAULT 0,
    times_heard_correct INT NOT NULL DEFAULT 0,
    last_seen TIMESTAMP NULL,
    PRIMARY KEY(id, character_id)
);

CREATE TABLE user_lexeme_stat (
    id UUID NOT NULL REFERENCES user_profile(id),
    lexeme_id UUID NOT NULL,
    times_seen INT NOT NULL DEFAULT 0,
    times_correct INT NOT NULL DEFAULT 0,
    times_pronounced_correct INT NOT NULL DEFAULT 0,
    times_heard_correct INT NOT NULL DEFAULT 0,
    last_seen TIMESTAMP NULL,
    PRIMARY KEY(id, lexeme_id)
);

CREATE INDEX IX_user_profile_user_id ON user_profile(user_id);
CREATE INDEX IX_exercise_session_id ON exercise(session_id);
CREATE INDEX IX_user_profile_active_lang ON user_profile(is_active, source_language_id, target_language_id);
CREATE INDEX IX_session_user_user_id ON session_user(user_id);
CREATE INDEX IX_character_script ON character(script_id);
CREATE INDEX IX_language_iso639_1 ON language(iso639_1);