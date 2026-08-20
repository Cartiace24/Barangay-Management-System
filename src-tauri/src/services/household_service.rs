use crate::{database::Database, services::auth_service::{audit, require, Session}};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdRecord { pub id: i64, pub household_code: String, pub address_line: String, pub purok: Option<String>, pub status: String, pub registered_at: String, pub household_head: Option<String>, pub member_count: i64 }
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdInput { pub address_line: String, pub purok: Option<String>, pub status: String, pub head_resident_id: Option<i64> }
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdListQuery { pub search: Option<String>, pub status: Option<String>, pub page: Option<u32>, pub page_size: Option<u32> }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdListResult { pub records: Vec<HouseholdRecord>, pub total: i64 }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResidentOption { pub id: i64, pub resident_code: String, pub full_name: String }
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberInput { pub resident_id: i64, pub relationship_to_head: String }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdMember { pub resident_id: i64, pub resident_code: String, pub full_name: String, pub relationship_to_head: String, pub is_household_head: bool, pub status: String }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdActivity { pub action: String, pub created_at: String, pub user_name: Option<String> }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdProfile { pub household: HouseholdRecord, pub members: Vec<HouseholdMember>, pub activity: Vec<HouseholdActivity> }

const HOUSEHOLD_SELECT: &str = "h.id,h.household_code,h.address_line,h.purok,h.status,h.registered_at,(SELECT trim(r.first_name || ' ' || coalesce(r.middle_name || ' ', '') || r.last_name || coalesce(' ' || r.suffix, '')) FROM household_members hm JOIN residents r ON r.id=hm.resident_id WHERE hm.household_id=h.id AND hm.is_household_head=1),(SELECT COUNT(*) FROM household_members hm WHERE hm.household_id=h.id)";
fn read_household(row: &rusqlite::Row<'_>) -> rusqlite::Result<HouseholdRecord> { Ok(HouseholdRecord { id: row.get(0)?, household_code: row.get(1)?, address_line: row.get(2)?, purok: row.get(3)?, status: row.get(4)?, registered_at: row.get(5)?, household_head: row.get(6)?, member_count: row.get(7)? }) }
fn optional_text(value: &Option<String>) -> Option<String> { value.as_ref().and_then(|value| (!value.trim().is_empty()).then(|| value.trim().to_string())) }
fn validate(input: &HouseholdInput, requires_head: bool) -> Result<(), String> { if input.address_line.trim().is_empty() { return Err("Household address is required.".into()); } if !matches!(input.status.as_str(), "active" | "inactive") { return Err("Select a valid household status.".into()); } if requires_head && input.head_resident_id.is_none() { return Err("Select the household head.".into()); } Ok(()) }
fn household_by_id(connection: &rusqlite::Connection, id: i64) -> Result<HouseholdRecord, String> { connection.query_row(&format!("SELECT {HOUSEHOLD_SELECT} FROM households h WHERE h.id=?1"), [id], read_household).map_err(|error| error.to_string()) }
fn active_resident(connection: &rusqlite::Connection, resident_id: i64) -> Result<(), String> { let found: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM residents WHERE id=?1 AND status='active')", [resident_id], |row| row.get(0)).map_err(|error| error.to_string())?; found.then_some(()).ok_or_else(|| "Select an active resident.".into()) }

#[tauri::command]
pub fn list_households(database: State<'_, Database>, session: State<'_, Session>, query: HouseholdListQuery) -> Result<HouseholdListResult, String> {
    require(&session, "households.view")?;
    let connection = database.connect().map_err(|error| error.to_string())?;
    let search = query.search.unwrap_or_default().trim().to_string(); let filter = format!("%{search}%"); let status = query.status.unwrap_or_else(|| "all".into());
    let where_clause = "(?1='' OR h.household_code LIKE ?2 OR h.address_line LIKE ?2 OR EXISTS(SELECT 1 FROM household_members hm JOIN residents r ON r.id=hm.resident_id WHERE hm.household_id=h.id AND hm.is_household_head=1 AND (r.first_name LIKE ?2 OR r.last_name LIKE ?2))) AND (?3='all' OR h.status=?3)";
    let total: i64 = connection.query_row(&format!("SELECT COUNT(*) FROM households h WHERE {where_clause}"), params![search, filter, status], |row| row.get(0)).map_err(|error| error.to_string())?;
    let page_size = query.page_size.unwrap_or(15).clamp(5, 100) as i64; let page = query.page.unwrap_or(1).max(1) as i64;
    let sql = format!("SELECT {HOUSEHOLD_SELECT} FROM households h WHERE {where_clause} ORDER BY h.registered_at DESC, h.id DESC LIMIT ?4 OFFSET ?5");
    let mut statement = connection.prepare(&sql).map_err(|error| error.to_string())?;
    let records = statement.query_map(params![search, filter, status, page_size, (page - 1) * page_size], read_household).map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
    Ok(HouseholdListResult { records, total })
}

#[tauri::command]
pub fn list_available_residents(database: State<'_, Database>, session: State<'_, Session>, search: Option<String>) -> Result<Vec<ResidentOption>, String> {
    require(&session, "households.view")?;
    let connection = database.connect().map_err(|error| error.to_string())?; let search = search.unwrap_or_default().trim().to_string(); let filter = format!("%{search}%");
    let mut statement = connection.prepare("SELECT r.id,r.resident_code,trim(r.first_name || ' ' || coalesce(r.middle_name || ' ', '') || r.last_name || coalesce(' ' || r.suffix, '')) FROM residents r LEFT JOIN household_members hm ON hm.resident_id=r.id WHERE r.status='active' AND hm.resident_id IS NULL AND (?1='' OR r.resident_code LIKE ?2 OR r.first_name LIKE ?2 OR r.last_name LIKE ?2) ORDER BY r.last_name,r.first_name LIMIT 100").map_err(|error| error.to_string())?;
    let residents = statement.query_map(params![search, filter], |row| Ok(ResidentOption { id: row.get(0)?, resident_code: row.get(1)?, full_name: row.get(2)? })).map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
    Ok(residents)
}

#[tauri::command]
pub fn create_household(database: State<'_, Database>, session: State<'_, Session>, input: HouseholdInput) -> Result<HouseholdRecord, String> {
    let actor = require(&session, "households.create")?; validate(&input, true)?; let connection = database.connect().map_err(|error| error.to_string())?; let head_id = input.head_resident_id.ok_or("Select the household head.")?; active_resident(&connection, head_id)?;
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| "System clock is unavailable.")?.as_millis();
    connection.execute("INSERT INTO households(household_code,address_line,purok,status) VALUES(?1,?2,?3,?4)", params![format!("TEMP-{seed}"), input.address_line.trim(), optional_text(&input.purok), input.status]).map_err(|error| error.to_string())?; let id = connection.last_insert_rowid();
    connection.execute("UPDATE households SET household_code=?1 WHERE id=?2", params![format!("HH-{id:06}"), id]).map_err(|error| error.to_string())?;
    connection.execute("INSERT INTO household_members(household_id,resident_id,relationship_to_head,is_household_head) VALUES(?1,?2,'Household Head',1)", params![id, head_id]).map_err(|_| "This resident is already assigned to another household.".to_string())?;
    audit(&connection, Some(actor.id), "household_created", "household", Some(id)); audit(&connection, Some(actor.id), "household_head_assigned", "household", Some(id)); household_by_id(&connection, id)
}

