mod downloader;
mod delta;
mod extractor;
mod backup;

#[tokio::main]
async fn main() {
    let patch_url = "https://example.com/path/to/patch.zip";
    let patch_path = "downloaded_patch.zip";
    let expected_hash = "your_expected_patch_hash_here";
    let use_blake3 = true; // Set to false for SHA-256
    let backup_dir = "backup";
    let current_dir = "current";
    let output_dir = "output";

    // Download the patch
    match downloader::download_file(patch_url, patch_path, expected_hash, use_blake3).await {
        Ok(_) => println!("Patch downloaded and verified successfully."),
        Err(e) => {
            eprintln!("Failed to download patch: {}", e);
            return;
        }
    }

    // Create a backup of the current files
    match backup::create_backup(current_dir, backup_dir) {
        Ok(_) => println!("Backup created successfully."),
        Err(e) => {
            eprintln!("Failed to create backup: {}", e);
            return;
        }
    }

    // Extract the patch
    match extractor::extract_zip(patch_path, output_dir) {
        Ok(_) => println!("Patch extracted successfully."),
        Err(e) => {
            eprintln!("Failed to extract patch: {}", e);
            // Restore the backup in case of failure
            if let Err(err) = backup::restore_backup(backup_dir, current_dir) {
                eprintln!("Failed to restore backup: {}", err);
            }
            return;
        }
    }

    // Apply the patch
    let old_file = "current/old_file";
    let new_file = "output/new_file";

    match delta::apply_patch(old_file, patch_path, new_file) {
        Ok(_) => println!("Patch applied successfully."),
        Err(e) => {
            eprintln!("Failed to apply patch: {}", e);
            // Restore the backup in case of failure
            if let Err(err) = backup::restore_backup(backup_dir, current_dir) {
                eprintln!("Failed to restore backup: {}", err);
            }
        }
    }
}