use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::AppDatabase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatorRun {
    pub id: String,
    pub construct_id: String,
    pub host: String,
    pub params: serde_json::Value,
    pub results: serde_json::Value,
    pub created_at: String,
}

impl AppDatabase {
    pub fn create_simulator_run(
        &self,
        construct_id: &str,
        host: &str,
        params: &serde_json::Value,
        results: &serde_json::Value,
    ) -> Result<SimulatorRun, String> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let params_str = serde_json::to_string(params).unwrap_or_default();
        let results_str = serde_json::to_string(results).unwrap_or_default();
        conn.execute(
            "INSERT INTO simulator_runs (id, construct_id, host, params, results) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, construct_id, host, params_str, results_str],
        )
        .map_err(|e| e.to_string())?;

        drop(conn);
        self.get_simulator_run(&id)
    }

    pub fn get_simulator_run(&self, id: &str) -> Result<SimulatorRun, String> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, construct_id, host, params, results, created_at FROM simulator_runs WHERE id = ?1",
            params![id],
            |row| {
                let p: String = row.get(3)?;
                let r: String = row.get(4)?;
                Ok(SimulatorRun {
                    id: row.get(0)?,
                    construct_id: row.get(1)?,
                    host: row.get(2)?,
                    params: serde_json::from_str(&p).unwrap_or_default(),
                    results: serde_json::from_str(&r).unwrap_or_default(),
                    created_at: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())
    }

    pub fn list_simulator_runs(&self, construct_id: &str) -> Result<Vec<SimulatorRun>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, construct_id, host, params, results, created_at FROM simulator_runs WHERE construct_id = ?1 ORDER BY created_at DESC",
            )
            .map_err(|e| e.to_string())?;

        let runs = stmt
            .query_map(params![construct_id], |row| {
                let p: String = row.get(3)?;
                let r: String = row.get(4)?;
                Ok(SimulatorRun {
                    id: row.get(0)?,
                    construct_id: row.get(1)?,
                    host: row.get(2)?,
                    params: serde_json::from_str(&p).unwrap_or_default(),
                    results: serde_json::from_str(&r).unwrap_or_default(),
                    created_at: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(runs)
    }

    pub fn delete_simulator_run(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM simulator_runs WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
