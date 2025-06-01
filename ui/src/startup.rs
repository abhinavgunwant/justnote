use vault::{
    is_first_start,
    files::{ vault::get_default_vault_name, vault_index::get_vault_index },
    types::vault_index::VaultIndex,
};

use crate::signals::{ FIRST_START, VAULT_INDEX, VAULT_NAME };

/// Runs on startup to initialize some global signals.
pub fn startup() {
    println!("startup()");
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
        }
    }
}

