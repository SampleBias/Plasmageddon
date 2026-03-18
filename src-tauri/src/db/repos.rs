use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::AppDatabase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

impl AppDatabase {
    pub fn create_repo(&self, name: &str, description: &str) -> Result<Repo, String> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO repos (id, name, description) VALUES (?1, ?2, ?3)",
            params![id, name, description],
        )
        .map_err(|e| e.to_string())?;

        self.get_repo(&id)
    }

    pub fn get_repo(&self, id: &str) -> Result<Repo, String> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, description, created_at, updated_at FROM repos WHERE id = ?1",
            params![id],
            |row| {
                Ok(Repo {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        )
        .map_err(|e| e.to_string())
    }

    pub fn list_repos(&self) -> Result<Vec<Repo>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, name, description, created_at, updated_at FROM repos ORDER BY updated_at DESC")
            .map_err(|e| e.to_string())?;

        let repos = stmt
            .query_map([], |row| {
                Ok(Repo {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(repos)
    }

    pub fn update_repo(&self, id: &str, name: &str, description: &str) -> Result<Repo, String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE repos SET name = ?1, description = ?2, updated_at = datetime('now') WHERE id = ?3",
            params![name, description, id],
        )
        .map_err(|e| e.to_string())?;
        drop(conn);
        self.get_repo(id)
    }

    pub fn delete_repo(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM repos WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn search_repos(&self, query: &str) -> Result<Vec<Repo>, String> {
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{query}%");
        let mut stmt = conn
            .prepare(
                "SELECT id, name, description, created_at, updated_at FROM repos WHERE name LIKE ?1 OR description LIKE ?1 ORDER BY updated_at DESC",
            )
            .map_err(|e| e.to_string())?;

        let repos = stmt
            .query_map(params![pattern], |row| {
                Ok(Repo {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(repos)
    }
}
