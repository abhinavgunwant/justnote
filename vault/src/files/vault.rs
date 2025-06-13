use std::{ fs::{ File, read_to_string }, io::Write };

use types::VaultError;

use crate::{
    files::{
        create_directories, notes::create_vault_notes_directory,
        vault_index::create_vault_index_file,
        vault_info::create_vault_info_file
    },
    paths::{ get_default_vault_file_path, get_local_dir, vault_exists },
};

// Creates the file that holds the name of the default vault.
//
// The file is named "default-vault" and is a text file that should contain
// only a single line containing the name of the default vault's directory
// inside the "vaults" directory.
pub fn create_default_vault_file(name: &str) -> Result<(), String> {
    let Some(default_vault_file_path) = get_default_vault_file_path() else {
        return Err(String::from(
            "Could not find the for the default vault file."
        ));
    };

    match File::create(default_vault_file_path) {
        Ok(mut file) => {
            match file.write(name.as_bytes()) {
                Ok(b) => {
                    if b > 0 {
                        return Ok(());
                    }

                    Err(String::from("No bytes written"))
                }

                Err(e) => {
                    eprintln!("{}", e);

                    Err(String::from("Error while writing file"))
                }
            }
        }

        Err(e) => {
            eprintln!("{}", e);

            Err(String::from("Couldn't open file"))
        }
    }
}

/// Gets the default vault's name from the "default-vault" file.
///
/// If there are more than one lines/entries in the "default-vault" file, only
/// the first line is considered.
///
/// For more information see: [`create_default_vault_file`].
pub fn get_default_vault_name() -> Result<String, VaultError> {
    let Some(default_file_path) = get_default_vault_file_path() else {
        return Err(VaultError::NoDefault);
    };

    match read_to_string(default_file_path.clone()) {
        Ok(file_name) => {
            let vault_name_lines = file_name.split("\n").collect::<Vec<&str>>();

            if vault_name_lines.is_empty() {
                return Err(VaultError::DefaultFileFirstLineEmpty);
            }

            let vault_name = vault_name_lines[0];

            match vault_exists(vault_name) {
                Ok(_) => Ok(String::from(vault_name)),
                Err(e) => Err(e),
            }
        }

        Err(e) => {
            eprintln!("Error when getting the default vault file name: {}", e);
            Err(VaultError::OSError(e.to_string()))
        }
    }
}

/// Creates vault
///
/// Typical vault structure:
///
/// C:\Users\<user>\AppData\Local\justnote\
/// + default-vault
/// + vaults\
///   + <vault-name>\
///     + info
///     + index
///     + notes\
pub fn create_vault(
    name: String, password: String, first_start: bool
) -> Result<(), String> {
    match get_local_dir() {
        Some(mut path) => {
            path.push("vaults");
            path.push(&name);
            println!("got local dir: {}", path.to_str().unwrap());

            if first_start {
                match create_directories(&path) {
                    Ok(()) => { println!("Created directories"); }
                    Err(e) => { return Err(e); }
                }
            }

            if let Err(e) = create_vault_info_file(&path, &name, &password) {
                return Err(format!("{}", e));
            }

            println!("Info file created");

            if let Err(e) = create_vault_index_file(&path) {
                return Err(format!("{}", e));
            }

            println!("Index file created");

            if let Err(e) = create_vault_notes_directory(&path) {
                return Err(format!("{}", e));
            }

            println!("vault notes directory created");

            println!("Creating default vault file");

            match create_default_vault_file(&name) {
                Ok(()) => Ok(()),
                Err(e) => Err(e),
            }
        }

        None => Err(String::from(
            "Could not find local directory! Please edit the config manually!"
        ))
    }
}

