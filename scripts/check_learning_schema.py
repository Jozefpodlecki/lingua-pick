"""Validate the experimental harness migrations using a locally built DuckDB DLL."""
import ctypes
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[1]
library = ctypes.CDLL(str(Path(sys.argv[1]).resolve()))
handle = ctypes.c_void_p
library.duckdb_open.argtypes = [ctypes.c_char_p, ctypes.POINTER(handle)]
library.duckdb_connect.argtypes = [handle, ctypes.POINTER(handle)]
library.duckdb_query.argtypes = [handle, ctypes.c_char_p, handle]
library.duckdb_result_error.argtypes = [handle]
library.duckdb_result_error.restype = ctypes.c_char_p
library.duckdb_destroy_result.argtypes = [handle]
library.duckdb_disconnect.argtypes = [ctypes.POINTER(handle)]
library.duckdb_close.argtypes = [ctypes.POINTER(handle)]
database, connection = handle(), handle()
assert library.duckdb_open(None, ctypes.byref(database)) == 0
assert library.duckdb_connect(database, ctypes.byref(connection)) == 0

def query(sql):
    result = ctypes.create_string_buffer(512)
    try:
        if library.duckdb_query(connection, sql.encode(), result):
            raise RuntimeError(library.duckdb_result_error(result).decode())
    finally:
        library.duckdb_destroy_result(result)

def reject(sql):
    try:
        query(sql)
    except RuntimeError:
        return
    raise AssertionError('Expected constraint rejection: ' + sql)

try:
    for migration in sorted((root / 'crates/lingua-cli/src/migration').glob('*.sql')):
        query('BEGIN')
        sql = migration.read_text(encoding='utf-8')
        if sql.strip():
            query(sql)
        query('COMMIT')
        print('Applied', migration.name)
    query("SELECT CASE WHEN count(*) = 3 THEN 1 ELSE error('Expected 3 learning stages') END FROM learning_stage")
    query("SELECT CASE WHEN count(*) = 100 AND count(*) FILTER (WHERE language_id = 'pt-BR') = 100 THEN 1 ELSE error('Expected 100 pt-BR concepts and no other target concepts') END FROM concept")
    query("SELECT CASE WHEN count(*) = 35 THEN 1 ELSE error('Expected 35 exercise definitions') END FROM exercise_definition")
    query("SELECT CASE WHEN count(*) = 35 THEN 1 ELSE error('Definition IDs must be UUIDv7') END FROM exercise_definition WHERE uuid_extract_version(id) = 7")
    query("SELECT CASE WHEN count(*) = 0 THEN 1 ELSE error('Unknown definition category') END FROM exercise_definition d LEFT JOIN exercise_category c ON c.name = d.category WHERE c.name IS NULL")
    query("SELECT CASE WHEN count(*) = 0 THEN 1 ELSE error('Invalid schema contract') END FROM exercise_definition WHERE json_extract_string(schema, '$.type') <> 'object' OR json_extract_string(verdict_schema, '$.type') <> 'object' OR verdict_schema IS NULL OR (kind <> 'dialogue' AND (answer_schema IS NULL OR json_extract_string(answer_schema, '$.type') <> 'object'))")
    query("SELECT CASE WHEN count(*) = 1 THEN 1 ELSE error('Dialogue must be ungraded') END FROM exercise_definition WHERE kind = 'dialogue' AND answer_schema IS NULL AND json_extract(verdict_schema, '$.properties.concept_results') IS NULL")
    query("INSERT INTO concept (language_id, skill_id, code, name, description) VALUES ('pt-BR', 'word_recognition', 'test.default', 'Default identity', 'Test database-generated UUID')")
    query("SELECT CASE WHEN uuid_extract_version(id) = 7 THEN 1 ELSE error('Default ID must be UUIDv7') END FROM concept WHERE code = 'test.default'")
    query("INSERT INTO concept VALUES ('20000000-0000-4000-8000-000000000001', 'pt-BR', 'word_recognition', 'test.recognition', 'Test recognition', 'Test-only learning concept')")
    query("INSERT INTO language_concept VALUES ('pt-BR', '20000000-0000-4000-8000-000000000001')")
    query("INSERT INTO user (id, source_language_id, username, password_hash) VALUES ('30000000-0000-4000-8000-000000000001', 'en-GB', 'schema-test', 'unused')")
    query("INSERT INTO user_stats (user_id, target_language_id) VALUES ('30000000-0000-4000-8000-000000000001', 'pt-BR')")
    query("SELECT CASE WHEN count(*) = 1 AND min(d.kind) = 'match_words' THEN 1 ELSE error('Words stage must allow matching only') END FROM learning_stage_exercise x JOIN exercise_definition d ON d.id = x.definition_id WHERE x.stage_code = 'words'")
    query("INSERT INTO session (id, user_id, target_language_id, exercise_count, max_exercise_count) VALUES ('40000000-0000-4000-8000-000000000001', '30000000-0000-4000-8000-000000000001', 'pt-BR', 0, 10)")
    query("INSERT INTO exercise (id, definition_id, session_id, kind, payload) VALUES ('50000000-0000-4000-8000-000000000001', (SELECT id FROM exercise_definition WHERE kind = 'match_words'), '40000000-0000-4000-8000-000000000001', 'match_words', '{}')")
    base = "INSERT INTO learning_evidence (id, user_id, target_language_id, exercise_id, concept_id, event_key, evidence_mode, correct, evaluator, policy_version) VALUES ('60000000-0000-4000-8000-000000000001','30000000-0000-4000-8000-000000000001','pt-BR','50000000-0000-4000-8000-000000000001','20000000-0000-4000-8000-000000000001','answer','recognition',TRUE,'schema-test','prototype-v1')"
    query(base)
    reject(base)
    reject(base.replace('60000000-0000-4000-8000-000000000001', '60000000-0000-4000-8000-000000000002').replace("'recognition',TRUE", "'exposure',TRUE"))
    query("SELECT CASE WHEN assessed_attempts = 1 AND accuracy = 1 THEN 1 ELSE error('Incorrect evidence aggregate') END FROM learner_concept_evidence")
    print('Seed counts, evidence aggregation, duplicate protection and ungraded constraints passed.')
finally:
    library.duckdb_disconnect(ctypes.byref(connection))
    library.duckdb_close(ctypes.byref(database))
