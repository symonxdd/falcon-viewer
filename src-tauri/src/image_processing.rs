use image::{imageops::FilterType, GenericImageView, ImageReader};
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

// Helper function to generate the thumbnails
fn generate_thumbnail(image_path: &str) -> Option<String> {
    let output_dir = Path::new(image_path).parent()?.join("miniatures");
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).ok()?; // Create "miniatures" folder if it doesn't exist
    }

    let file_name = Path::new(image_path).file_name()?.to_str()?;
    let thumbnail_path = output_dir.join(file_name);

    // ✅ If the thumbnail already exists, return its path
    if thumbnail_path.exists() {
        return Some(thumbnail_path.to_string_lossy().to_string());
    }

    // Open the image file and prepare to generate the thumbnail
    let file = File::open(image_path).ok()?;
    let reader = BufReader::new(file);

    // Use ImageReader to read the image and decode it
    let img = match ImageReader::new(reader)
        .with_guessed_format() // Automatically guess the format based on the image content
        .map_err(|e: std::io::Error| -> Box<dyn std::error::Error> { e.into() })
        .and_then(|reader| {
            reader
                .decode()
                .map_err(|e: image::ImageError| -> Box<dyn std::error::Error> { e.into() })
        }) {
        Ok(img) => img,
        Err(_) => return None, // If decoding fails, return None
    };

    // Resize the image to a desired thumbnail size (keeping aspect ratio)
    let thumbnail = img.resize(200, 200, FilterType::Lanczos3); // Example resolution 200x200

    // Optionally crop the image (center crop)
    let (width, height) = thumbnail.dimensions(); // Now this works because GenericImageView is in scope
    let crop_width = std::cmp::min(width, 200);
    let crop_height = std::cmp::min(height, 200);
    let cropped_thumbnail = thumbnail.crop_imm(
        (width - crop_width) / 2,
        (height - crop_height) / 2,
        crop_width,
        crop_height,
    );

    // Save the generated thumbnail to the disk
    let thumbnail_path_str = thumbnail_path.to_str()?;
    cropped_thumbnail.save(thumbnail_path_str).ok()?;

    Some(thumbnail_path_str.to_string()) // Return the path as a string
}

// This function will be invoked by the frontend through Tauri
#[tauri::command(rename_all = "snake_case")]
pub async fn get_images(folder_path: String) -> Vec<(String, Option<String>)> {
    let folder_path = Path::new(&folder_path);
    let mut images_with_thumbnails = Vec::new();

    if folder_path.exists() && folder_path.is_dir() {
        // Read directory and filter for images
        match fs::read_dir(folder_path) {
            Ok(entries) => {
                // Use parallel processing to handle multiple files
                let entries: Vec<_> = entries.filter_map(Result::ok).collect();
                let paths: Vec<PathBuf> = entries
                    .into_iter()
                    .filter_map(|entry| {
                        let path = entry.path();
                        if path.is_file() && is_image(&path) {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect();

                // Process each image in parallel
                images_with_thumbnails = paths
                    .into_par_iter()
                    .map(|path| {
                        let full_path = path.to_str().unwrap_or_default().to_string();
                        let thumbnail_path = generate_thumbnail(&full_path);
                        (full_path, thumbnail_path)
                    })
                    .collect();
            }
            Err(_) => {
                // Handle error reading the directory
                eprintln!("Error reading directory: {}", folder_path.display());
            }
        }
    }

    images_with_thumbnails
}

// Helper function to check if a file is an image (you can expand this as needed)
fn is_image(path: &Path) -> bool {
    let extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp"];
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return extensions.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}
