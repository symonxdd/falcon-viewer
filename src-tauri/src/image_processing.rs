use image::{imageops::FilterType, GenericImageView, ImageReader};
use rayon::prelude::*;
use serde::Serialize;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};

// 📢 Event payload structure
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageProcessed {
  file_name: String,
  image_path: Option<String>,
  miniature_path: Option<String>,
}

// 🖼️ Helper function to generate thumbnails
fn generate_thumbnail(image_path: &str) -> Option<String> {
  let output_dir = Path::new(image_path).parent()?.join("miniatures");
  if !output_dir.exists() {
    fs::create_dir_all(&output_dir).ok()?; // Create "miniatures" folder if missing
  }

  let file_name = Path::new(image_path).file_name()?.to_str()?; // Get file name
  let thumbnail_path = output_dir.join(file_name);

  // ✅ Return existing thumbnail path if already generated
  if thumbnail_path.exists() {
    return Some(thumbnail_path.to_string_lossy().to_string());
  }

  // Open and decode the image file
  let file = File::open(image_path).ok()?;
  let reader = BufReader::new(file);
  let img = ImageReader::new(reader)
    .with_guessed_format()
    .ok()?
    .decode()
    .ok()?;

  // Resize to 200x200 while maintaining aspect ratio
  let thumbnail = img.resize(200, 200, FilterType::Lanczos3);
  let (width, height) = thumbnail.dimensions();
  let crop_width = std::cmp::min(width, 200);
  let crop_height = std::cmp::min(height, 200);
  let cropped_thumbnail = thumbnail.crop_imm(
    (width - crop_width) / 2,
    (height - crop_height) / 2,
    crop_width,
    crop_height,
  );

  // Save the generated thumbnail
  let thumbnail_path_str = thumbnail_path.to_str()?;
  cropped_thumbnail.save(thumbnail_path_str).ok()?;

  Some(thumbnail_path_str.to_string()) // ✅ Return path of the new thumbnail
}

// Process images & emit events (does not return anything)
#[tauri::command(rename_all = "snake_case")]
pub async fn process_images(app: AppHandle, folder_path: String) {
  let folder_path = Path::new(&folder_path);

  if folder_path.exists() && folder_path.is_dir() {
    if let Ok(entries) = fs::read_dir(folder_path) {
      // ✅ Collect image paths
      let paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && is_image(path))
        .collect();

      // ✅ Process images in parallel
      paths.into_par_iter().for_each(|path| {
        let full_path = path.to_str().unwrap_or_default().to_string();
        let thumbnail_path = generate_thumbnail(&full_path);

        // 📢 Emit event to frontend after processing each image
        app
          .emit(
            "image-processed",
            ImageProcessed {
              file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
              image_path: Some(full_path.clone()),
              miniature_path: thumbnail_path.clone(),
            },
          )
          .ok(); // ✅ Don't panic if emitting fails
      });
    }
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_image(image_path: String) -> Result<(), String> {
  let image_path = Path::new(&image_path);

  if !image_path.exists() {
    return Err("Image not found".to_string());
  }

  // Try to delete the original image
  if let Err(_) = fs::remove_file(image_path) {
    return Err("Failed to delete the original image".to_string());
  }

  // Try to delete the miniature image if it exists
  let miniature_path = image_path
    .parent()
    .unwrap()
    .join("miniatures")
    .join(image_path.file_name().unwrap());
  if miniature_path.exists() {
    if let Err(_) = fs::remove_file(miniature_path) {
      return Err("Failed to delete the miniature image".to_string());
    }
  }

  Ok(())
}

// 🔍 Helper function to check if a file is an image
fn is_image(path: &Path) -> bool {
  let extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp"];
  path
    .extension()
    .and_then(|ext| ext.to_str())
    .map(|ext_str| extensions.contains(&ext_str.to_lowercase().as_str()))
    .unwrap_or(false)
}
