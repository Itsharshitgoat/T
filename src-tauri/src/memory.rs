use rusqlite::{Connection, Result};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

pub struct DbState {
    pub db: Mutex<Connection>,
}

#[derive(Serialize, Deserialize)]
pub struct Memory {
    pub id: Option<i32>,
    pub content: String,
    pub importance: i32,
    pub category: String,
}

#[derive(Serialize, Deserialize)]
pub struct Conversation {
    pub id: Option<i32>,
    pub user_message: String,
    pub assistant_response: String,
}

pub fn init_db(app_dir: &std::path::Path) -> Result<Connection> {
    let db_path = app_dir.join("t_memory.db");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS memories (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            importance INTEGER NOT NULL,
            category TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS conversations (
            id INTEGER PRIMARY KEY,
            user_message TEXT NOT NULL,
            assistant_response TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS preferences (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

#[tauri::command]
pub fn add_conversation(user_message: String, assistant_response: String, state: tauri::State<DbState>) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "INSERT INTO conversations (user_message, assistant_response) VALUES (?1, ?2)",
        rusqlite::params![user_message, assistant_response],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_recent_conversations(state: tauri::State<DbState>) -> Result<Vec<Conversation>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, user_message, assistant_response FROM conversations ORDER BY id DESC LIMIT 5").map_err(|e| e.to_string())?;
    let conv_iter = stmt.query_map([], |row| {
        Ok(Conversation {
            id: row.get(0)?,
            user_message: row.get(1)?,
            assistant_response: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut convs = Vec::new();
    for conv in conv_iter {
        convs.push(conv.map_err(|e| e.to_string())?);
    }
    // Reverse to chronological order
    convs.reverse();
    Ok(convs)
}

#[tauri::command]
pub fn get_preference(key: String, state: tauri::State<DbState>) -> Result<Option<String>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT value FROM preferences WHERE key = ?1").map_err(|e| e.to_string())?;
    let mut rows = stmt.query(rusqlite::params![key]).map_err(|e| e.to_string())?;
    
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(Some(row.get(0).map_err(|e| e.to_string())?))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn set_preference(key: String, value: String, state: tauri::State<DbState>) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "INSERT INTO preferences (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        rusqlite::params![key, value],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
