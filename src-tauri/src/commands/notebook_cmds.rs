use tauri::State;

use crate::db::notebooks::Notebook;
use crate::db::AppDatabase;

#[tauri::command]
pub fn create_notebook(db: State<'_, AppDatabase>, repo_id: &str, title: &str) -> Result<Notebook, String> {
    db.create_notebook(repo_id, title)
}

#[tauri::command]
pub fn get_notebook(db: State<'_, AppDatabase>, id: &str) -> Result<Notebook, String> {
    db.get_notebook(id)
}

#[tauri::command]
pub fn list_notebooks(db: State<'_, AppDatabase>, repo_id: &str) -> Result<Vec<Notebook>, String> {
    db.list_notebooks(repo_id)
}

#[tauri::command]
pub fn update_notebook(
    db: State<'_, AppDatabase>,
    id: &str,
    title: &str,
    content: &str,
) -> Result<Notebook, String> {
    db.update_notebook(id, title, content)
}

#[tauri::command]
pub fn delete_notebook(db: State<'_, AppDatabase>, id: &str) -> Result<(), String> {
    db.delete_notebook(id)
}
