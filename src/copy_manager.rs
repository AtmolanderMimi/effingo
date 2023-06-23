use crate::config::Config;

use std::ffi::OsStr;
use std::fs;
use std::error::Error;
use std::io::Error as IoError;
use std::path::PathBuf;

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
            let target_path = format!("{}\\{}", target, entry_name.to_string_lossy());

            let is_link = path.extension().unwrap_or(&OsStr::new("")) == ".lnk";
    
            if path.is_dir() {
                self.copy_directory(&path.to_string_lossy(), &target_path, inside_link)?;
            } else if is_link && !inside_link {
                println!("Called");
                self.copy_link(&path.to_string_lossy(), &target_path)?;
            } else if is_link && inside_link {
                println!("Not called");
                fs::copy(path, target_path)?;
            } else if path.is_file() {
                fs::copy(path, target_path)?;
            } else {
                panic!("Unrecognised element at {}", path.to_string_lossy());
            }
        }
    
        Ok(())
    }

    fn copy_link(&self, link_path: &str, target: &str) -> Result<(), Box<dyn Error>> {
        let link_path = PathBuf::from(link_path);
        
        let refered_entry = link_path.read_link()?;
        let refered_entry_name = refered_entry.file_name().unwrap();
        let target_path = format!("{}\\{}", target, refered_entry_name.to_string_lossy());

        if refered_entry.is_dir() {
            self.copy_directory(&refered_entry.to_string_lossy(), &target_path, true)?;
        } else if refered_entry.is_file() || refered_entry.is_symlink() {
            fs::copy(refered_entry.to_string_lossy().to_string(), target_path)?;
        } else {
            panic!("Link points to unrecognised element {}", refered_entry.to_string_lossy());
        }

        Ok(())
    }
}