use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::State;

use crate::database::Database;

const MAX_IMAGE_BYTES: usize = 2 * 1024 * 1024;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BarangayProfile {
    pub id: i64,
    pub name: String,
    pub municipality: String,
    pub province: String,
    pub address: Option<String>,
    pub contact_number: Option<String>,
    pub email: Option<String>,
    pub logo_path: Option<String>,
    pub authorized_signatory: Option<String>,
    pub signatory_position: Option<String>,
    pub signature_path: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInput {
    pub name: String,
    pub municipality: String,
    pub province: String,
    pub address: Option<String>,
    pub contact_number: Option<String>,
    pub email: Option<String>,
    pub authorized_signatory: Option<String>,
    pub signatory_position: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfficialInput {
    pub title: String,
    pub full_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupInput {
    pub profile: ProfileInput,
    pub officials: Vec<OfficialInput>,
    pub administrator_name: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageInput {
    pub file_name: String,
    pub mime_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

fn required(value: &str, field: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{field} is required."))
    } else {
        Ok(())
    }
}

fn profile_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<BarangayProfile> {
    Ok(BarangayProfile {
        id: row.get(0)?,
        name: row.get(1)?,
        municipality: row.get(2)?,
        province: row.get(3)?,
        address: row.get(4)?,
        contact_number: row.get(5)?,
        email: row.get(6)?,
        logo_path: row.get(7)?,
        authorized_signatory: row.get(8)?,
        signatory_position: row.get(9)?,
        signature_path: row.get(10)?,
    })
}

fn current_profile(database: &Database) -> Result<Option<BarangayProfile>, String> {
    let connection = database.connect().map_err(|error| error.to_string())?;
    connection.query_row("SELECT id, name, municipality, province, address, contact_number, email, logo_path, authorized_signatory, signatory_position, signature_path FROM barangay_profile ORDER BY id LIMIT 1", [], profile_from_row).optional().map_err(|error| error.to_string())
}

fn hash_password(password: &str) -> Result<String, String> {
    if password.len() < 10 {
        return Err("Password must contain at least 10 characters.".into());
    }
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| error.to_string())
}

fn save_image(database: &Database, image: ImageInput, kind: &str) -> Result<String, String> {
    if image.bytes.is_empty() || image.bytes.len() > MAX_IMAGE_BYTES {
        return Err("Image must be between 1 byte and 2 MB.".into());
    }
    let extension = match image.mime_type.as_str() {
        "image/png" => "png",
        "image/jpeg" | "image/jpg" => "jpg",
        "image/webp" => "webp",
        _ => return Err("Only PNG, JPG/JPEG, and WebP images are allowed.".into()),
    };
    if !image
        .file_name
        .to_ascii_lowercase()
        .ends_with(&format!(".{extension}"))
        && !(extension == "jpg" && image.file_name.to_ascii_lowercase().ends_with(".jpeg"))
    {
        return Err("File extension does not match the image type.".into());
    }
    let directory = database.data_dir().join("branding");
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    let relative = format!("branding/{kind}.{extension}");
    fs::write(database.data_dir().join(&relative), image.bytes)
        .map_err(|error| error.to_string())?;
    Ok(relative)
}

fn delete_image(database: &Database, path: &Option<String>) {
    if let Some(relative_path) = path {
        let _ = fs::remove_file(database.data_dir().join(relative_path));
    }
}

#[tauri::command]
pub fn get_barangay_profile(
    database: State<'_, Database>,
) -> Result<Option<BarangayProfile>, String> {
    current_profile(&database)
}

#[tauri::command]
pub fn authenticate_user(database: State<'_, Database>, input: LoginInput) -> Result<bool, String> {
    required(&input.username, "Username")?;
    required(&input.password, "Password")?;
    let connection = database.connect().map_err(|error| error.to_string())?;
    let password_hash: Option<String> = connection
        .query_row(
            "SELECT password_hash FROM users WHERE username = ?1 AND status = 'active'",
            [input.username.trim()],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| error.to_string())?;
    let Some(password_hash) = password_hash else {
        return Ok(false);
    };
    let parsed = PasswordHash::new(&password_hash).map_err(|error| error.to_string())?;
    let valid = Argon2::default()
        .verify_password(input.password.as_bytes(), &parsed)
        .is_ok();
    if valid {
        connection
            .execute(
                "UPDATE users SET last_login_at=CURRENT_TIMESTAMP WHERE username=?1",
                [input.username.trim()],
            )
            .map_err(|error| error.to_string())?;
    }
    Ok(valid)
}

#[tauri::command]
pub fn complete_first_time_setup(
    database: State<'_, Database>,
    input: SetupInput,
) -> Result<BarangayProfile, String> {
    crate::services::auth_service::seed_defaults(&database)?;
    required(&input.profile.name, "Barangay name")?;
    required(&input.profile.municipality, "Municipality / City")?;
    required(&input.profile.province, "Province")?;
    required(&input.administrator_name, "Administrator name")?;
    required(&input.username, "Username")?;
    if current_profile(&database)?.is_some() {
        return Err("First-time setup has already been completed.".into());
    }
    if input.officials.len() != 4
        || input.officials.iter().any(|official| {
            official.title.trim().is_empty() || official.full_name.trim().is_empty()
        })
    {
        return Err("Complete all four initial officials.".into());
    }
    let password_hash = hash_password(&input.password)?;
    let mut connection = database.connect().map_err(|error| error.to_string())?;
    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    let role_id: i64 = transaction
        .query_row(
            "SELECT id FROM roles WHERE code='administrator'",
            [],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    for (index, official) in input.officials.iter().enumerate() {
        transaction
            .execute(
                "INSERT OR IGNORE INTO official_positions (title, display_order) VALUES (?1, ?2)",
                params![official.title.trim(), index as i64],
            )
            .map_err(|error| error.to_string())?;
    }
    let captain = &input.officials[0];
    transaction.execute("INSERT INTO barangay_profile (name, municipality, province, address, contact_number, email, authorized_signatory, signatory_position) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", params![input.profile.name.trim(), input.profile.municipality.trim(), input.profile.province.trim(), input.profile.address, input.profile.contact_number, input.profile.email, captain.full_name.trim(), captain.title.trim()]).map_err(|error| error.to_string())?;
    transaction.execute("INSERT INTO users (username, password_hash, full_name, role_id) VALUES (?1, ?2, ?3, ?4)", params![input.username.trim(), password_hash, input.administrator_name.trim(), role_id]).map_err(|error| error.to_string())?;
    transaction.execute("INSERT INTO audit_logs (action, entity_type, entity_id, details_json) VALUES ('setup_completed', 'barangay_profile', '1', '{\"source\":\"first_time_setup\"}')", []).map_err(|error| error.to_string())?;
    transaction.commit().map_err(|error| error.to_string())?;
    current_profile(&database)?.ok_or_else(|| "Profile was not saved.".into())
}

#[tauri::command]
pub fn update_barangay_profile(
    database: State<'_, Database>,
    input: ProfileInput,
) -> Result<BarangayProfile, String> {
    required(&input.name, "Barangay name")?;
    required(&input.municipality, "Municipality / City")?;
    required(&input.province, "Province")?;
    let profile = current_profile(&database)?
        .ok_or_else(|| "Barangay profile has not been configured.".to_string())?;
    let connection = database.connect().map_err(|error| error.to_string())?;
    connection.execute("UPDATE barangay_profile SET name=?1, municipality=?2, province=?3, address=?4, contact_number=?5, email=?6, authorized_signatory=?7, signatory_position=?8, updated_at=CURRENT_TIMESTAMP WHERE id=?9", params![input.name.trim(), input.municipality.trim(), input.province.trim(), input.address, input.contact_number, input.email, input.authorized_signatory, input.signatory_position, profile.id]).map_err(|error| error.to_string())?;
    connection.execute("INSERT INTO audit_logs (action, entity_type, entity_id, details_json) VALUES ('profile_updated', 'barangay_profile', ?1, '{\"source\":\"settings\"}')", [profile.id]).map_err(|error| error.to_string())?;
    current_profile(&database)?.ok_or_else(|| "Profile was not saved.".into())
}

#[tauri::command]
pub fn update_branding_image(
    database: State<'_, Database>,
    kind: String,
    image: ImageInput,
) -> Result<BarangayProfile, String> {
    if kind != "logo" && kind != "signature" {
        return Err("Unknown branding image type.".into());
    }
    let profile = current_profile(&database)?
        .ok_or_else(|| "Barangay profile has not been configured.".to_string())?;
    let new_path = save_image(&database, image, &kind)?;
    let old_path = if kind == "logo" {
        profile.logo_path.clone()
    } else {
        profile.signature_path.clone()
    };
    let column = if kind == "logo" {
        "logo_path"
    } else {
        "signature_path"
    };
    let connection = database.connect().map_err(|error| error.to_string())?;
    connection
        .execute(
            &format!(
                "UPDATE barangay_profile SET {column}=?1, updated_at=CURRENT_TIMESTAMP WHERE id=?2"
            ),
            params![new_path, profile.id],
        )
        .map_err(|error| error.to_string())?;
    delete_image(&database, &old_path);
    current_profile(&database)?.ok_or_else(|| "Branding image was not saved.".into())
}

#[tauri::command]
pub fn remove_branding_image(
    database: State<'_, Database>,
    kind: String,
) -> Result<BarangayProfile, String> {
    if kind != "logo" && kind != "signature" {
        return Err("Unknown branding image type.".into());
    }
    let profile = current_profile(&database)?
        .ok_or_else(|| "Barangay profile has not been configured.".to_string())?;
    let previous_path = if kind == "logo" {
        profile.logo_path.clone()
    } else {
        profile.signature_path.clone()
    };
    let column = if kind == "logo" {
        "logo_path"
    } else {
        "signature_path"
    };
    let connection = database.connect().map_err(|error| error.to_string())?;
    connection.execute(&format!("UPDATE barangay_profile SET {column}=NULL, updated_at=CURRENT_TIMESTAMP WHERE id=?1"), [profile.id]).map_err(|error| error.to_string())?;
    delete_image(&database, &previous_path);
    current_profile(&database)?.ok_or_else(|| "Branding image was not updated.".into())
}

#[tauri::command]
pub fn get_branding_image(
    database: State<'_, Database>,
    kind: String,
) -> Result<Option<String>, String> {
    let profile = match current_profile(&database)? {
        Some(profile) => profile,
        None => return Ok(None),
    };
    let path = if kind == "logo" {
        profile.logo_path
    } else if kind == "signature" {
        profile.signature_path
    } else {
        return Err("Unknown branding image type.".into());
    };
    match path {
        Some(relative_path) => {
            let bytes = fs::read(database.data_dir().join(relative_path))
                .map_err(|error| error.to_string())?;
            Ok(Some(STANDARD.encode(bytes)))
        }
        None => Ok(None),
    }
}