#[tauri::command]
pub fn update_household(database: State<'_, Database>, session: State<'_, Session>, household_id: i64, input: HouseholdInput) -> Result<HouseholdRecord, String> {
    let actor = require(&session, "households.edit")?; validate(&input, false)?; let connection = database.connect().map_err(|error| error.to_string())?; household_by_id(&connection, household_id)?;
    connection.execute("UPDATE households SET address_line=?1,purok=?2,status=?3,updated_at=CURRENT_TIMESTAMP WHERE id=?4", params![input.address_line.trim(), optional_text(&input.purok), input.status, household_id]).map_err(|error| error.to_string())?; audit(&connection, Some(actor.id), "household_updated", "household", Some(household_id)); household_by_id(&connection, household_id)
}

#[tauri::command]
pub fn add_household_member(database: State<'_, Database>, session: State<'_, Session>, household_id: i64, input: MemberInput) -> Result<(), String> {
    let actor = require(&session, "households.edit")?; if input.relationship_to_head.trim().is_empty() { return Err("Relationship to household head is required.".into()); } let connection = database.connect().map_err(|error| error.to_string())?; household_by_id(&connection, household_id)?; active_resident(&connection, input.resident_id)?;
    connection.execute("INSERT INTO household_members(household_id,resident_id,relationship_to_head,is_household_head) VALUES(?1,?2,?3,0)", params![household_id, input.resident_id, input.relationship_to_head.trim()]).map_err(|_| "This resident is already assigned to a household.".to_string())?; audit(&connection, Some(actor.id), "household_member_added", "household", Some(household_id)); Ok(())
}

