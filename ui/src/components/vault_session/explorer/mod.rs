mod note_entry;
mod resize_bar;

use freya::prelude::*;

use crate::{
    components::vault_session::explorer::{
        note_entry::ExplorerNoteEntry,
        resize_bar::ExplorerResizeBar,
    }, signals::{ EXPLORER_WIDTH, VAULT_INDEX, VAULT_NAME }, utils::write_vault_index
};

/// Displays notes and folders on the left.
#[component]
pub fn Explorer() -> Element {
    let platform = use_platform();

    let vault_name = if let Some(vn) = VAULT_NAME.cloned() {
        vn
    } else {
        String::default()
    };

    let vault_name_cloned = vault_name.clone();

    use_effect(move || { write_vault_index(&vault_name_cloned); });

    rsx! {
        rect {
            direction: "horizontal",
            height: "fill",

            rect {
                width: "{ EXPLORER_WIDTH }",
                onmouseenter: move |_| {
                    platform.set_cursor(CursorIcon::Default);
                },

                label { "Vault: { vault_name }" }

                ScrollView {
                    for item in VAULT_INDEX.read().entries.iter() {
                        ExplorerNoteEntry {
                            vault_name: "{vault_name}",
                            note_id: item.id,
                            note_name: item.name.clone(),
                        }
                    }
                }
            }

            ExplorerResizeBar {}
        }
    }
}

