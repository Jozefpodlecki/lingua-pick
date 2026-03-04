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
    iso2 NVARCHAR(4) NULL,
    iso3 NVARCHAR(4) NOT NULL
);

CREATE TABLE session (
    id UUID NOT NULL PRIMARY KEY,
    created_on TIMESTAMP NOT NULL,
    updated_on TIMESTAMP NOT NULL,
    completed_on TIMESTAMP NULL,
    source_language_id UUID NOT NULL,
    target_language_id UUID NOT NULL,
    exercise_count TINYINT NOT NULL,
    current_exercise_id UUID NULL,
    FOREIGN KEY (source_language_id) REFERENCES language(id),
    FOREIGN KEY (target_language_id) REFERENCES language(id)
);

CREATE TABLE session_user (
    session_id UUID NOT NULL,
    user_id UUID NOT NULL
);

CREATE TABLE exercise (
    id UUID NOT NULL PRIMARY KEY,
    kind SMALLINT NOT NULL,
    created_on TIMESTAMP NOT NULL,
    completed_on TIMESTAMP NULL,
    session_id UUID NOT NULL,
    data TEXT NOT NULL
);

-- CREATE INDEX IX_user_profile_user_id ON user_profile(user_id);
-- CREATE INDEX IX_session_source_language ON session(source_language_id);
-- CREATE INDEX IX_session_target_language ON session(target_language_id);
-- CREATE INDEX IX_exercise_session_id ON exercise(session_id);