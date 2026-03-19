use tauri::State;

use crate::bio::ode_simulator::{self, CircuitPart, OdeParams, OdeSimConfig, OdeSimResult};
use crate::db::AppDatabase;

#[tauri::command]
pub fn run_ode_simulation(
    circuit_type: &str,
    parts: Vec<CircuitPart>,
    duration_hours: f64,
    dt: f64,
    parameters: Option<OdeParams>,
) -> Result<OdeSimResult, String> {
    let config = OdeSimConfig {
        circuit_type: circuit_type.to_string(),
        parts,
        duration_hours,
        dt,
        parameters,
    };
    ode_simulator::run_ode_simulation(&config)
}

#[tauri::command]
pub fn detect_circuit(parts: Vec<CircuitPart>) -> Result<String, String> {
    Ok(ode_simulator::detect_circuit_type(&parts))
}

#[tauri::command]
pub fn seed_bacterial_demo(db: State<'_, AppDatabase>) -> Result<String, String> {
    db.seed_bacterial_parts()?;
    Ok("Bacterial demos and characterized parts seeded successfully".to_string())
}
