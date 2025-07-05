//! Module for all the code that creates or reads files or directories.

pub mod vault;
pub mod notes;
pub mod vault_info;
pub mod vault_index;

use std::{ fs::create_dir_all, path::PathBuf };

use log::error;

pub fn create_directories(path: &PathBuf) -> Result<(), String> {
    if let Some(p) = path.to_str() {
        match create_dir_all(p) {
            Ok(_) => { return Ok(()); }

            Err(e) => {
                error!("{}", e);
                return Err(String::from("Error creating the directories"));
            }
        }
    }

    Err(String::from("Error in path name"))
}

