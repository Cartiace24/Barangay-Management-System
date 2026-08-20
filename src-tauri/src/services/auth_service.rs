use crate::database::Database;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

pub struct Session(pub Mutex<Option<SessionUser>>);
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUser {
    pub id: i64,
    pub full_name: String,
    pub username: String,
    pub role_id: i64,
    pub role_name: String,
    pub permissions: Vec<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    pub status: String,
    pub user: Option<SessionUser>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRecord {
    pub id: i64,
    pub full_name: String,
    pub username: String,
    pub position_id: Option<i64>,
    pub position_title: Option<String>,
    pub role_id: Option<i64>,
    pub role_name: Option<String>,
    pub status: String,
    pub last_login_at: Option<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lookup {
    pub id: i64,
    pub name: String,
    pub status: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInput {
    pub full_name: String,
    pub username: String,
    pub password: Option<String>,
    pub position_id: Option<i64>,
    pub role_id: i64,
    pub status: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordInput {
    pub user_id: i64,
    pub password: String,
}
#[derive(Deserialize)]
pub struct ChangePasswordInput {
    pub current_password: String,
    pub new_password: String,
}
#[derive(Deserialize)]
pub struct PositionInput {
    pub title: String,
    pub status: String,
}

const PERMISSIONS: &[(&str, &str, &str)] = &[
    ("users.view", "Users", "users"),
    ("users.create", "Create users", "users"),
    ("users.edit", "Edit users", "users"),
    ("users.deactivate", "Deactivate users", "users"),
    ("residents.view", "View residents", "residents"),
    ("residents.create", "Create residents", "residents"),
    ("residents.edit", "Edit residents", "residents"),
    ("residents.archive", "Archive residents", "residents"),
    ("households.view", "View households", "households"),
    ("households.create", "Create households", "households"),
    ("households.edit", "Edit households", "households"),
    ("documents.view", "View documents", "documents"),
    ("documents.create", "Create documents", "documents"),
    ("documents.approve", "Approve documents", "documents"),
    ("documents.release", "Release documents", "documents"),
    ("blotter.view", "View blotter", "blotter"),
    ("blotter.create", "Create blotter", "blotter"),
    ("blotter.edit", "Edit blotter", "blotter"),
    ("payments.view", "View payments", "payments"),
    ("payments.create", "Create payments", "payments"),
    ("payments.edit", "Edit payments", "payments"),
    ("reports.view", "View reports", "reports"),
    ("reports.export", "Export reports", "reports"),
    ("settings.view", "View settings", "settings"),
    ("settings.edit", "Edit settings", "settings"),
    ("audit.view", "View audit log", "audit"),
];
const ROLES: &[(&str, &str)] = &[
    ("administrator", "Administrator"),
    ("secretary", "Secretary"),
    ("treasurer", "Treasurer"),
    ("staff", "Staff"),
    ("viewer", "Viewer"),
];
const POSITIONS: &[&str] = &[
    "Barangay Captain",
    "Barangay Secretary",
    "Barangay Treasurer",
    "SK Chairman",
    "Barangay Kagawad",
    "Barangay Staff",
    "Encoder",
    "Other",
];

pub fn hash_password(password: &str) -> Result<String, String> {
    if password.len() < 10 {
        return Err("Password must contain at least 10 characters.".into());
    }
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|v| v.to_string())
        .map_err(|e| e.to_string())
}
pub fn seed_defaults(database: &Database) -> Result<(), String> {
    let mut c = database.connect().map_err(|e| e.to_string())?;
    let tx = c.transaction().map_err(|e| e.to_string())?;
    for (code, name) in ROLES {
        tx.execute(
            "INSERT OR IGNORE INTO roles(code,name) VALUES(?1,?2)",
            params![code, name],
        )
        .map_err(|e| e.to_string())?;
    }
    for (code, name, module) in PERMISSIONS {
        tx.execute(
            "INSERT OR IGNORE INTO permissions(code,name,module) VALUES(?1,?2,?3)",
            params![code, name, module],
        )
        .map_err(|e| e.to_string())?;
    }
    let admin: i64 = tx
        .query_row("SELECT id FROM roles WHERE code='administrator'", [], |r| {
            r.get(0)
        })
        .map_err(|e| e.to_string())?;
    tx.execute("INSERT OR IGNORE INTO role_permissions(role_id,permission_id) SELECT ?1,id FROM permissions",[admin]).map_err(|e|e.to_string())?;
    for (i, title) in POSITIONS.iter().enumerate() {
        tx.execute("INSERT OR IGNORE INTO official_positions(title,display_order,status) VALUES(?1,?2,'active')",params![title,i as i64]).map_err(|e|e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())
}
pub(crate) fn audit(c: &rusqlite::Connection, user: Option<i64>, action: &str, entity: &str, id: Option<i64>) {
    let _ = c.execute(
        "INSERT INTO audit_logs(user_id,action,entity_type,entity_id) VALUES(?1,?2,?3,?4)",
        params![user, action, entity, id.map(|x| x.to_string())],
    );
}
fn session_user(c: &rusqlite::Connection, id: i64) -> Result<SessionUser, String> {
    let (id,full_name,username,role_id,role_name)=c.query_row("SELECT u.id,u.full_name,u.username,r.id,r.name FROM users u JOIN roles r ON r.id=u.role_id WHERE u.id=?1",[id],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?))).map_err(|e|e.to_string())?;
    let mut s=c.prepare("SELECT p.code FROM permissions p JOIN role_permissions rp ON rp.permission_id=p.id WHERE rp.role_id=?1").map_err(|e|e.to_string())?;
    let permissions = s
        .query_map([role_id], |r| r.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();
    Ok(SessionUser {
        id,
        full_name,
        username,
        role_id,
        role_name,
        permissions,
    })
}
pub(crate) fn require(session: &Session, permission: &str) -> Result<SessionUser, String> {
    let user = session
        .0
        .lock()
        .map_err(|_| "Session unavailable".to_string())?
        .clone()
        .ok_or_else(|| "Sign in required.".to_string())?;
    if user.permissions.iter().any(|p| p == permission) {
        Ok(user)
    } else {
        Err("You do not have permission for this action.".into())
    }
}
fn admins(c: &rusqlite::Connection) -> Result<i64, String> {
    c.query_row("SELECT COUNT(*) FROM users u JOIN roles r ON r.id=u.role_id WHERE r.code='administrator' AND u.status='active'",[],|r|r.get(0)).map_err(|e|e.to_string())
}

#[tauri::command]
pub fn login(
    database: State<'_, Database>,
    session: State<'_, Session>,
    username: String,
    password: String,
) -> Result<LoginResult, String> {
    let c = database.connect().map_err(|e| e.to_string())?;
    let row: Option<(i64, String, String)> = c
        .query_row(
            "SELECT id,password_hash,status FROM users WHERE username=?1",
            [username.trim()],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let Some((id, hash, status)) = row else {
        audit(&c, None, "login_failed", "user", None);
        return Ok(LoginResult {
            status: "invalid".into(),
            user: None,
        });
    };
    if status != "active" {
        audit(&c, Some(id), "login_disabled", "user", Some(id));
        return Ok(LoginResult {
            status: "disabled".into(),
            user: None,
        });
    }
    let parsed = PasswordHash::new(&hash).map_err(|e| e.to_string())?;
    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_err()
    {
        audit(&c, Some(id), "login_failed", "user", Some(id));
        return Ok(LoginResult {
            status: "invalid".into(),
            user: None,
        });
    }
    c.execute(
        "UPDATE users SET last_login_at=CURRENT_TIMESTAMP WHERE id=?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    let user = session_user(&c, id)?;
    *session
        .0
        .lock()
        .map_err(|_| "Session unavailable".to_string())? = Some(user.clone());
    audit(&c, Some(id), "login", "user", Some(id));
    Ok(LoginResult {
        status: "success".into(),
        user: Some(user),
    })
}
#[tauri::command]
pub fn logout(database: State<'_, Database>, session: State<'_, Session>) -> Result<(), String> {
    let user = session
        .0
        .lock()
        .map_err(|_| "Session unavailable".to_string())?
        .take();
    if let Some(user) = user {
        let c = database.connect().map_err(|e| e.to_string())?;
        audit(&c, Some(user.id), "logout", "user", Some(user.id));
    }
    Ok(())
}
#[tauri::command]
pub fn current_session(session: State<'_, Session>) -> Result<Option<SessionUser>, String> {
    Ok(session
        .0
        .lock()
        .map_err(|_| "Session unavailable".to_string())?
        .clone())
}
#[tauri::command]
pub fn list_users(
    database: State<'_, Database>,
    session: State<'_, Session>,
) -> Result<Vec<UserRecord>, String> {
    require(&session, "users.view")?;
    let c = database.connect().map_err(|e| e.to_string())?;
    let mut s=c.prepare("SELECT u.id,u.full_name,u.username,u.official_position_id,op.title,u.role_id,r.name,u.status,u.last_login_at FROM users u LEFT JOIN official_positions op ON op.id=u.official_position_id LEFT JOIN roles r ON r.id=u.role_id ORDER BY u.full_name").map_err(|e|e.to_string())?;
    let records = s
        .query_map([], |r| {
            Ok(UserRecord {
                id: r.get(0)?,
                full_name: r.get(1)?,
                username: r.get(2)?,
                position_id: r.get(3)?,
                position_title: r.get(4)?,
                role_id: r.get(5)?,
                role_name: r.get(6)?,
                status: r.get(7)?,
                last_login_at: r.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(records)
}
#[tauri::command]
pub fn list_roles(
    database: State<'_, Database>,
    session: State<'_, Session>,
) -> Result<Vec<Lookup>, String> {
    require(&session, "users.view")?;
    let c = database.connect().map_err(|e| e.to_string())?;
    let mut s = c
        .prepare("SELECT id,name,status FROM roles ORDER BY name")
        .map_err(|e| e.to_string())?;
    let records = s
        .query_map([], |r| {
            Ok(Lookup {
                id: r.get(0)?,
                name: r.get(1)?,
                status: r.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(records)
}
#[tauri::command]
pub fn list_positions(
    database: State<'_, Database>,
    session: State<'_, Session>,
) -> Result<Vec<Lookup>, String> {
    require(&session, "users.view")?;
    let c = database.connect().map_err(|e| e.to_string())?;
    let mut s = c
        .prepare("SELECT id,title,status FROM official_positions ORDER BY display_order,title")
        .map_err(|e| e.to_string())?;
    let records = s
        .query_map([], |r| {
            Ok(Lookup {
                id: r.get(0)?,
                name: r.get(1)?,
                status: r.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(records)
}
#[tauri::command]
pub fn create_position(
    database: State<'_, Database>,
    session: State<'_, Session>,
    input: PositionInput,
) -> Result<Lookup, String> {
    require(&session, "settings.edit")?;
    if input.title.trim().is_empty() {
        return Err("Position title is required.".into());
    }
    let c = database.connect().map_err(|e| e.to_string())?;
    c.execute(
        "INSERT INTO official_positions(title,status,display_order) VALUES(?1,?2,999)",
        params![input.title.trim(), input.status],
    )
    .map_err(|e| e.to_string())?;
    let id = c.last_insert_rowid();
    let actor = session
        .0
        .lock()
        .map_err(|_| "Session unavailable".to_string())?
        .clone()
        .unwrap();
    audit(
        &c,
        Some(actor.id),
        "position_created",
        "official_position",
        Some(id),
    );
    Ok(Lookup {
        id,
        name: input.title.trim().into(),
        status: input.status,
    })
}
#[tauri::command]
pub fn create_user(
    database: State<'_, Database>,
    session: State<'_, Session>,
    input: UserInput,
) -> Result<UserRecord, String> {
    let actor = require(&session, "users.create")?;
    let password = hash_password(input.password.as_deref().ok_or("Password is required.")?)?;
    let c = database.connect().map_err(|e| e.to_string())?;
    c.execute("INSERT INTO users(full_name,username,password_hash,official_position_id,role_id,status) VALUES(?1,?2,?3,?4,?5,?6)",params![input.full_name.trim(),input.username.trim(),password,input.position_id,input.role_id,input.status]).map_err(|e|e.to_string())?;
    let id = c.last_insert_rowid();
    audit(&c, Some(actor.id), "user_created", "user", Some(id));
    get_user(&c, id)
}
fn get_user(c: &rusqlite::Connection, id: i64) -> Result<UserRecord, String> {
    c.query_row("SELECT u.id,u.full_name,u.username,u.official_position_id,op.title,u.role_id,r.name,u.status,u.last_login_at FROM users u LEFT JOIN official_positions op ON op.id=u.official_position_id LEFT JOIN roles r ON r.id=u.role_id WHERE u.id=?1",[id],|r|Ok(UserRecord{id:r.get(0)?,full_name:r.get(1)?,username:r.get(2)?,position_id:r.get(3)?,position_title:r.get(4)?,role_id:r.get(5)?,role_name:r.get(6)?,status:r.get(7)?,last_login_at:r.get(8)?})).map_err(|e|e.to_string())
}
#[tauri::command]
pub fn update_user(
    database: State<'_, Database>,
    session: State<'_, Session>,
    user_id: i64,
    input: UserInput,
) -> Result<UserRecord, String> {
    let actor = require(&session, "users.edit")?;
    let c = database.connect().map_err(|e| e.to_string())?;
    let target = get_user(&c, user_id)?;
    let admin_role: bool = c
        .query_row(
            "SELECT code='administrator' FROM roles WHERE id=?1",
            [target.role_id],
            |r| r.get(0),
        )
        .unwrap_or(false);
    let new_admin: bool = c
        .query_row(
            "SELECT code='administrator' FROM roles WHERE id=?1",
            [input.role_id],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if actor.id == user_id
        && admin_role
        && (!new_admin || input.status != "active")
        && admins(&c)? <= 1
    {
        return Err("The last active administrator cannot remove their own administrator access or deactivate themselves.".into());
    }
    c.execute("UPDATE users SET full_name=?1,official_position_id=?2,role_id=?3,status=?4,updated_at=CURRENT_TIMESTAMP WHERE id=?5",params![input.full_name.trim(),input.position_id,input.role_id,input.status,user_id]).map_err(|e|e.to_string())?;
    audit(
        &c,
        Some(actor.id),
        if target.status != input.status {
            "user_status_changed"
        } else if target.role_id != Some(input.role_id) {
            "role_changed"
        } else if target.position_id != input.position_id {
            "position_changed"
        } else {
            "user_edited"
        },
        "user",
        Some(user_id),
    );
    get_user(&c, user_id)
}
#[tauri::command]
pub fn reset_user_password(
    database: State<'_, Database>,
    session: State<'_, Session>,
    input: PasswordInput,
) -> Result<(), String> {
    let actor = require(&session, "users.edit")?;
    let c = database.connect().map_err(|e| e.to_string())?;
    let hash = hash_password(&input.password)?;
    c.execute(
        "UPDATE users SET password_hash=?1,updated_at=CURRENT_TIMESTAMP WHERE id=?2",
        params![hash, input.user_id],
    )
    .map_err(|e| e.to_string())?;
    audit(
        &c,
        Some(actor.id),
        "password_reset",
        "user",
        Some(input.user_id),
    );
    Ok(())
}
#[tauri::command]
pub fn change_own_password(
    database: State<'_, Database>,
    session: State<'_, Session>,
    input: ChangePasswordInput,
) -> Result<(), String> {
    let user = session
        .0
        .lock()
        .map_err(|_| "Session unavailable".to_string())?
        .clone()
        .ok_or("Sign in required.")?;
    let c = database.connect().map_err(|e| e.to_string())?;
    let hash: String = c
        .query_row(
            "SELECT password_hash FROM users WHERE id=?1",
            [user.id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    let parsed = PasswordHash::new(&hash).map_err(|e| e.to_string())?;
    if Argon2::default()
        .verify_password(input.current_password.as_bytes(), &parsed)
        .is_err()
    {
        return Err("Current password is incorrect.".into());
    }
    let replacement = hash_password(&input.new_password)?;
    c.execute(
        "UPDATE users SET password_hash=?1,updated_at=CURRENT_TIMESTAMP WHERE id=?2",
        params![replacement, user.id],
    )
    .map_err(|e| e.to_string())?;
    audit(&c, Some(user.id), "password_changed", "user", Some(user.id));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn default_security_data_is_seeded_and_passwords_are_hashed() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("bms-auth-test-{nonce}.sqlite3"));
        let database = Database::open(path.clone()).expect("database");
        seed_defaults(&database).expect("defaults");
        let connection = database.connect().expect("connection");
        let roles: i64 = connection
            .query_row("SELECT COUNT(*) FROM roles", [], |row| row.get(0))
            .expect("roles");
        let permissions: i64 = connection
            .query_row("SELECT COUNT(*) FROM permissions", [], |row| row.get(0))
            .expect("permissions");
        assert!(roles >= 5);
        assert_eq!(permissions as usize, PERMISSIONS.len());
        let hash = hash_password("a-strong-test-password").expect("hash");
        assert_ne!(hash, "a-strong-test-password");
        let parsed = PasswordHash::new(&hash).expect("parse hash");
        assert!(Argon2::default()
            .verify_password(b"a-strong-test-password", &parsed)
            .is_ok());
        drop(connection);
        let _ = std::fs::remove_file(path);
    }
}
