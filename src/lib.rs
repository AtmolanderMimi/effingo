//! # Effingo
//! 
//! a command line tool for making backups that make sure that the contents of shortcuts are kept with the backup.
//! 
//! ## Usage
//! 
//! Simply use the effingo command and provide it with the `directory to copy` and `the place to copy`:
//! 
//! ```text
//! C:\Users\Admin> effingo backup_dir D:\
//!                         copied -> target
//! ```

pub mod config;
pub mod copy_manager;
pub mod waiting_animation;

use std::{error::Error, fmt::Display};

pub use config::Config;
pub use copy_manager::CopyManager;
pub use waiting_animation::make_animation_thread;

#[derive(Debug)]
/// Newtype over lnk::Error so that we can implement Error on it
enum LnkError {
    Container(lnk::Error),
    Description(String),
}

impl Display for LnkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LnkError::Container(e) => f.write_str(&format!("{:?}", e)),
            LnkError::Description(s) => f.write_str(&format!("{:?}", s)),
        }
    }
}

impl Error for LnkError {}

impl From<lnk::Error> for LnkError {
    fn from(value: lnk::Error) -> Self {
        LnkError::Container(value)
    }
}