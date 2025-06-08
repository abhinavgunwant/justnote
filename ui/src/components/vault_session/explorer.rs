use std::borrow::Cow;

use freya::prelude::*;

use crate::{
    colors::{ COLOR_DARK_1, COLOR_DARK_2 },
    signals::{ VAULT_INDEX, VAULT_NAME },
    components::vault_session::explorer_note_entry::ExplorerNoteEntry,
};

#[component]
pub fn Explorer() -> Element {
    let vault_name = if let Some(vn) = VAULT_NAME.cloned() {
        vn
    } else {
        String::default()
    };

    let ResizeButtonTheme = ButtonThemeWith {
        font_theme: None,
        background: Some(Cow::Borrowed(COLOR_DARK_2)),
        hover_background: Some(Cow::Borrowed(COLOR_DARK_1)),
        border_fill: Some(Cow::Borrowed("none")),
        focus_border_fill: None,
        shadow: None,
        margin: None,
        corner_radius: None,
        width: Some(Cow::Borrowed("3")),
        height: Some(Cow::Borrowed("fill")),
        padding: Some(Cow::Borrowed("8 15")),
    };

    rsx! {
        rect {
            direction: "horizontal",
            height: "fill",

            rect {
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

            Button {
                theme: ResizeButtonTheme.clone(),

                label { "" }
            }
        }
    }
}

