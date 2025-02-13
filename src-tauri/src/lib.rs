use image::{imageops::FilterType, DynamicImage, GenericImageView};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[tauri::command(rename_all = "snake_case")]
fn get_images(folder_path: String) -> Vec<(String, String)> {
    let path = PathBuf::from(&folder_path);
    let thumb_dir = path.join("miniatures"); // ✅ Store thumbnails in a separate folder

    // ✅ Ensure the miniatures folder exists
    if !thumb_dir.exists() {
        let _ = fs::create_dir_all(&thumb_dir);
    }

    if path.is_dir() {
        let entries: Vec<_> = fs::read_dir(path)
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|file_path| {
                if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                    ["jpg", "jpeg", "png", "gif", "bmp", "webp"]
                        .contains(&ext.to_lowercase().as_str())
                } else {
                    false
                }
            })
            .collect();

        // ✅ Use Rayon for parallel processing
        let thumb_dir = Arc::new(thumb_dir);
        entries
            .par_iter()
            .filter_map(|file_path| {
                let thumb_path = create_thumbnail(file_path, &thumb_dir);
                Some((file_path.to_string_lossy().to_string(), thumb_path))
            })
            .collect()
    } else {
        vec![]
    }
}

fn create_thumbnail(image_path: &Path, thumb_dir: &Path) -> String {
    let file_name = match image_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return "".to_string(),
    };

    let thumb_path = thumb_dir.join(format!("thumb_{}", file_name));

    // ✅ If the thumbnail already exists, return its path
    if thumb_path.exists() {
        return thumb_path.to_string_lossy().to_string();
    }

    // ✅ Try to generate and save the thumbnail
    if let Ok(img) = image::open(image_path) {
        let thumb = img.resize(128, 128, FilterType::Lanczos3);
        if thumb.save(&thumb_path).is_ok() {
            return thumb_path.to_string_lossy().to_string();
        }
    }

    "".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
