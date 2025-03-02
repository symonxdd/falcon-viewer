mod image_processing;
mod likes;

use crate::likes::LikeState;
use image_processing::delete_image;
use image_processing::process_images;
use likes::{get_liked_images, toggle_like};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .manage(LikeState::default())
    .invoke_handler(tauri::generate_handler![
      process_images,
      delete_image,
      toggle_like,
      get_liked_images
    ]) // Register commands here
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
