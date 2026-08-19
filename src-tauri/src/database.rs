use rusqlite::{Connection, OpenFlags, Transaction};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

const DATABASE_FILE_NAME: &str = "barangay-management.sqlite3";

struct Migration {
    tag: &'static str,
    sql: &'static str,
}

const MIGRATIONS: &[Migration] = &[Migration {
    tag: "0000_initial_schema",
    sql: include_str!("../../drizzle/0000_initial_schema.sql"),
}];

#[derive(Clone)]
pub struct Database {
    path: PathBuf,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    pub path: String,
    pub migration_count: usize,
}

impl Database {
    pub fn initialize(app: &AppHandle) -> rusqlite::Result<Self> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
        fs::create_dir_all(&data_dir)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;

        Self::open(data_dir.join(DATABASE_FILE_NAME))
    }

    pub fn open(path: PathBuf) -> rusqlite::Result<Self> {
        let database = Self { path };
        let mut connection = database.connect()?;
        database.apply_migrations(&mut connection)?;
        Ok(database)
    }

    pub fn connect(&self) -> rusqlite::Result<Connection> {
        let connection = Connection::open_with_flags(
            &self.path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;
        connection.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
        Ok(connection)
    }

    pub fn status(&self) -> rusqlite::Result<DatabaseStatus> {
        let connection = self.connect()?;
        let migration_count =
            connection.query_row("SELECT COUNT(*) FROM __drizzle_migrations", [], |row| {
                row.get::<_, i64>(0)
            })? as usize;

        Ok(DatabaseStatus {
            path: self.path.display().to_string(),
            migration_count,
        })
    }

    fn apply_migrations(&self, connection: &mut Connection) -> rusqlite::Result<()> {
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS __drizzle_migrations (
                tag TEXT PRIMARY KEY NOT NULL,
                applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );",
        )?;

        let transaction = connection.transaction()?;
        for migration in MIGRATIONS {
            if !migration_applied(&transaction, migration.tag)? {
                transaction.execute_batch(migration.sql)?;
                transaction.execute(
                    "INSERT INTO __drizzle_migrations (tag) VALUES (?1)",
                    [migration.tag],
                )?;
            }
        }
        transaction.commit()
    }
}

fn migration_applied(transaction: &Transaction<'_>, tag: &str) -> rusqlite::Result<bool> {
    transaction.query_row(
        "SELECT EXISTS(SELECT 1 FROM __drizzle_migrations WHERE tag = ?1)",
        [tag],
        |row| row.get(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_database_path() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("bms-database-test-{nonce}.sqlite3"))
    }

    #[test]
    fn migrations_create_required_tables_and_preserve_data_after_reopening() {
        let path = temporary_database_path();
        let database = Database::open(path.clone()).expect("database should initialize");
        let connection = database.connect().expect("database should connect");

        for table in [
            "barangay_profile",
            "users",
            "roles",
            "permissions",
            "role_permissions",
            "official_positions",
            "residents",
            "households",
            "household_members",
            "document_types",
            "document_requests",
            "documents",
            "audit_logs",
        ] {
            let exists: bool = connection
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1)",
                    [table],
                    |row| row.get(0),
                )
                .expect("table query should succeed");
            assert!(exists, "{table} should be created by migrations");
        }

        connection
            .execute(
                "INSERT INTO roles (code, name) VALUES ('test-role', 'Test role')",
                [],
            )
            .expect("test row should be inserted");
        drop(connection);
        drop(database);

        let reopened = Database::open(path.clone()).expect("database should reopen");
        let reopened_connection = reopened
            .connect()
            .expect("reopened database should connect");
        let role_count: i64 = reopened_connection
            .query_row(
                "SELECT COUNT(*) FROM roles WHERE code = 'test-role'",
                [],
                |row| row.get(0),
            )
            .expect("persisted row should be queryable");
        assert_eq!(role_count, 1);
        assert_eq!(
            reopened
                .status()
                .expect("status should load")
                .migration_count,
            MIGRATIONS.len()
        );

        drop(reopened_connection);
        let _ = fs::remove_file(path);
    }
}
