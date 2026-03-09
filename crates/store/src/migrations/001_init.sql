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
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    username NVARCHAR(50) NOT NULL,
    password_hash BLOB NOT NULL
);

CREATE TABLE user_profile (
    id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL,
    created_on TIMESTAMP NOT NULL,
    updated_on TIMESTAMP NOT NULL,
    is_active BOOL NOT NULL,
    source_language_id INTEGER NOT NULL,
    target_language_id INTEGER NOT NULL
);

CREATE SEQUENCE part_of_speech_sequence;

CREATE TABLE part_of_speech (
    id INTEGER PRIMARY KEY DEFAULT nextval('part_of_speech_sequence'),
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name NVARCHAR(50) NOT NULL UNIQUE,
    description TEXT NULL
);

CREATE SEQUENCE language_sequence;
CREATE TABLE language (
    id INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('language_sequence'),
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name NVARCHAR(50) NOT NULL,
    iso639_1 CHAR(2) NULL,
    iso639_3 CHAR(3) NULL,
    ietf_bcp_47 NVARCHAR(20) NOT NULL,
    is_dialect BOOLEAN DEFAULT FALSE,
    is_regional_standard BOOLEAN DEFAULT FALSE,
    is_macro_language BOOLEAN DEFAULT FALSE,
    parent_id INTEGER NULL REFERENCES language(id)
);

CREATE SEQUENCE language_feature_sequence;
CREATE TABLE language_feature (
    id INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('language_feature_sequence'),
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    language_id INTEGER NOT NULL REFERENCES language(id),
    name VARCHAR(100) NOT NULL,
    description TEXT NULL ,
    parent_id INTEGER NULL REFERENCES language_feature(id)
);

CREATE SEQUENCE language_asset_sequence;
CREATE TABLE language_asset (
    id INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('language_asset_sequence'),
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    file_name NVARCHAR(50) NOT NULL,
    language_id INTEGER NOT NULL REFERENCES language(id)
);

CREATE TABLE session (
    id UUID NOT NULL PRIMARY KEY,
    user_profile_id UUID NOT NULL,
    created_on TIMESTAMP NOT NULL,
    updated_on TIMESTAMP NOT NULL,
    completed_on TIMESTAMP NULL,
    exercise_count TINYINT NOT NULL,
    total_exercise_count TINYINT NOT NULL,
    current_exercise_id INTEGER NULL,
    FOREIGN KEY (user_profile_id) REFERENCES user_profile(id)
);

