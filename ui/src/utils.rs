use log::error;

use vault::files::vault_index::get_vault_index;
use types::VaultIndex;

use crate::signals::VAULT_INDEX;

pub fn write_vault_index(vault_name: &String) {
    if !vault_name.is_empty() {
        *VAULT_INDEX.write() = match get_vault_index(vault_name) {
            Ok(index) => index,
            Err(e) => {
                error!(
                    "Can't get vault index, falling back to default: {}",
                    e
                );

                VaultIndex::default()
            }
        };
    }
}

