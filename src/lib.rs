//! # Effingo
//! 
//! a command line tool for making backups that make sure that the contents of shortcuts are kept with the backup.
//! 
//! ## Usage
//! 
//! Simply use the effingo command and provide it with the `directory to copy` and `the place to copy`:
//! 
//! ```
//! C:\Users\Admin> effingo backup_dir D:\
//!                         copied -> target
//! ```

pub mod config;

use std::fs;
use std::error::Error;

/// Copies a directory and its subdirectories to the target directory
/// 
/// # Error
/// will fail if it does not like you :)
pub fn copy_directory(path: &str, target: &str) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let entry_name = path.file_name().unwrap();
        let target_path = format!("{}\\{}", target, entry_name.to_str().unwrap().to_string());

        if path.is_dir() {
            fs::create_dir(&target_path).unwrap();
            copy_directory(&path.to_str().unwrap(), &target_path)?;
        } else if path.is_file() {
            fs::copy(path, target_path)?;
        } else if path.is_symlink() {
            todo!("yay .lnk files");
        }
    }

    Ok(())
}
