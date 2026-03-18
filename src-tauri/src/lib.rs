pub mod ai;
pub mod bio;
pub mod biosecurity;
pub mod commands;
pub mod db;
pub mod parsers;

use db::AppDatabase;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            let database = AppDatabase::new(app_dir).expect("Failed to init database");
            database.seed_default_parts().ok();
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Repos
            commands::construct_cmds::create_repo,
            commands::construct_cmds::get_repo,
            commands::construct_cmds::list_repos,
            commands::construct_cmds::update_repo,
            commands::construct_cmds::delete_repo,
            commands::construct_cmds::search_repos,
            // Constructs
            commands::construct_cmds::create_construct,
            commands::construct_cmds::get_construct,
            commands::construct_cmds::list_constructs,
            commands::construct_cmds::update_construct,
            commands::construct_cmds::delete_construct,
            commands::construct_cmds::search_constructs,
            // Versions
            commands::construct_cmds::list_versions,
            commands::construct_cmds::revert_construct,
            // Construct Parts
            commands::construct_cmds::add_construct_part,
            commands::construct_cmds::get_construct_parts,
            commands::construct_cmds::remove_construct_part,
            commands::construct_cmds::reorder_construct_parts,
            commands::construct_cmds::flip_construct_part,
            // Parts library
            commands::parts_cmds::create_part,
            commands::parts_cmds::get_part,
            commands::parts_cmds::list_parts,
            commands::parts_cmds::search_parts,
            commands::parts_cmds::delete_part,
            commands::parts_cmds::seed_parts,
            // Sequence tools
            commands::sequence_cmds::find_restriction_sites,
            commands::sequence_cmds::compute_gc_content,
            commands::sequence_cmds::compute_melting_temp,
            commands::sequence_cmds::find_orfs,
            commands::sequence_cmds::find_in_sequence,
            commands::sequence_cmds::translate_sequence,
            // AI
            commands::ai_cmds::run_compiler,
            commands::ai_cmds::run_simulator,
            commands::ai_cmds::ai_chat,
            commands::ai_cmds::ai_chat_stream,
            commands::ai_cmds::get_chat_history,
            commands::ai_cmds::clear_chat_history,
            commands::ai_cmds::suggest_parts,
            // I/O
            commands::io_cmds::import_file,
            commands::io_cmds::export_genbank,
            commands::io_cmds::export_fasta,
            commands::io_cmds::export_csv,
            commands::io_cmds::screen_sequence,
            // Settings
            commands::settings_cmds::get_setting,
            commands::settings_cmds::set_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Plasmageddon");
}
