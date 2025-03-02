use dirs::data_local_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tauri::{AppHandle, State};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct LikeState {
  pub liked_images: Mutex<HashSet<String>>,
}

#[derive(Serialize, Deserialize)]
struct LikeStorage {
  images: HashSet<String>,
}

// 📂 Get the correct `likes.json` path
fn get_likes_file_path(app: &AppHandle) -> PathBuf {
  let data_local_dir = data_local_dir().expect("Failed to resolve the data local directory");
  let app_name = app.package_info().name.to_string();

  let app_folder = data_local_dir.join(app_name);
  std::fs::create_dir_all(&app_folder).expect("Failed to create app folder");

  app_folder.join("likes.json")
}

// 📂 Load liked images from `likes.json`
pub fn load_liked_images(app: &AppHandle) -> HashSet<String> {
  let file_path = get_likes_file_path(app);
  if file_path.exists() {
    if let Ok(file) = File::open(&file_path) {
      if let Ok(storage) = serde_json::from_reader::<_, LikeStorage>(BufReader::new(file)) {
        return storage.images;
      }
    }
  }
  HashSet::new()
}

// 💾 Save liked images to `likes.json`
fn save_liked_images(app: &AppHandle, likes: &HashSet<String>) {
  let file_path = get_likes_file_path(app);
  let storage = LikeStorage {
    images: likes.clone(),
  };

  if let Ok(file) = File::create(file_path) {
    if let Err(err) = serde_json::to_writer_pretty(file, &storage) {
      eprintln!("Failed to save liked images: {}", err);
    }
  }
}

// ❤️ Toggle like status of an image
#[tauri::command(rename_all = "snake_case")]
pub async fn toggle_like(
  app: AppHandle,
  image_path: String,
  state: State<'_, LikeState>,
) -> Result<bool, String> {
  let mut liked_images = state.liked_images.lock().await;
  let is_now_liked = if liked_images.contains(&image_path) {
    liked_images.remove(&image_path);
    false
  } else {
    liked_images.insert(image_path.clone());
    true
  };

  save_liked_images(&app, &liked_images);
  Ok(is_now_liked)
}

// 🔄 Get all liked images
#[tauri::command(rename_all = "snake_case")]
pub async fn get_liked_images(app: AppHandle) -> Result<Vec<String>, String> {
  let liked_images = load_liked_images(&app);
  Ok(liked_images.iter().cloned().collect())
}
