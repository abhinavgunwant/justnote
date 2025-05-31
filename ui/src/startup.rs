use vault::{ is_first_start, files::vault::get_default_vault_name };

use crate::signals::{ VAULT_NAME, FIRST_START };

/// Runs on startup to initialize some global signals.
pub fn startup() {
    println!("startup()");
    if is_first_start() {
        *FIRST_START.write() = true;
    } else {
        *FIRST_START.write() = false;

        if let Ok(vault_name) = get_default_vault_name() {
            *VAULT_NAME.write() = Some(vault_name);
        }
    }
}

