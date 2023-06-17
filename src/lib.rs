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