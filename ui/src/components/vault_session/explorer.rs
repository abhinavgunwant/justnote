use freya::prelude::*;

use crate::{
    colors::COLOR_DARK_2,
    signals::{ VAULT_INDEX, VAULT_NAME, EXPLORER_WIDTH },
    components::vault_session::explorer_note_entry::ExplorerNoteEntry,
};

#[component]
pub fn ExplorerResizeBar() -> Element {
    let mut hovered = use_signal::<bool>(|| false);
    let mut dragging = use_signal::<bool>(|| false);

    let background = if *hovered.read() || *dragging.read() {
       COLOR_DARK_2
    } else {
       "#444444"
    };

    rsx! {
        rect {
            height: "fill",
            width: "3",
            background: "{ background }",
            onmouseenter: move |_| {
                *hovered.write() = true;
            },
            onmouseleave: move |_| {
                *hovered.write() = false;
            },
            onglobalmousemove: move |e| {
                if *dragging.read() {
                    let pos = e.get_screen_coordinates();

                    *EXPLORER_WIDTH.write() = pos.x as u16;
                }
            },
            onmousedown: move |_| {
                *dragging.write() = true;
            },
            onmouseup: move |_| {
                *dragging.write() = false;
            },

            label { "" }
        }
    }
}

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

                rect {
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

