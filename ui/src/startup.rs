use vault::{
    files::{ vault::get_default_vault_name, vault_index::get_vault_index, vault_info::get_vault_info }, is_first_start
};

use types::VaultIndex;

use crate::signals::{ FIRST_START, VAULT_INDEX, VAULT_NAME, AUTHENTICATED };

/// Runs on startup to initialize some global signals.
pub fn startup() {
    // first start
    if is_first_start() {
        *FIRST_START.write() = true;
    } else {
        *FIRST_START.write() = false;

        if let Ok(vault_name) = get_default_vault_name() {
            *VAULT_NAME.write() = Some(vault_name.clone());

            *VAULT_INDEX.write() = match get_vault_index(&vault_name) {
                Ok(index) => index,
                Err(_e) => VaultIndex::default(),
            };

            if let Ok(info) = get_vault_info(vault_name.as_str()) {
                // is the default vault unencrypted?
                if info.password.is_empty() {
                    *AUTHENTICATED.write() = true;
                }
            }
        }
    }
}

