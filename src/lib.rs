use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::path::Path;
use anyhow::*;

pub fn copy_to_output(path: &str, build_type: &str) -> Result<()> {
    let mut options = CopyOptions::new();
    let mut from_path = Vec::new();
    let out_path = format!("target/{}", build_type);

    // Overwrite existing files with same name
    options.overwrite = true;

    from_path.push(path);
    copy_items(&from_path, &out_path, &options)?;

    Ok(())
}

pub fn copy_to_output_path(path: &Path, build_type: &str) -> Result<()> {
    let path_str = path.to_str().expect("Could not convert file path to string");
    copy_to_output(path_str, build_type)?;

    Ok(())
}
