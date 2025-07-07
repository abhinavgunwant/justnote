use log::{ info, debug };

use config::Config;

use crate::{
    files::{
        create_directories, notes::create_vault_notes_directory,
        vault_index::create_vault_index_file,
        vault_info::create_vault_info_file
    },
    paths::{get_local_dir, get_vault_root_dir},
};

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
            debug!("got local dir: {}", path.to_str().unwrap());

            if first_start {
                match create_directories(&path) {
                    Ok(()) => { debug!("Created directories"); }
                    Err(e) => { return Err(e); }
                }
            }

            if let Err(e) = create_vault_info_file(&path, &name, &password) {
                return Err(format!("{}", e));
            }

            info!("Info file created");

            if let Err(e) = create_vault_index_file(&path) {
                return Err(format!("{}", e));
            }

            info!("Index file created");

            if let Err(e) = create_vault_notes_directory(&path) {
                return Err(format!("{}", e));
            }

            info!("vault notes directory created");

            let mut config = Config::from_config_file();

            config.startup.default_vault = name.clone();
            config.to_config_file();

            info!("Updated '{}' as default vault in config file", name);

            Ok(())
        }

        None => Err(String::from(
            "Could not find local directory! Please edit the config manually!"
        ))
    }
}

pub fn vault_exists(vault_name: String) -> bool {
    debug!("Checking to see if vault {} exists.", vault_name);

    if let Some(mut path) = get_vault_root_dir() {
        path.push(vault_name);

        if !path.exists() {
            debug!("Vault directory does not exist.");

            return false;
        }

        path.push("info");

        if !path.exists() {
            debug!("Vault info file does not exist.");

            return false;
        }

        path.pop();
        path.push("notes");

        return path.exists();
    }

    false
}

pub fn get_vault_list() -> Vec<String> {
    let mut vault_list = vec![];

    if let Some(path) = get_vault_root_dir() {
        if let Ok(read_dir) = path.read_dir() {
            for entry_wrapped in read_dir {
                if let Ok(entry) = entry_wrapped {
                    if entry.path().is_dir() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            vault_list.push(file_name.to_owned());
                        }
                    }
                }
            }
        }
    }

    vault_list
}

