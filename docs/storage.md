# Native storage

## Status

The `lingua-store` crate and its first DuckDB migration are implemented. The Tauri application opens the database and applies migrations during setup. Session commands and the Yew session flow do not write to the store yet, so history and statistics screens still have no persisted outcomes.

Browser-only runs cannot use DuckDB. Their future progress persistence remains a browser-storage adapter with the same domain meaning, not direct access to the native database.

## Dependency choice

The native store uses the official Rust `duckdb` crate with the `bundled`, `chrono`, and `uuid` features. The bundled feature compiles the matching DuckDB source into the desktop application and avoids a machine-wide DuckDB installation. The version requirement is pinned to the current DuckDB 1.5.6 binding series so a patch update cannot silently switch the embedded database engine series. See the [duckdb-rs project](https://github.com/duckdb/duckdb-rs) and [Rust API](https://docs.rs/duckdb/latest/duckdb/).

One DuckDB connection is held behind a mutex. This matches the crate's `Send` but non-`Sync` connection contract and gives Tauri one serialized persistence boundary. Add pooling only after measured command concurrency requires it.

## Database lifecycle

The Tauri application resolves its platform-specific application data directory and uses `lingua-pick.duckdb` inside it. The store performs startup in this order:

1. Inspect whether the database file already exists.
2. Create its parent directory when needed.
3. Open the file, which creates a new DuckDB database when absent.
4. Ensure the migration ledger exists.
5. Verify every recorded migration name and SHA-256 checksum.
6. Reject migration versions newer than the application knows.
7. Apply each pending embedded SQL migration in its own transaction.
8. Expose the initialized store as managed Tauri state.

DuckDB transactions are atomic and roll back when migration execution or ledger insertion fails. The application must fail setup instead of continuing with a partially upgraded schema.

## Initial schema

### `schema_migration`

Records the integer version, stable name, source checksum, and application timestamp for every applied migration. Applied migration files are immutable. Schema changes require a new migration with the next integer version.

### `learning_session`

Stores one activity run:

- UUIDv7 session identity.
- Exact target identifier, including regional variety.
- Content source: authored, sample, or generated.
- Lifecycle status: in progress, completed, abandoned, or failed.
- Start, update, and optional completion timestamps.
- Total, answered, and correct exercise counts.
- Stored-record schema version.

Constraints prevent blank targets, zero-length sessions, impossible counts, or a completed session without all exercise outcomes.

### `session_exercise`

Stores the ordered exercises belonging to a session:

- Parent session and one-based position.
- Stable exercise identifier and exercise kind.
- Versioned exercise payload serialized as JSON text.
- Optional submitted answer serialized as JSON text.
- Optional correctness, allowing future ungraded dialogue and practice events.
- Presentation and answer timestamps.

Position and exercise identity are unique within a session. Payloads are serialized before the insertion transaction starts. Recording an outcome recalculates the session counters from exercise rows, making repeated writes idempotent with respect to aggregate counts.

### `target_stats`

This view derives sessions started, sessions completed, answers recorded, graded answers, correct answers, and accuracy for each exact target identifier. Accuracy divides correct answers by graded answers, so ungraded dialogue and practice outcomes do not lower it. Accuracy is absent when no graded answers exist. Statistics are not maintained as a second mutable table because session history is the canonical evidence and duplicated aggregates could drift.

## Repository boundaries

The session repository creates a session and its complete ordered exercise batch atomically, records outcomes, marks fully answered sessions complete, reads stored sessions and exercise rows, and lists recent sessions for one exact target. The statistics repository reads the target-level view.

Repositories accept typed records and bind SQL parameters. They do not accept arbitrary SQL, depend on Tauri, or expose the DuckDB connection. Tauri commands added later should perform database work outside async executor threads when operations can block.

## Migration rules

- Keep migration versions contiguous and append-only.
- Never edit or rename an applied migration; its checksum is part of the database contract.
- Put data repair and backfill in explicit migrations.
- Add constraints with the schema when DuckDB supports them.
- Do not rely on cascading foreign-key deletes; DuckDB does not support `ON DELETE CASCADE`.
- Test opening a new database, reopening an up-to-date database, and upgrading from every supported schema version.

## Next integration

Add typed Tauri commands that start a persisted session, record each accepted outcome, complete or abandon a session, and query history/statistics. The web application should call those commands only in Tauri mode. Browser-only runs need a separate adapter before they can claim durable session history.
