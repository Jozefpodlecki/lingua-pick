CREATE TABLE learning_session (
    id UUID PRIMARY KEY,
    target_language_id VARCHAR NOT NULL,
    content_source VARCHAR NOT NULL CHECK (content_source IN ('authored', 'sample', 'generated')),
    status VARCHAR NOT NULL DEFAULT 'in_progress' CHECK (status IN ('in_progress', 'completed', 'abandoned', 'failed')),
    started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
    completed_at TIMESTAMP WITH TIME ZONE,
    total_exercises USMALLINT NOT NULL CHECK (total_exercises > 0),
    answered_exercises USMALLINT NOT NULL DEFAULT 0,
    correct_exercises USMALLINT NOT NULL DEFAULT 0,
    schema_version USMALLINT NOT NULL DEFAULT 1,
    CHECK (length(trim(target_language_id)) > 0),
    CHECK (answered_exercises <= total_exercises),
    CHECK (correct_exercises <= answered_exercises),
    CHECK (
        (status = 'completed' AND completed_at IS NOT NULL AND answered_exercises = total_exercises)
        OR (status <> 'completed' AND completed_at IS NULL)
    )
);

CREATE TABLE session_exercise (
    session_id UUID NOT NULL REFERENCES learning_session(id),
    position USMALLINT NOT NULL CHECK (position > 0),
    exercise_id VARCHAR NOT NULL,
    exercise_kind VARCHAR NOT NULL,
    payload_json VARCHAR NOT NULL,
    answer_json VARCHAR,
    correct BOOLEAN,
    presented_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
    answered_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (session_id, position),
    UNIQUE (session_id, exercise_id),
    CHECK (length(trim(exercise_id)) > 0),
    CHECK (length(trim(exercise_kind)) > 0),
    CHECK (
        (answered_at IS NULL AND answer_json IS NULL AND correct IS NULL)
        OR (answered_at IS NOT NULL AND answer_json IS NOT NULL)
    )
);

CREATE INDEX learning_session_target_started
    ON learning_session (target_language_id, started_at);

CREATE INDEX session_exercise_session_answered
    ON session_exercise (session_id, answered_at);

CREATE VIEW target_stats AS
WITH exercise_stats AS (
    SELECT
        session_id,
        count(*) FILTER (WHERE answered_at IS NOT NULL)::BIGINT AS answers_recorded,
        count(*) FILTER (WHERE correct IS NOT NULL)::BIGINT AS graded_answers,
        count(*) FILTER (WHERE correct IS TRUE)::BIGINT AS correct_answers
    FROM session_exercise
    GROUP BY session_id
), target_totals AS (
    SELECT
        session.target_language_id,
        count(*)::BIGINT AS sessions_started,
        count(*) FILTER (WHERE session.status = 'completed')::BIGINT AS sessions_completed,
        coalesce(sum(exercise.answers_recorded), 0)::BIGINT AS answers_recorded,
        coalesce(sum(exercise.graded_answers), 0)::BIGINT AS graded_answers,
        coalesce(sum(exercise.correct_answers), 0)::BIGINT AS correct_answers
    FROM learning_session AS session
    LEFT JOIN exercise_stats AS exercise ON exercise.session_id = session.id
    GROUP BY session.target_language_id
)
SELECT
    target_language_id,
    sessions_started,
    sessions_completed,
    answers_recorded,
    graded_answers,
    correct_answers,
    CASE
        WHEN graded_answers = 0 THEN NULL
        ELSE correct_answers::DOUBLE / graded_answers::DOUBLE
    END AS accuracy
FROM target_totals;
