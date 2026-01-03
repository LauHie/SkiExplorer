mod ski_area;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_geolocation::init())
        .invoke_handler(tauri::generate_handler![ski_area::fetch_ski_areas])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
