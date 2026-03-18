use tauri::State;

use crate::db::parts::Part;
use crate::db::AppDatabase;

#[tauri::command]
pub fn create_part(
    db: State<'_, AppDatabase>,
    name: &str,
    part_type: &str,
    sequence: &str,
    description: &str,
) -> Result<Part, String> {
    db.create_part(name, part_type, sequence, description)
}

#[tauri::command]
pub fn get_part(db: State<'_, AppDatabase>, id: &str) -> Result<Part, String> {
    db.get_part(id)
}

#[tauri::command]
pub fn list_parts(db: State<'_, AppDatabase>, part_type: Option<String>) -> Result<Vec<Part>, String> {
    db.list_parts(part_type.as_deref())
}

#[tauri::command]
pub fn search_parts(db: State<'_, AppDatabase>, query: &str) -> Result<Vec<Part>, String> {
    db.search_parts(query)
}

#[tauri::command]
pub fn delete_part(db: State<'_, AppDatabase>, id: &str) -> Result<(), String> {
    db.delete_part(id)
}

#[tauri::command]
pub fn seed_parts(db: State<'_, AppDatabase>) -> Result<(), String> {
    db.seed_default_parts()
}
