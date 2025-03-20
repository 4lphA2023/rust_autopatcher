mod downloader;
mod delta;

#[tokio::main]
async fn main() {
    let url = "https://example.com/path/to/patch";
    let patch_path = "downloaded_patch";
    let expected_hash = "your_expected_patch_hash_here";
    let use_blake3 = true; // Set to false for SHA-256

    match downloader::download_file(url, patch_path, expected_hash, use_blake3).await {
        Ok(_) => println!("Patch downloaded and verified successfully."),
        Err(e) => eprintln!("Failed to download patch: {}", e),
    }

    let old_file = "path/to/old_file";
    let new_file = "path/to/new_file";

    match delta::apply_patch(old_file, patch_path, new_file) {
        Ok(_) => println!("Patch applied successfully."),
        Err(e) => eprintln!("Failed to apply patch: {}", e),
    }
}