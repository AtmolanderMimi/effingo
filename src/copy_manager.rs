use crate::config::Config;

use std::fs;
use std::error::Error;
use std::io::Error as IoError;

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
        self.copy_directory(&self.config.directory_to_copy.to_string_lossy(), &self.config.target_directory.to_string_lossy())
    }

    fn copy_directory(&self, path: &str, target: &str) -> Result<(), Box<dyn Error>> {

        match fs::create_dir(target) {
            Ok(()) => println!("Directory '{}' doesn't exist. It has been created", target),
            Err(_) => println!("Directory '{}' already exists", target),
        };
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let entry_name = path.file_name().unwrap();
            let target_path = format!("{}\\{}", target, entry_name.to_string_lossy());
    
            if path.is_dir() {
                self.copy_directory(&path.to_string_lossy(), &target_path)?;
            } else if path.is_file() {
                fs::copy(path, target_path)?;
            } else if path.is_symlink() {
                self.copy_link(&path.to_string_lossy());
            } else {
                panic!("Unrecognised element at {}", path.to_string_lossy());
            }
        }
    
        Ok(())
    }

    fn copy_link(&self, path: &str) {
        todo!("Funny .lnk file")
    }
}