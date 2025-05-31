use freya::prelude::*;

use crate::signals::{ VAULT_NAME, CURRENT_NOTE };

use vault::{
    files::{notes::save_note_to_vault, vault_index::get_vault_index},
    types::{
        note::Note, vault_index::VaultIndex,
        vault_index_entry::{ VaultIndexEntry, VaultIndexEntryType },
    }
};

#[component]
pub fn Explorer() -> Element {
    let mut vault_index = use_signal(||
        match VAULT_NAME.cloned() {
            Some(vault_name) => {
                match get_vault_index(&vault_name) {
                    Ok(index) => index,
                    Err(_e) => VaultIndex::default(),
                }
            }

            None => VaultIndex::default(),
        }
    );

    let vault_name = if let Some(vn) = VAULT_NAME.cloned() {
        vn
    } else {
        String::default()
    };

    rsx! {
        rect {
            label { "Vault: { vault_name }" }

            rect {
                Button {
                    onclick: move |_e| {
                        let Some(vault_name) = VAULT_NAME.cloned() else {
                            return;
                        };

                        let id = vault_index.read().last_id + 1;

                        let name = String::from("Untitled Note");

                        vault_index.write().entries.push(VaultIndexEntry {
                            id,
                            name: name.clone(),
                            entry_type: VaultIndexEntryType::Note,
                            parent_folder: None,
                        });

                        let note = Note::new(id, name, String::default());

                        let _ = save_note_to_vault(&vault_name, &note);

                        *CURRENT_NOTE.write() = Some(note);
                    },

                    label { "New Note" }
                }

                Button {
                    onclick: move |_e| {
                        let Some(vault_name) = VAULT_NAME.cloned() else {
                            return;
                        };

                        if let Some(note) = CURRENT_NOTE.cloned() {
                            let _ = save_note_to_vault(&vault_name, &note);
                        }
                    },

                    label { "Save Note" }
                }
            }

            rect {
                for item in vault_index.read().entries.iter() {
                    label { "{ item.name }" }
                }
            }
        }
    }
}

