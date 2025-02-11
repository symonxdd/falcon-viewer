// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::path::PathBuf;

// Greet function (default in Tauri template)
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ✅ New function to get images from a folder
#[tauri::command(rename_all = "snake_case")]
fn get_images(folder_path: String) -> Vec<String> {
    let path = PathBuf::from(folder_path);
    if path.is_dir() {
        let entries = fs::read_dir(path).unwrap();
        entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_path = entry.path();
                let ext = file_path.extension()?.to_str()?.to_lowercase();
                if ["jpg", "jpeg", "png", "gif", "bmp", "webp"].contains(&ext.as_str()) {
                    Some(file_path.to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![]
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_images]) // ✅ Register both functions
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
