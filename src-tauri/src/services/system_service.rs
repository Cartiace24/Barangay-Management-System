use crate::database::{Database, DatabaseStatus};
use tauri::State;

#[tauri::command]
pub fn get_database_status(database: State<'_, Database>) -> Result<DatabaseStatus, String> {
    database.status().map_err(|error| error.to_string())
}
