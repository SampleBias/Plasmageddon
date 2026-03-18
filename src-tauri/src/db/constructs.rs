use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::AppDatabase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Construct {
    pub id: String,
    pub repo_id: String,
    pub name: String,
    pub description: String,
    pub topology: String,
    pub tags: Vec<String>,
    pub sequence: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructPart {
    pub id: String,
    pub construct_id: String,
    pub part_id: String,
    pub position: i64,
    pub strand: i32,
    pub order_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructVersion {
    pub id: String,
    pub construct_id: String,
    pub version_num: i64,
    pub sequence: String,
    pub parts_json: String,
    pub snapshot_at: String,
}

impl AppDatabase {
    pub fn create_construct(
        &self,
        repo_id: &str,
        name: &str,
        description: &str,
        topology: &str,
    ) -> Result<Construct, String> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO constructs (id, repo_id, name, description, topology) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, repo_id, name, description, topology],
        )
        .map_err(|e| e.to_string())?;
        drop(conn);
        self.get_construct(&id)
    }

    pub fn get_construct(&self, id: &str) -> Result<Construct, String> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, repo_id, name, description, topology, tags, sequence, created_at, updated_at FROM constructs WHERE id = ?1",
            params![id],
            |row| {
                let tags_str: String = row.get(5)?;
                let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
                Ok(Construct {
                    id: row.get(0)?,
                    repo_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    topology: row.get(4)?,
                    tags,
                    sequence: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        )
        .map_err(|e| e.to_string())
    }

    pub fn list_constructs(&self, repo_id: &str) -> Result<Vec<Construct>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, repo_id, name, description, topology, tags, sequence, created_at, updated_at FROM constructs WHERE repo_id = ?1 ORDER BY updated_at DESC",
            )
            .map_err(|e| e.to_string())?;

        let items = stmt
            .query_map(params![repo_id], |row| {
                let tags_str: String = row.get(5)?;
                let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
                Ok(Construct {
                    id: row.get(0)?,
                    repo_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    topology: row.get(4)?,
                    tags,
                    sequence: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(items)
    }

    pub fn update_construct(
        &self,
        id: &str,
        name: &str,
        description: &str,
        topology: &str,
        tags: &[String],
        sequence: &str,
    ) -> Result<Construct, String> {
        let tags_json = serde_json::to_string(tags).unwrap_or_else(|_| "[]".into());
        {
            let conn = self.conn.lock().unwrap();
            conn.execute(
                "UPDATE constructs SET name=?1, description=?2, topology=?3, tags=?4, sequence=?5, updated_at=datetime('now') WHERE id=?6",
                params![name, description, topology, tags_json, sequence, id],
            )
            .map_err(|e| e.to_string())?;
        }
        self.snapshot_version(id)?;
        self.get_construct(id)
    }

    pub fn delete_construct(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM constructs WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn search_constructs(&self, query: &str) -> Result<Vec<Construct>, String> {
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{query}%");
        let mut stmt = conn
            .prepare(
                "SELECT id, repo_id, name, description, topology, tags, sequence, created_at, updated_at FROM constructs WHERE name LIKE ?1 OR description LIKE ?1 ORDER BY updated_at DESC LIMIT 50",
            )
            .map_err(|e| e.to_string())?;

        let items = stmt
            .query_map(params![pattern], |row| {
                let tags_str: String = row.get(5)?;
                let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
                Ok(Construct {
                    id: row.get(0)?,
                    repo_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    topology: row.get(4)?,
                    tags,
                    sequence: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(items)
    }

    // Auto-versioning
    fn snapshot_version(&self, construct_id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        let next_ver: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version_num), 0) + 1 FROM construct_versions WHERE construct_id = ?1",
                params![construct_id],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        let construct_seq: String = conn
            .query_row(
                "SELECT sequence FROM constructs WHERE id = ?1",
                params![construct_id],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        let parts_json = self.get_construct_parts_json_inner(&conn, construct_id)?;

        let vid = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO construct_versions (id, construct_id, version_num, sequence, parts_json) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![vid, construct_id, next_ver, construct_seq, parts_json],
        )
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn get_construct_parts_json_inner(
        &self,
        conn: &rusqlite::Connection,
        construct_id: &str,
    ) -> Result<String, String> {
        let mut stmt = conn
            .prepare(
                "SELECT id, construct_id, part_id, position, strand, order_index FROM construct_parts WHERE construct_id = ?1 ORDER BY order_index",
            )
            .map_err(|e| e.to_string())?;

        let parts: Vec<ConstructPart> = stmt
            .query_map(params![construct_id], |row| {
                Ok(ConstructPart {
                    id: row.get(0)?,
                    construct_id: row.get(1)?,
                    part_id: row.get(2)?,
                    position: row.get(3)?,
                    strand: row.get(4)?,
                    order_index: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        serde_json::to_string(&parts).map_err(|e| e.to_string())
    }

    pub fn list_versions(&self, construct_id: &str) -> Result<Vec<ConstructVersion>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, construct_id, version_num, sequence, parts_json, snapshot_at FROM construct_versions WHERE construct_id = ?1 ORDER BY version_num DESC",
            )
            .map_err(|e| e.to_string())?;

        let items = stmt
            .query_map(params![construct_id], |row| {
                Ok(ConstructVersion {
                    id: row.get(0)?,
                    construct_id: row.get(1)?,
                    version_num: row.get(2)?,
                    sequence: row.get(3)?,
                    parts_json: row.get(4)?,
                    snapshot_at: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(items)
    }

    pub fn revert_construct(&self, construct_id: &str, version_id: &str) -> Result<Construct, String> {
        let conn = self.conn.lock().unwrap();
        let (seq, parts_json): (String, String) = conn
            .query_row(
                "SELECT sequence, parts_json FROM construct_versions WHERE id = ?1 AND construct_id = ?2",
                params![version_id, construct_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .map_err(|e| e.to_string())?;

        conn.execute(
            "UPDATE constructs SET sequence = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![seq, construct_id],
        )
        .map_err(|e| e.to_string())?;

        conn.execute(
            "DELETE FROM construct_parts WHERE construct_id = ?1",
            params![construct_id],
        )
        .map_err(|e| e.to_string())?;

        let parts: Vec<ConstructPart> =
            serde_json::from_str(&parts_json).unwrap_or_default();
        for p in parts {
            let new_id = Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO construct_parts (id, construct_id, part_id, position, strand, order_index) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![new_id, construct_id, p.part_id, p.position, p.strand, p.order_index],
            )
            .map_err(|e| e.to_string())?;
        }

        drop(conn);
        self.get_construct(construct_id)
    }

    // Construct parts management
    pub fn add_construct_part(
        &self,
        construct_id: &str,
        part_id: &str,
        position: i64,
        strand: i32,
        order_index: i64,
    ) -> Result<ConstructPart, String> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO construct_parts (id, construct_id, part_id, position, strand, order_index) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, construct_id, part_id, position, strand, order_index],
        )
        .map_err(|e| e.to_string())?;

        Ok(ConstructPart {
            id,
            construct_id: construct_id.to_string(),
            part_id: part_id.to_string(),
            position,
            strand,
            order_index,
        })
    }

    pub fn get_construct_parts(&self, construct_id: &str) -> Result<Vec<ConstructPart>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, construct_id, part_id, position, strand, order_index FROM construct_parts WHERE construct_id = ?1 ORDER BY order_index",
            )
            .map_err(|e| e.to_string())?;

        let parts = stmt
            .query_map(params![construct_id], |row| {
                Ok(ConstructPart {
                    id: row.get(0)?,
                    construct_id: row.get(1)?,
                    part_id: row.get(2)?,
                    position: row.get(3)?,
                    strand: row.get(4)?,
                    order_index: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(parts)
    }

    pub fn remove_construct_part(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM construct_parts WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn reorder_construct_parts(&self, construct_id: &str, part_ids: &[String]) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        for (idx, pid) in part_ids.iter().enumerate() {
            conn.execute(
                "UPDATE construct_parts SET order_index = ?1 WHERE id = ?2 AND construct_id = ?3",
                params![idx as i64, pid, construct_id],
            )
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn update_construct_part_strand(&self, id: &str, strand: i32) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE construct_parts SET strand = ?1 WHERE id = ?2",
            params![strand, id],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
