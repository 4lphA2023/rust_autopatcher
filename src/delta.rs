use bsdiff::Bsdiff;
use std::fs::File;
use std::io::{self, Read, Write};

pub fn create_patch(old_file: &str, new_file: &str, patch_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut old = File::open(old_file)?;
    let mut new = File::open(new_file)?;
    let mut patch = File::create(patch_file)?;

    let mut old_data = Vec::new();
    let mut new_data = Vec::new();

    old.read_to_end(&mut old_data)?;
    new.read_to_end(&mut new_data)?;

    Bsdiff::diff(&old_data, &new_data, &mut patch)?;

    Ok(())
}

pub fn apply_patch(old_file: &str, patch_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut old = File::open(old_file)?;
    let mut patch = File::open(patch_file)?;
    let mut output = File::create(output_file)?;

    let mut old_data = Vec::new();
    let mut patch_data = Vec::new();

    old.read_to_end(&mut old_data)?;
    patch.read_to_end(&mut patch_data)?;

    let new_data = Bsdiff::patch(&old_data, &patch_data)?;

    output.write_all(&new_data)?;

    Ok(())
}