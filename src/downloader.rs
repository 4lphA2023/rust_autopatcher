use reqwest::Client;
use sha2::{Sha256, Digest};
use blake3::Hasher;
use std::fs::File;
use std::io::{self, Write};
use tokio::io::AsyncWriteExt;
use tokio::fs::OpenOptions;
use futures_util::StreamExt;

pub async fn download_file(url: &str, output_path: &str, expected_hash: &str, use_blake3: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client
    let client = Client::new();

    // Send GET request
    let response = client.get(url).send().await?;

    // Ensure successful response
    if !response.status().is_success() {
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Failed to download file")));
    }

    // Create file to write the downloaded content
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_path)
        .await?;

    // Stream the response and write to file
    let mut stream = response.bytes_stream();
    let mut hasher_sha256 = Sha256::new();
    let mut hasher_blake3 = Hasher::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;

        // Update hash
        if use_blake3 {
            hasher_blake3.update(&chunk);
        } else {
            hasher_sha256.update(&chunk);
        }
    }

    file.flush().await?;

    // Verify hash
    let calculated_hash = if use_blake3 {
        let result = hasher_blake3.finalize();
        hex::encode(result.as_bytes())
    } else {
        let result = hasher_sha256.finalize();
        hex::encode(result)
    };

    if calculated_hash != expected_hash {
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Hash mismatch")));
    }

    Ok(())
}