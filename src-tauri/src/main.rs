#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod downloader;
mod delta;
mod extractor;
mod backup;

use tauri::command;

#[command]
async fn download_patch(url: String, output_path: String, expected_hash: String, use_blake3: bool) -> Result<String, String> {
  match downloader::download_file(&url, &output_path, &expected_hash, use_blake3).await {
      Ok(_) => Ok("Patch downloaded and verified successfully.".into()),
      Err(e) => Err(format!("Failed to download patch: {}", e)),
  }
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![download_patch])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}