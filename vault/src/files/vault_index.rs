use std::{
    path::PathBuf, fs::{ File, read, write },
    io::{ Error as IOError, ErrorKind as IOErrorKind },
};

use fb::vault_index::{bytes_to_vault_index, vault_index_to_bytes};
use flexbuffers::{ FlexbufferSerializer, Reader };

use types::VaultIndex;

use crate::paths::get_vault_index_path;

pub fn create_vault_index_file(path: &PathBuf) -> Result<(), String> {
    let mut index_path_buf = path.clone();
    index_path_buf.push("index");

    let index_path = if let Some(p) = index_path_buf.to_str() { p } else { "" };

    if !index_path.is_empty() {
        return match File::create(index_path) {
            Ok(_) => Ok(()),

            Err(e) => {
                eprintln!("{}", e);
                Err(String::from("Couldn't create file"))
            }
        };
    }

    Err(String::from("Invalid path"))
}

/// Reads the vault index file, and returns the deserialized object
pub fn get_vault_index(vault_name: &String) -> Result<VaultIndex, IOError> {
    if let Some(path) = get_vault_index_path(vault_name) {
        if let Some(file_path) = path.to_str() {
            return match read(file_path) {
                Ok(bytes) => bytes_to_vault_index(bytes),
                Err(e) => {
                    eprintln!("Other Index error: {}", e);
                    Err(e)
                }
            };
        }

        println!("Invalid Index data");
        return Err(IOError::from(IOErrorKind::InvalidData));
    }

    println!("Index not found");

    Err(IOError::from(IOErrorKind::NotFound))
}

/// Serializes `vault_index` into vault index file.
pub fn set_vault_index(
    vault_name: &String, vault_index: &VaultIndex,
) -> Result<(), IOError> {
    if let Some(path) = get_vault_index_path(&vault_name) {
        let bytes = vault_index_to_bytes(vault_index);

        return match write(path, bytes.as_slice()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }

    Err(IOError::from(IOErrorKind::NotFound))
}

/// Builds entire vault index from scratch
///
/// TODO: Implement it.
/// For now it just builds an empty index
pub fn build_vault_index(vault_name: &String) -> Result<(), IOError> {
    if let Some(_path) = get_vault_index_path(vault_name) {
        return set_vault_index(vault_name, &VaultIndex::default());
    }

    Err(IOError::from(IOErrorKind::NotFound))
}

