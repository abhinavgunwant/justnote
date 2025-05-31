use std::{ path::PathBuf, fs::{ File, read, write } };

use flexbuffers::{ FlexbufferSerializer, Reader };
use serde::{ Deserialize, Serialize };

use crate::{
    paths::get_vault_index_path,
    types::vault_index::VaultIndex,
};

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
pub fn get_vault_index(vault_name: &String) -> Result<VaultIndex, std::io::Error> {
    if let Some(path) = get_vault_index_path(vault_name) {
        if let Some(file_path) = path.to_str() {
            match read(file_path) {
                Ok(bytes) => {
                    if bytes.is_empty() {
                        println!("Vault index is empty!");
                        return Ok(VaultIndex::default());
                    }

                    if let Ok(reader) = Reader::get_root(bytes.as_slice()) {
                        if let Ok(vault_index) = VaultIndex::deserialize(reader) {
                            return Ok(vault_index);
                        }
                    }

                    println!("Other Index error");

                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other, "Other Error"
                    ));
                }

                Err(e) => {
                    eprintln!("Other Index error: {}", e);
                    return Err(e);
                }
            };
        }

        println!("Invalid Index data");
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
    }

    println!("Index not found");

    Err(std::io::Error::from(std::io::ErrorKind::NotFound))
}

/// Serializes `vault_index` into vault index file.
pub fn set_vault_index(
    vault_name: &String, vault_index: &VaultIndex,
) -> Result<(), std::io::Error> {
    if let Some(path) = get_vault_index_path(&vault_name) {
        let mut serializer = FlexbufferSerializer::new();

        if let Err(e) = vault_index.serialize(&mut serializer) {
            eprintln!("{}", e);
        }

        return match write(path, serializer.view()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }

    Err(std::io::Error::from(std::io::ErrorKind::NotFound))
}

/// Builds entire vault index from scratch
///
/// TODO: Implement it.
/// For now it just builds an empty index
pub fn build_vault_index(vault_name: &String) -> Result<(), std::io::Error> {
    if let Some(_path) = get_vault_index_path(vault_name) {
        return set_vault_index(vault_name, &VaultIndex::default());
    }

    Err(std::io::Error::from(std::io::ErrorKind::NotFound))
}

