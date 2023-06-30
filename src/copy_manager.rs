use lnk::ShellLink;

use crate::config::Config;

use std::ffi::OsStr;
use std::fs;
use std::error::Error;
use std::path::PathBuf;

/// Handles all the copying
/// 
/// # Usage
/// ```ignore
/// use effingo::{CopyManager, Config};
/// 
/// let config = match Config::build_from_args(&vec!["".to_string(), "C:/".to_string(), "D:/".to_string()]) {
///     Some(cm) => cm,
///     None => panic!("Arguments are not sufficient"),
/// };
/// 
/// let copy_manager = CopyManager::new(config);
/// copy_manager.run();
/// ```
pub struct CopyManager {
    config: Config,
}

impl CopyManager {
    pub fn new(config: Config) -> CopyManager {
        CopyManager {
            config
        }
    }
}

impl CopyManager {
    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        self.copy_directory(
            &self.config.directory_to_copy.to_string_lossy(),
            &self.config.target_directory.to_string_lossy(),
            false)
    }

    fn copy_directory(&self, path: &str, target: &str, inside_link: bool) -> Result<(), Box<dyn Error>> {
        let _ = fs::create_dir(target);

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let entry_name = path.file_name().unwrap();

            let is_lnk = path.extension().unwrap_or(&OsStr::new("")) == "lnk";
            let is_symlink = path.is_symlink();

            // I don't know why; I don't know if I should;
            // But I do it anyway because it works
            let target_path = match is_lnk && !inside_link {
                true => target.to_string(),
                false => format!("{}/{}", target, entry_name.to_string_lossy()),
            };
    
            if path.is_dir() {
                // Since symlinks act as normal directories/folders I added the or operation to the inside_link
                self.copy_directory(&path.to_string_lossy(), &target_path, inside_link || is_symlink)?;
            } else if is_lnk && inside_link {
                fs::copy(path, target_path)?;
            } else if is_lnk && !inside_link {
                self.copy_lnk(&path.to_string_lossy(), &target_path)?;
            } else if path.is_file() {
                fs::copy(path, target_path)?;
            } else {
                eprintln!("Unrecognised element at {}", path.to_string_lossy());
            }
        }
    
        Ok(())
    }

    fn copy_lnk(&self, link_path: &str, target: &str) -> Result<(), Box<dyn Error>> {
        let link_path = PathBuf::from(link_path);

        // I'll use unwrap because lnk::Error doesn't implement std::error::Error :facepalm:
        let referred_entry_path = ShellLink::open(link_path.clone()).unwrap()
            .link_info()
            .clone().unwrap()
            .local_base_path()
            .clone()
            .unwrap();

        let referred_entry = PathBuf::from(referred_entry_path);
        let referred_entry_name = referred_entry.file_name().unwrap();
        let target_path = format!("{}/{}", target, referred_entry_name.to_string_lossy());

        if referred_entry.is_dir() {
            self.copy_directory(&referred_entry.to_string_lossy(), &target_path, true)?;
        } else if referred_entry.is_file() || referred_entry.is_symlink() {
            fs::copy(referred_entry.to_string_lossy().to_string(), target_path)?;
        } else {
            eprintln!("Shortcut (.lnk) points to unrecognised element {}", referred_entry.to_string_lossy());
        }

        Ok(())
    }

    // Since symbiotic links act as normal directories/files this is not needed
    fn _copy_symlink(&self, link_path: &str, target: &str) -> Result<(), Box<dyn Error>> {
        let link_path = PathBuf::from(link_path);

        // I'll use unwrap because lnk::Error doesn't implement std::error::Error :facepalm:
        let link_path = PathBuf::from(link_path);

        let referred_entry = PathBuf::from(link_path.read_link()?);
        let referred_entry_name = referred_entry.file_name().unwrap();
        let target_path = format!("{}/{}", target, referred_entry_name.to_string_lossy());

        if referred_entry.is_dir() {
            self.copy_directory(&referred_entry.to_string_lossy(), &target_path, true)?;
        } else if referred_entry.is_file() || referred_entry.is_symlink() {
            fs::copy(referred_entry.to_string_lossy().to_string(), target_path)?;
        } else {
            eprintln!("Symbiotic link points to unrecognised element {}", referred_entry.to_string_lossy());
        }

        Ok(())
    }
}