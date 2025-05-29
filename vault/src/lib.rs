pub mod types;
pub mod auth;
pub mod paths;
pub mod files;

use std::path::Path;
use crate::paths::get_local_dir;

pub fn is_first_start() -> bool {
    if let Some(path) = get_local_dir() {
        if let Some(path_str) = path.to_str() {
            return !Path::new(path_str).exists();
        }
    }

    true
}

