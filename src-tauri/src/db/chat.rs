use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::AppDatabase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub construct_id: Option<String>,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

impl AppDatabase {
    pub fn add_chat_message(
        &self,
        construct_id: Option<&str>,
        role: &str,
        content: &str,
    ) -> Result<ChatMessage, String> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO chat_messages (id, construct_id, role, content) VALUES (?1, ?2, ?3, ?4)",
            params![id, construct_id, role, content],
        )
        .map_err(|e| e.to_string())?;

        Ok(ChatMessage {
            id,
            construct_id: construct_id.map(|s| s.to_string()),
            role: role.to_string(),
            content: content.to_string(),
            created_at: String::new(),
        })
    }

    pub fn get_chat_history(&self, construct_id: Option<&str>) -> Result<Vec<ChatMessage>, String> {
        let conn = self.conn.lock().unwrap();
        let (sql, bind): (String, Vec<Box<dyn rusqlite::types::ToSql>>) = match construct_id {
            Some(cid) => (
                "SELECT id, construct_id, role, content, created_at FROM chat_messages WHERE construct_id = ?1 ORDER BY created_at ASC".into(),
                vec![Box::new(cid.to_string())],
            ),
            None => (
                "SELECT id, construct_id, role, content, created_at FROM chat_messages WHERE construct_id IS NULL ORDER BY created_at ASC".into(),
                vec![],
            ),
        };

        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
        let params_slice: Vec<&dyn rusqlite::types::ToSql> = bind.iter().map(|b| b.as_ref()).collect();
        let msgs = stmt
            .query_map(params_slice.as_slice(), |row| {
                Ok(ChatMessage {
                    id: row.get(0)?,
                    construct_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(msgs)
    }

    pub fn clear_chat_history(&self, construct_id: Option<&str>) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        match construct_id {
            Some(cid) => {
                conn.execute(
                    "DELETE FROM chat_messages WHERE construct_id = ?1",
                    params![cid],
                )
                .map_err(|e| e.to_string())?;
            }
            None => {
                conn.execute(
                    "DELETE FROM chat_messages WHERE construct_id IS NULL",
                    [],
                )
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    // Settings
    pub fn get_setting(&self, key: &str) -> Result<Option<String>, String> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get::<_, String>(0),
        );
        match result {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
