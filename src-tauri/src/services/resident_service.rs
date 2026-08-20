use crate::{
    database::Database,
    services::auth_service::{audit, require, Session},
};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resident {
    pub id: i64,
    pub resident_code: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub suffix: Option<String>,
    pub birth_date: Option<String>,
    pub sex: String,
    pub civil_status: Option<String>,
    pub nationality: String,
    pub contact_number: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub barangay: Option<String>,
    pub municipality: Option<String>,
    pub province: Option<String>,
    pub occupation: Option<String>,
    pub status: String,
    pub registered_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResidentInput {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub suffix: Option<String>,
    pub birth_date: Option<String>,
    pub sex: String,
    pub civil_status: Option<String>,
    pub nationality: Option<String>,
    pub contact_number: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub barangay: Option<String>,
    pub municipality: Option<String>,
    pub province: Option<String>,
    pub occupation: Option<String>,
    pub status: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResidentListQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub sex: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResidentListResult {
    pub records: Vec<Resident>,
    pub total: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdSummary {
    pub household_code: String,
    pub address_line: String,
    pub purok: Option<String>,
    pub relationship_to_head: String,
    pub is_household_head: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSummary {
    pub document_number: String,
    pub document_type: String,
    pub status: String,
    pub issued_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySummary {
    pub action: String,
    pub created_at: String,
    pub user_name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResidentProfile {
    pub resident: Resident,
    pub household: Option<HouseholdSummary>,
    pub documents: Vec<DocumentSummary>,
    pub activity: Vec<ActivitySummary>,
}

fn optional_text(value: &Option<String>) -> Option<String> {
    value.as_ref().and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn is_valid_date(value: &str) -> bool {
    let parts: Vec<_> = value.split('-').collect();
    if parts.len() != 3 { return false; }
    let (Ok(year), Ok(month), Ok(day)) = (parts[0].parse::<i32>(), parts[1].parse::<u32>(), parts[2].parse::<u32>()) else { return false; };
    if !(1900..=2100).contains(&year) || !(1..=12).contains(&month) { return false; }
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) => 29,
        2 => 28,
        _ => return false,
    };
    (1..=max_day).contains(&day)
}

fn validate(input: &ResidentInput) -> Result<(), String> {
    if input.first_name.trim().is_empty() { return Err("First name is required.".into()); }
    if input.last_name.trim().is_empty() { return Err("Last name is required.".into()); }
    if let Some(date) = optional_text(&input.birth_date) { if !is_valid_date(&date) { return Err("Date of birth must be a valid date.".into()); } }
    if !matches!(input.sex.as_str(), "male" | "female" | "other" | "unspecified") { return Err("Select a valid sex.".into()); }
    if !matches!(input.status.as_str(), "active" | "inactive" | "deceased" | "moved_out") { return Err("Select a valid resident status.".into()); }
    if let Some(email) = optional_text(&input.email) { if !email.contains('@') { return Err("Enter a valid email address.".into()); } }
    Ok(())
}

fn read_resident(row: &rusqlite::Row<'_>) -> rusqlite::Result<Resident> {
    Ok(Resident { id: row.get(0)?, resident_code: row.get(1)?, first_name: row.get(2)?, middle_name: row.get(3)?, last_name: row.get(4)?, suffix: row.get(5)?, birth_date: row.get(6)?, sex: row.get(7)?, civil_status: row.get(8)?, nationality: row.get(9)?, contact_number: row.get(10)?, email: row.get(11)?, address: row.get(12)?, barangay: row.get(13)?, municipality: row.get(14)?, province: row.get(15)?, occupation: row.get(16)?, status: row.get(17)?, registered_at: row.get(18)?, created_at: row.get(19)?, updated_at: row.get(20)? })
}

const RESIDENT_COLUMNS: &str = "id,resident_code,first_name,middle_name,last_name,suffix,birth_date,sex,civil_status,nationality,contact_number,email,address,barangay,municipality,province,occupation,status,registered_at,created_at,updated_at";

fn duplicate_exists(connection: &rusqlite::Connection, input: &ResidentInput, except_id: Option<i64>) -> Result<bool, String> {
    let date = optional_text(&input.birth_date).unwrap_or_default();
    connection.query_row("SELECT EXISTS(SELECT 1 FROM residents WHERE lower(first_name)=lower(?1) AND lower(last_name)=lower(?2) AND COALESCE(birth_date,'')=?3 AND (?4 IS NULL OR id != ?4))", params![input.first_name.trim(), input.last_name.trim(), date, except_id], |row| row.get(0)).map_err(|error| error.to_string())
}

fn resident_by_id(connection: &rusqlite::Connection, id: i64) -> Result<Resident, String> {
    connection.query_row(&format!("SELECT {RESIDENT_COLUMNS} FROM residents WHERE id=?1"), [id], read_resident).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_residents(database: State<'_, Database>, session: State<'_, Session>, query: ResidentListQuery) -> Result<ResidentListResult, String> {
    require(&session, "residents.view")?;
    let connection = database.connect().map_err(|error| error.to_string())?;
    let search = query.search.unwrap_or_default().trim().to_string();
    let status = query.status.unwrap_or_else(|| "all".into());
    let sex = query.sex.unwrap_or_else(|| "all".into());
    let filter = format!("%{search}%");
    let total: i64 = connection.query_row("SELECT COUNT(*) FROM residents WHERE (?1='' OR resident_code LIKE ?2 OR first_name LIKE ?2 OR middle_name LIKE ?2 OR last_name LIKE ?2) AND (?3='all' OR status=?3) AND (?4='all' OR sex=?4)", params![search, filter, status, sex], |row| row.get(0)).map_err(|error| error.to_string())?;
    let sort = match query.sort_by.as_deref() { Some("firstName") => "first_name", Some("registeredAt") => "registered_at", Some("residentCode") => "resident_code", _ => "last_name" };
    let direction = if query.sort_direction.as_deref() == Some("asc") { "ASC" } else { "DESC" };
    let page_size = query.page_size.unwrap_or(15).clamp(5, 100) as i64;
    let page = query.page.unwrap_or(1).max(1) as i64;
    let sql = format!("SELECT {RESIDENT_COLUMNS} FROM residents WHERE (?1='' OR resident_code LIKE ?2 OR first_name LIKE ?2 OR middle_name LIKE ?2 OR last_name LIKE ?2) AND (?3='all' OR status=?3) AND (?4='all' OR sex=?4) ORDER BY {sort} {direction}, first_name ASC LIMIT ?5 OFFSET ?6");
    let mut statement = connection.prepare(&sql).map_err(|error| error.to_string())?;
    let records = statement.query_map(params![search, filter, status, sex, page_size, (page - 1) * page_size], read_resident).map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
    Ok(ResidentListResult { records, total })
}

#[tauri::command]
pub fn create_resident(database: State<'_, Database>, session: State<'_, Session>, input: ResidentInput) -> Result<Resident, String> {
    let actor = require(&session, "residents.create")?;
    validate(&input)?;
    let connection = database.connect().map_err(|error| error.to_string())?;
    if duplicate_exists(&connection, &input, None)? { return Err("A resident with the same first name, last name, and date of birth already exists.".into()); }
    let code_seed = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| "System clock is unavailable.")?.as_millis();
    connection.execute("INSERT INTO residents(resident_code,first_name,middle_name,last_name,suffix,birth_date,sex,civil_status,nationality,contact_number,email,address,barangay,municipality,province,occupation,status) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17)", params![format!("TEMP-{code_seed}"), input.first_name.trim(), optional_text(&input.middle_name), input.last_name.trim(), optional_text(&input.suffix), optional_text(&input.birth_date), input.sex, optional_text(&input.civil_status), optional_text(&input.nationality).unwrap_or_else(|| "Filipino".into()), optional_text(&input.contact_number), optional_text(&input.email), optional_text(&input.address), optional_text(&input.barangay), optional_text(&input.municipality), optional_text(&input.province), optional_text(&input.occupation), input.status]).map_err(|error| error.to_string())?;
    let id = connection.last_insert_rowid();
    connection.execute("UPDATE residents SET resident_code=?1 WHERE id=?2", params![format!("RES-{id:06}"), id]).map_err(|error| error.to_string())?;
    audit(&connection, Some(actor.id), "resident_created", "resident", Some(id));
    resident_by_id(&connection, id)
}

#[tauri::command]
pub fn update_resident(database: State<'_, Database>, session: State<'_, Session>, resident_id: i64, input: ResidentInput) -> Result<Resident, String> {
    let actor = require(&session, "residents.edit")?;
    validate(&input)?;
    let connection = database.connect().map_err(|error| error.to_string())?;
    resident_by_id(&connection, resident_id)?;
    if duplicate_exists(&connection, &input, Some(resident_id))? { return Err("A resident with the same first name, last name, and date of birth already exists.".into()); }
    connection.execute("UPDATE residents SET first_name=?1,middle_name=?2,last_name=?3,suffix=?4,birth_date=?5,sex=?6,civil_status=?7,nationality=?8,contact_number=?9,email=?10,address=?11,barangay=?12,municipality=?13,province=?14,occupation=?15,status=?16,updated_at=CURRENT_TIMESTAMP WHERE id=?17", params![input.first_name.trim(), optional_text(&input.middle_name), input.last_name.trim(), optional_text(&input.suffix), optional_text(&input.birth_date), input.sex, optional_text(&input.civil_status), optional_text(&input.nationality).unwrap_or_else(|| "Filipino".into()), optional_text(&input.contact_number), optional_text(&input.email), optional_text(&input.address), optional_text(&input.barangay), optional_text(&input.municipality), optional_text(&input.province), optional_text(&input.occupation), input.status, resident_id]).map_err(|error| error.to_string())?;
    audit(&connection, Some(actor.id), "resident_updated", "resident", Some(resident_id));
    resident_by_id(&connection, resident_id)
}

#[tauri::command]
pub fn archive_resident(database: State<'_, Database>, session: State<'_, Session>, resident_id: i64) -> Result<(), String> {
    let actor = require(&session, "residents.archive")?;
    let connection = database.connect().map_err(|error| error.to_string())?;
    if connection.execute("UPDATE residents SET status='inactive',updated_at=CURRENT_TIMESTAMP WHERE id=?1 AND status!='inactive'", [resident_id]).map_err(|error| error.to_string())? == 0 { return Err("Resident was not found or is already archived.".into()); }
    audit(&connection, Some(actor.id), "resident_archived", "resident", Some(resident_id));
    Ok(())
}

#[tauri::command]
pub fn get_resident_profile(database: State<'_, Database>, session: State<'_, Session>, resident_id: i64) -> Result<ResidentProfile, String> {
    require(&session, "residents.view")?;
    let connection = database.connect().map_err(|error| error.to_string())?;
    let resident = resident_by_id(&connection, resident_id)?;
    let household = connection.query_row("SELECT h.household_code,h.address_line,h.purok,hm.relationship_to_head,hm.is_household_head FROM household_members hm JOIN households h ON h.id=hm.household_id WHERE hm.resident_id=?1", [resident_id], |row| Ok(HouseholdSummary { household_code: row.get(0)?, address_line: row.get(1)?, purok: row.get(2)?, relationship_to_head: row.get(3)?, is_household_head: row.get(4)? })).optional().map_err(|error| error.to_string())?;
    let mut documents_statement = connection.prepare("SELECT d.document_number,dt.name,d.status,d.issued_at FROM documents d JOIN document_types dt ON dt.id=d.document_type_id WHERE d.resident_id=?1 ORDER BY d.issued_at DESC").map_err(|error| error.to_string())?;
    let documents = documents_statement.query_map([resident_id], |row| Ok(DocumentSummary { document_number: row.get(0)?, document_type: row.get(1)?, status: row.get(2)?, issued_at: row.get(3)? })).map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
    let mut activity_statement = connection.prepare("SELECT a.action,a.created_at,u.full_name FROM audit_logs a LEFT JOIN users u ON u.id=a.user_id WHERE a.entity_type='resident' AND a.entity_id=?1 ORDER BY a.created_at DESC LIMIT 20").map_err(|error| error.to_string())?;
    let activity = activity_statement.query_map([resident_id.to_string()], |row| Ok(ActivitySummary { action: row.get(0)?, created_at: row.get(1)?, user_name: row.get(2)? })).map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
    Ok(ResidentProfile { resident, household, documents, activity })
}
