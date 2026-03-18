use tauri::State;

use crate::db::AppDatabase;

#[tauri::command]
pub fn get_setting(db: State<'_, AppDatabase>, key: &str) -> Result<Option<String>, String> {
    db.get_setting(key)
}

#[tauri::command]
pub fn set_setting(db: State<'_, AppDatabase>, key: &str, value: &str) -> Result<(), String> {
    db.set_setting(key, value)
}
