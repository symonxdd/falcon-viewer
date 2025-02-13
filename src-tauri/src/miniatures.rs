use image::{imageops::FilterType, GenericImageView};
use rayon::prelude::*;
use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};

// ✅ Define a struct for event payloads
#[derive(Clone, Serialize)]
struct ThumbnailReadyEvent {
    image_path: String,
    thumb_path: String,
}

// ✅ Fully async function using Rayon
pub fn generate_thumbnail_async(app: &AppHandle, image_path: &Path, thumb_dir: &Path) {
    let file_name = match image_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return,
    };

    let thumb_path = thumb_dir.join(format!("thumb_{}", file_name));

    if thumb_path.exists() {
        return;
    }

    // ✅ Move owned values into the thread
    let app_clone = app.clone();
    let image_path = image_path.to_path_buf();
    let thumb_path = thumb_path.to_path_buf();

    rayon::spawn_fifo(move || {
        if let Ok(img) = image::open(&image_path) {
            let thumb = img.resize(128, 128, FilterType::Lanczos3);
            if thumb.save(&thumb_path).is_ok() {
                // ✅ Emit event using Tauri 2's `emit`
                let _ = app_clone.emit(
                    "thumbnail_ready",
                    ThumbnailReadyEvent {
                        image_path: image_path.to_string_lossy().to_string(),
                        thumb_path: thumb_path.to_string_lossy().to_string(),
                    },
                );
            }
        }
    });
}
