mod note_entry;
mod resize_bar;

use freya::prelude::*;

use crate::{
    signals::{ VAULT_INDEX, VAULT_NAME, EXPLORER_WIDTH },
    components::vault_session::explorer::{
        note_entry::ExplorerNoteEntry,
        resize_bar::ExplorerResizeBar,
    },
};

/// Displays notes and folders on the left.
#[component]
pub fn Explorer() -> Element {
    let vault_name = if let Some(vn) = VAULT_NAME.cloned() {
        vn
    } else {
        String::default()
    };

    rsx! {
        rect {
            direction: "horizontal",
            height: "fill",

            rect {
                width: "{ EXPLORER_WIDTH }",

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

