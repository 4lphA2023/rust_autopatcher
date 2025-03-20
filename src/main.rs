mod downloader;

#[tokio::main]
async fn main() {
    let url = "https://example.com/path/to/file";
    let output_path = "downloaded_file";
    let expected_hash = "your_expected_hash_here";
    let use_blake3 = true; // Set to false for SHA-256

    match downloader::download_file(url, output_path, expected_hash, use_blake3).await {
        Ok(_) => println!("File downloaded and verified successfully."),
        Err(e) => eprintln!("Failed to download file: {}", e),
    }
}