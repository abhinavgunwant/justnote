use std::{
    path::PathBuf, fs::{ File, read },
    io::{ Error as IOError, ErrorKind as IOErrorKind, Write },
};

use types::VaultInfo;
use fb::vault_info::{ bytes_to_vault_info, vault_info_to_bytes };

use crate::{ auth::generate_password_hash, paths::get_vault_root_dir };

pub fn get_vault_info_path(vault_name: &str) -> String {
    if let Some(mut path) = get_vault_root_dir() {
        path.push(vault_name);
        path.push("info");

        if let Some(path_str) = path.to_str() {
            return path_str.to_owned();
        }
    }

    String::default()
}

/// Creates vault info file with vault name and password.
///
/// Password is hashed before storing to the file.
///
/// Stores empty string as password hash when no password is
/// given
pub fn create_vault_info_file(
    path: &PathBuf, name: &String, password: &String
) -> Result<(), String> {
    let mut info_path_buf = path.clone();
    info_path_buf.push("info");

    let info_path = get_vault_info_path(name);

    if !info_path.is_empty() {
        return match File::create(info_path) {
            Ok(mut file) => {
                let password_hash: String = if !password.is_empty() {
                    match generate_password_hash(&password) {
                        Ok(password) => { password }
                        Err(e) => { return Err(e); },
                    }
                } else {
                    String::default()
                };

                println!("password hash for {} is {}", password, password_hash);

                let info = VaultInfo::new(name.clone(), password_hash);

                match file.write(vault_info_to_bytes(&info).as_slice()) {
                    Ok(_) => Ok(()),

                    Err(e) => {
                        eprintln!("{}", e);
                        Err(String::from("Some issue writing file"))
                    }
                }
            }

            Err(e) => {
                eprintln!("{}", e);
                Err(String::from("Couldn't open file"))
            }
        };
    }

    Err(String::from("Invalid path"))
}

pub fn get_vault_info(vault_name: &str) -> Result<VaultInfo, IOError> {
    let info_file = get_vault_info_path(vault_name);

    if info_file.is_empty() {
        return Err(IOError::new(
            IOErrorKind::NotFound,
            "Vault info file not found."
        ));
    }

    match read(info_file) {
        Ok(bytes) => bytes_to_vault_info(bytes),
        Err(e) => Err(e),
    }
}

