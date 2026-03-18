use tauri::State;

use crate::db::constructs::{Construct, ConstructPart, ConstructVersion};
use crate::db::repos::Repo;
use crate::db::AppDatabase;

// -- Repos --

#[tauri::command]
pub fn create_repo(db: State<'_, AppDatabase>, name: &str, description: &str) -> Result<Repo, String> {
    db.create_repo(name, description)
}

#[tauri::command]
pub fn get_repo(db: State<'_, AppDatabase>, id: &str) -> Result<Repo, String> {
    db.get_repo(id)
}

#[tauri::command]
pub fn list_repos(db: State<'_, AppDatabase>) -> Result<Vec<Repo>, String> {
    db.list_repos()
}

#[tauri::command]
pub fn update_repo(db: State<'_, AppDatabase>, id: &str, name: &str, description: &str) -> Result<Repo, String> {
    db.update_repo(id, name, description)
}

#[tauri::command]
pub fn delete_repo(db: State<'_, AppDatabase>, id: &str) -> Result<(), String> {
    db.delete_repo(id)
}

#[tauri::command]
pub fn search_repos(db: State<'_, AppDatabase>, query: &str) -> Result<Vec<Repo>, String> {
    db.search_repos(query)
}

// -- Constructs --

#[tauri::command]
pub fn create_construct(
    db: State<'_, AppDatabase>,
    repo_id: &str,
    name: &str,
    description: &str,
    topology: &str,
) -> Result<Construct, String> {
    db.create_construct(repo_id, name, description, topology)
}

#[tauri::command]
pub fn get_construct(db: State<'_, AppDatabase>, id: &str) -> Result<Construct, String> {
    db.get_construct(id)
}

#[tauri::command]
pub fn list_constructs(db: State<'_, AppDatabase>, repo_id: &str) -> Result<Vec<Construct>, String> {
    db.list_constructs(repo_id)
}

#[tauri::command]
pub fn update_construct(
    db: State<'_, AppDatabase>,
    id: &str,
    name: &str,
    description: &str,
    topology: &str,
    tags: Vec<String>,
    sequence: &str,
) -> Result<Construct, String> {
    db.update_construct(id, name, description, topology, &tags, sequence)
}

#[tauri::command]
pub fn delete_construct(db: State<'_, AppDatabase>, id: &str) -> Result<(), String> {
    db.delete_construct(id)
}

#[tauri::command]
pub fn search_constructs(db: State<'_, AppDatabase>, query: &str) -> Result<Vec<Construct>, String> {
    db.search_constructs(query)
}

// -- Versions --

#[tauri::command]
pub fn list_versions(db: State<'_, AppDatabase>, construct_id: &str) -> Result<Vec<ConstructVersion>, String> {
    db.list_versions(construct_id)
}

#[tauri::command]
pub fn revert_construct(db: State<'_, AppDatabase>, construct_id: &str, version_id: &str) -> Result<Construct, String> {
    db.revert_construct(construct_id, version_id)
}

// -- Construct Parts --

#[tauri::command]
pub fn add_construct_part(
    db: State<'_, AppDatabase>,
    construct_id: &str,
    part_id: &str,
    position: i64,
    strand: i32,
    order_index: i64,
) -> Result<ConstructPart, String> {
    db.add_construct_part(construct_id, part_id, position, strand, order_index)
}

#[tauri::command]
pub fn get_construct_parts(db: State<'_, AppDatabase>, construct_id: &str) -> Result<Vec<ConstructPart>, String> {
    db.get_construct_parts(construct_id)
}

#[tauri::command]
pub fn remove_construct_part(db: State<'_, AppDatabase>, id: &str) -> Result<(), String> {
    db.remove_construct_part(id)
}

#[tauri::command]
pub fn reorder_construct_parts(db: State<'_, AppDatabase>, construct_id: &str, part_ids: Vec<String>) -> Result<(), String> {
    db.reorder_construct_parts(construct_id, &part_ids)
}

#[tauri::command]
pub fn flip_construct_part(db: State<'_, AppDatabase>, id: &str, strand: i32) -> Result<(), String> {
    db.update_construct_part_strand(id, strand)
}
