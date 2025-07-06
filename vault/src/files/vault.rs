use log::{ info, debug, error };

use config::Config;

use crate::{
    files::{
        create_directories, notes::create_vault_notes_directory,
        vault_index::create_vault_index_file,
        vault_info::create_vault_info_file
    },
    paths::get_local_dir,
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

