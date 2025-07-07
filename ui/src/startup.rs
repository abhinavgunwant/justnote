use log::{ info, debug };

use vault::is_first_start;
use config::Config;

use crate::{
    signals::{ FIRST_START, VAULT_NAME, SHOW_EXPLORER },
    utils::write_vault_index,
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

            write_vault_index(&vault_name);
        } else {
            debug!("Did not get the default vault name.");
        }

        if config.startup.hide_explorer {
            *SHOW_EXPLORER.write() = false;
        }
    }
}

