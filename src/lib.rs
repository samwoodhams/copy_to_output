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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::path::Path;
    use crate::copy_to_output;

    #[test]
    fn test_file_copied() {
        let file_name = "test_file";

        // Create test file to copy
        File::create(file_name).expect("could not create file");

        // Copy test file to output
        copy_to_output(file_name, "debug").expect("could not copy file");
        assert_eq!(true, Path::new(&format!("target/debug/{}", file_name)).exists());

        // Clean up test file
        fs::remove_file(file_name).expect("failed to cleanup file");
        fs::remove_file(format!("target/debug/{}", file_name)).expect("failed to cleanup file");
    }

    #[test]
    fn test_empty_directory_copied() {
        let dir_name = "empty_test_dir";

        // Create test directory to copy
        fs::create_dir(dir_name).expect("could not create directory");

        // Copy test directory to output
        copy_to_output(dir_name, "debug").expect("could not copy directory");
        assert_eq!(true, Path::new(&format!("target/debug/{}", dir_name)).exists());

        // Clean up test directory
        fs::remove_dir(dir_name).expect("failed to cleanup directory");
        fs::remove_dir(format!("target/debug/{}", dir_name)).expect("failed to cleanup directory");
    }

    #[test]
    fn test_directory_with_subfile_copied() {
        let dir_name = "test_dir";
        let file_name = "test_subfile";

        // Create test directory and test file to copy
        fs::create_dir(dir_name).expect("could not create directory");
        File::create(Path::new(&format!("{}/{}", dir_name, file_name))).expect("could not create file");

        // Copy test directory to output
        copy_to_output(dir_name, "debug").expect("could not copy directory");
        assert_eq!(true, Path::new(&format!("target/debug/{}", dir_name)).exists()
                   && Path::new(&format!("target/debug/{}/{}", dir_name, file_name)).exists());

        // Clean up test directory and test file
        fs::remove_dir_all(dir_name).expect("failed to cleanup directory");
        fs::remove_dir_all(format!("target/debug/{}", dir_name)).expect("failed to cleanup directory");
    }
}
