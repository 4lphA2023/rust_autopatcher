use std::fs;
use std::path::Path;

pub fn create_backup(src: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(dest).exists() {
        fs::remove_dir_all(dest)?;
    }
    fs::create_dir_all(dest)?;
    copy_dir_recursive(src, dest)?;
    Ok(())
}

pub fn restore_backup(src: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(dest).exists() {
        fs::remove_dir_all(dest)?;
    }
    copy_dir_recursive(src, dest)?;
    Ok(())
}

fn copy_dir_recursive(src: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = Path::new(dest).join(entry.file_name());

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir_recursive(&path.to_string_lossy(), &dest_path.to_string_lossy())?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}