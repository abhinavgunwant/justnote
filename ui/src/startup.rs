use log::{ info, debug, error };

use vault::{
    files::{ vault_index::get_vault_index, vault_info::get_vault_info },
    is_first_start
};

use types::VaultIndex;
use config::Config;

use crate::signals::{
    AUTHENTICATED, FIRST_START, VAULT_INDEX, VAULT_NAME, SHOW_EXPLORER,
};

/// Runs on startup to initialize some global signals.
pub fn startup() {
    info!("Startup");

    // first start
    if is_first_start() {
        info!("This is the first start!");

        *FIRST_START.write() = true;
    } else {
        let config = Config::from_config_file();

        *FIRST_START.write() = false;

        let vault_name = config.startup.default_vault;

        if !vault_name.is_empty() {
            info!("Default vault: {}", vault_name);

            *VAULT_NAME.write() = Some(vault_name.clone());

            *VAULT_INDEX.write() = match get_vault_index(&vault_name) {
                Ok(index) => index,
                Err(e) => {
                    error!(
                        "Can't get vault index, falling back to default: {}",
                        e
                    );

                    VaultIndex::default()
                }
            };

            if let Ok(info) = get_vault_info(vault_name.as_str()) {
                // is the default vault unencrypted?
                if info.password.is_empty() {
                    *AUTHENTICATED.write() = true;
                }
            }
        } else {
            debug!("Did not get the default vault name.");
        }

        if config.startup.hide_explorer {
            *SHOW_EXPLORER.write() = false;
        }
    }
}