CREATE TABLE session_user (
    session_id UUID NOT NULL,
    user_id UUID NOT NULL,
    created_on TIMESTAMP NOT NULL,
    PRIMARY KEY(session_id, user_id),
    FOREIGN KEY (session_id) REFERENCES session(id),
    FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE SEQUENCE exercise_type_sequence;
CREATE TABLE exercise_type (
    id TINYINT PRIMARY KEY DEFAULT nextval('exercise_type_sequence'),
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name NVARCHAR(50) NOT NULL,
    description TEXT NULL,
    modality NVARCHAR(20) NOT NULL
);

CREATE SEQUENCE exercise_sequence;
CREATE TABLE exercise (
    id INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('exercise_sequence'),
    type_id TINYINT NOT NULL,
    session_id UUID NOT NULL,
    created_on TIMESTAMP NOT NULL,
    completed_on TIMESTAMP NULL,
    data TEXT NOT NULL,
    result TEXT NULL,
    verdict TEXT NULL,
    FOREIGN KEY (type_id) REFERENCES exercise_type(id),
);

CREATE SEQUENCE meaning_sequence;
CREATE TABLE meaning (
    id INTEGER PRIMARY KEY DEFAULT nextval('meaning_sequence'),
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT NOT NULL
);

CREATE SEQUENCE reading_system_sequence;
CREATE TABLE reading_system (
    id INTEGER PRIMARY KEY DEFAULT nextval('reading_system_sequence'),
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name NVARCHAR(50) NOT NULL UNIQUE,
    type NVARCHAR(20) NULL
);

CREATE TABLE language_exercise_type (
    language_id INTEGER NOT NULL REFERENCES language(id),
    exercise_type_id TINYINT NOT NULL REFERENCES exercise_type(id),
    PRIMARY KEY(language_id, exercise_type_id)
);

CREATE SEQUENCE script_sequence;
CREATE TABLE script (
    id INTEGER PRIMARY KEY DEFAULT nextval('script_sequence'),
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name NVARCHAR(50) NOT NULL,
    iso15924 CHAR(4) NOT NULL UNIQUE
);

CREATE TABLE language_script (
    language_id INTEGER NOT NULL,
    script_id INTEGER NOT NULL,
    is_primary BOOLEAN DEFAULT TRUE,
    PRIMARY KEY (language_id, script_id),
    FOREIGN KEY (language_id) REFERENCES language(id),
    FOREIGN KEY (script_id) REFERENCES script(id)
);

CREATE SEQUENCE lexeme_sequence;
CREATE TABLE lexeme (
    id INTEGER PRIMARY KEY DEFAULT nextval('lexeme_sequence'),
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    text NVARCHAR(100) NOT NULL,
    language_id INTEGER NOT NULL,    
    normalized NVARCHAR(100) NULL
);

CREATE SEQUENCE lexeme_reading_sequence;
CREATE TABLE lexeme_reading (
    id INTEGER PRIMARY KEY DEFAULT nextval('lexeme_reading_sequence'),
    lexeme_id INTEGER NOT NULL REFERENCES lexeme(id),
    system NVARCHAR(50) NOT NULL,
    value NVARCHAR(50) NOT NULL,
    UNIQUE(lexeme_id, system, value)
);

CREATE SEQUENCE grapheme_sequence;
CREATE TABLE grapheme (
    id INTEGER PRIMARY KEY DEFAULT nextval('grapheme_sequence'),
    created_on TIMESTAMP NOT NULL,
    script_id INTEGER NOT NULL,
    symbol NVARCHAR(10) NOT NULL,
    FOREIGN KEY (script_id) REFERENCES script(id)
);

CREATE SEQUENCE grapheme_reading_sequence;
CREATE TABLE grapheme_reading (
    id INTEGER PRIMARY KEY DEFAULT nextval('grapheme_reading_sequence'),
    grapheme_id INTEGER NOT NULL,
    language_id INTEGER NOT NULL,
    system NVARCHAR(20) NOT NULL,
    value NVARCHAR(20) NOT NULL,
    FOREIGN KEY (language_id) REFERENCES language(id),
    FOREIGN KEY (grapheme_id) REFERENCES grapheme(id)
);

CREATE TABLE lexeme_pos (
    lexeme_id INTEGER NOT NULL REFERENCES lexeme(id),
    pos_id INTEGER NOT NULL REFERENCES part_of_speech(id),
    PRIMARY KEY(lexeme_id, pos_id)
);

CREATE TABLE lexeme_meaning (
    lexeme_id INTEGER NOT NULL REFERENCES lexeme(id),
    meaning_id INTEGER NOT NULL REFERENCES meaning(id),
    PRIMARY KEY(lexeme_id, meaning_id)
);

CREATE SEQUENCE classification_system_sequence;
CREATE TABLE classification_system (
    id INTEGER PRIMARY KEY DEFAULT nextval('classification_system_sequence'),
    language_id INTEGER NOT NULL,
    name NVARCHAR(50) NOT NULL
);

CREATE SEQUENCE classification_level_sequence;
CREATE TABLE classification_level (
    id INTEGER PRIMARY KEY DEFAULT nextval('classification_level_sequence'),
    system_id INTEGER NOT NULL,
    name NVARCHAR(50) NOT NULL,
    difficulty TINYINT NOT NULL
);

CREATE TABLE lexeme_classification (
    lexeme_id INTEGER NOT NULL REFERENCES lexeme(id),
    level_id INTEGER NOT NULL REFERENCES classification_level(id),
    PRIMARY KEY(lexeme_id, level_id)
);

CREATE SEQUENCE user_skill_metric_sequence;
CREATE TABLE user_skill_metric (
    id UUID NOT NULL PRIMARY KEY DEFAULT nextval('user_skill_metric_sequence'),
    metric TEXT NOT NULL,
    value DOUBLE NOT NULL,
    updated_on TIMESTAMP NOT NULL,
    FOREIGN KEY (id) REFERENCES user_profile(id)
);

CREATE TABLE user_grapheme_stat (
    id UUID NOT NULL REFERENCES user_profile(id),
    grapheme_id INTEGER NOT NULL,
    times_seen INTEGER NOT NULL DEFAULT 0,
    times_correct INTEGER NOT NULL DEFAULT 0,
    times_pronounced_correct INTEGER NOT NULL DEFAULT 0,
    times_heard_correct INTEGER NOT NULL DEFAULT 0,
    updated_on TIMESTAMP NOT NULL,
    PRIMARY KEY(id, grapheme_id)
);

CREATE TABLE user_lexeme_stat (
    id UUID NOT NULL REFERENCES user_profile(id),
    lexeme_id INTEGER NOT NULL,
    times_seen INTEGER NOT NULL DEFAULT 0,
    times_correct INTEGER NOT NULL DEFAULT 0,
    times_pronounced_correct INTEGER NOT NULL DEFAULT 0,
    times_heard_correct INTEGER NOT NULL DEFAULT 0,
    updated_on TIMESTAMP NOT NULL,
    PRIMARY KEY(id, lexeme_id)
);

CREATE TABLE word_of_the_day (
    id DATE PRIMARY KEY,
    lexeme_id INTEGER NOT NULL REFERENCES lexeme(id),
    language_id INTEGER NOT NULL REFERENCES language(id)
);

CREATE INDEX IX_user_profile_user_id ON user_profile(user_id);
CREATE INDEX IX_exercise_session_id ON exercise(session_id);
CREATE INDEX IX_lexeme_language_id ON lexeme(language_id);
CREATE INDEX IX_user_profile_active_lang ON user_profile(is_active, source_language_id, target_language_id);
CREATE INDEX IX_session_user_user_id ON session_user(user_id);
CREATE INDEX IX_grapheme_script ON grapheme(script_id);
CREATE INDEX IX_language_iso639_1 ON language(iso639_1);