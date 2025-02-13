use crate::miniatures::generate_thumbnail_async;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

#[tauri::command(rename_all = "snake_case")]
pub fn get_images(app: AppHandle, folder_path: String) -> Vec<(String, Option<String>)> {
    let path = PathBuf::from(&folder_path);
    let thumb_dir = path.join("miniatures");

    if !thumb_dir.exists() {
        let _ = fs::create_dir_all(&thumb_dir);
    }

    // Use Arc and Mutex to share entries across threads safely
    let entries: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(
        fs::read_dir(&path)
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|file_path| {
                file_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|ext| {
                        matches!(
                            ext.to_lowercase().as_str(),
                            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp"
                        )
                    })
                    .unwrap_or(false)
            })
            .collect(),
    ));

    let app_clone = app.clone();
    let thumb_dir_clone = thumb_dir.clone();

    // Clone Arc to move into the thread, so it can still be used after the move
    let entries_clone = Arc::clone(&entries);

    // Spawn thumbnail generation asynchronously
    std::thread::spawn(move || {
        let entries = entries_clone.lock().unwrap(); // Lock the entries to access them
        for file_path in &*entries {
            generate_thumbnail_async(&app_clone, file_path, &thumb_dir_clone);
        }
    });

    // Return image paths immediately (thumbnails will be updated dynamically)
    let entries = entries.lock().unwrap(); // Lock to get a reference to entries
    entries
        .iter()
        .map(|file_path| (file_path.to_string_lossy().to_string(), None))
        .collect()
}
