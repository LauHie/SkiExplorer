mod types;
mod ski_area;
mod standort;
pub use types::Position;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ski_area::fetch_ski_areas,standort::get_standort]) //Wichtig um die Funktionen im Front-End nutzen zu können
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