#[tauri::command]
pub fn remove_household_member(database: State<'_, Database>, session: State<'_, Session>, household_id: i64, resident_id: i64) -> Result<(), String> {
    let actor = require(&session, "households.edit")?; let connection = database.connect().map_err(|error| error.to_string())?;
    let is_head: Option<bool> = connection.query_row("SELECT is_household_head FROM household_members WHERE household_id=?1 AND resident_id=?2", params![household_id, resident_id], |row| row.get(0)).optional().map_err(|error| error.to_string())?; let Some(is_head) = is_head else { return Err("Household member was not found.".into()); }; if is_head { return Err("Assign another household head before removing the current head.".into()); }
    connection.execute("DELETE FROM household_members WHERE household_id=?1 AND resident_id=?2", params![household_id, resident_id]).map_err(|error| error.to_string())?; audit(&connection, Some(actor.id), "household_member_removed", "household", Some(household_id)); Ok(())
}

#[tauri::command]
pub fn change_household_head(database: State<'_, Database>, session: State<'_, Session>, household_id: i64, resident_id: i64) -> Result<(), String> {
    let actor = require(&session, "households.edit")?; let mut connection = database.connect().map_err(|error| error.to_string())?; let transaction = connection.transaction().map_err(|error| error.to_string())?;
    let exists: bool = transaction.query_row("SELECT EXISTS(SELECT 1 FROM household_members WHERE household_id=?1 AND resident_id=?2)", params![household_id, resident_id], |row| row.get(0)).map_err(|error| error.to_string())?; if !exists { return Err("The selected resident is not a member of this household.".into()); }
    transaction.execute("UPDATE household_members SET is_household_head=0 WHERE household_id=?1", [household_id]).map_err(|error| error.to_string())?; transaction.execute("UPDATE household_members SET is_household_head=1,relationship_to_head='Household Head' WHERE household_id=?1 AND resident_id=?2", params![household_id, resident_id]).map_err(|error| error.to_string())?; transaction.commit().map_err(|error| error.to_string())?; audit(&connection, Some(actor.id), "household_head_changed", "household", Some(household_id)); Ok(())
}

#[tauri::command]
pub fn get_household_profile(database: State<'_, Database>, session: State<'_, Session>, household_id: i64) -> Result<HouseholdProfile, String> {
    require(&session, "households.view")?; let connection = database.connect().map_err(|error| error.to_string())?; let household = household_by_id(&connection, household_id)?;
    let mut members_statement = connection.prepare("SELECT r.id,r.resident_code,trim(r.first_name || ' ' || coalesce(r.middle_name || ' ', '') || r.last_name || coalesce(' ' || r.suffix, '')),hm.relationship_to_head,hm.is_household_head,r.status FROM household_members hm JOIN residents r ON r.id=hm.resident_id WHERE hm.household_id=?1 ORDER BY hm.is_household_head DESC,r.last_name,r.first_name").map_err(|error| error.to_string())?;
    let members = members_statement.query_map([household_id], |row| Ok(HouseholdMember { resident_id: row.get(0)?, resident_code: row.get(1)?, full_name: row.get(2)?, relationship_to_head: row.get(3)?, is_household_head: row.get(4)?, status: row.get(5)? })).map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
    let mut activity_statement = connection.prepare("SELECT a.action,a.created_at,u.full_name FROM audit_logs a LEFT JOIN users u ON u.id=a.user_id WHERE a.entity_type='household' AND a.entity_id=?1 ORDER BY a.created_at DESC LIMIT 20").map_err(|error| error.to_string())?;
    let activity = activity_statement.query_map([household_id.to_string()], |row| Ok(HouseholdActivity { action: row.get(0)?, created_at: row.get(1)?, user_name: row.get(2)? })).map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
    Ok(HouseholdProfile { household, members, activity })
}
