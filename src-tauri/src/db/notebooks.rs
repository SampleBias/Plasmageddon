use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::AppDatabase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub repo_id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

impl AppDatabase {
    pub fn create_notebook(&self, repo_id: &str, title: &str) -> Result<Notebook, String> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO notebooks (id, repo_id, title) VALUES (?1, ?2, ?3)",
            params![id, repo_id, title],
        )
        .map_err(|e| e.to_string())?;
        drop(conn);
        self.get_notebook(&id)
    }

    pub fn get_notebook(&self, id: &str) -> Result<Notebook, String> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, repo_id, title, content, created_at, updated_at FROM notebooks WHERE id = ?1",
            params![id],
            |row| {
                Ok(Notebook {
                    id: row.get(0)?,
                    repo_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())
    }

    pub fn list_notebooks(&self, repo_id: &str) -> Result<Vec<Notebook>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, repo_id, title, content, created_at, updated_at FROM notebooks WHERE repo_id = ?1 ORDER BY updated_at DESC")
            .map_err(|e| e.to_string())?;
        let items = stmt
            .query_map(params![repo_id], |row| {
                Ok(Notebook {
                    id: row.get(0)?,
                    repo_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(items)
    }

    pub fn update_notebook(&self, id: &str, title: &str, content: &str) -> Result<Notebook, String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE notebooks SET title = ?1, content = ?2, updated_at = datetime('now') WHERE id = ?3",
            params![title, content, id],
        )
        .map_err(|e| e.to_string())?;
        drop(conn);
        self.get_notebook(id)
    }

    pub fn delete_notebook(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM notebooks WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
