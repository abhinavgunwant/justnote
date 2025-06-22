use freya::prelude::*;

use vault::files::notes::get_note;

use crate::{
    colors::{ COLOR_DARK_0, COLOR_DARK_1 },
    signals::{ CURRENT_NOTE, EXPLORER_WIDTH, VAULT_KEY },
};

/// The explorer note entry.
///
/// Used to show the list of notes in the explorer.
///
/// Note: I planned on using the `Button` component from freya, but it did not
/// support left aligned text which I needed for my component.
#[component]
pub fn ExplorerNoteEntry(
    vault_name: String,
    note_id: u32,
    note_name: String
) -> Element {
    let mut button_hover = use_signal(|| false);

    let onmouseenter = move |_| { *button_hover.write() = true; };

    let onmouseleave = move |_| { *button_hover.write() = false; };

    let onpointerup = move |_| {
        if *button_hover.read() {
            if let Ok(note) = get_note(&vault_name, note_id, *VAULT_KEY.read()) {
                *CURRENT_NOTE.write() = Some(note);
            }
        }
    };

    let background = if *button_hover.read() {
        COLOR_DARK_1
    } else {
        COLOR_DARK_0
    };

    rsx! {
        rect {
            onmouseenter,
            onmouseleave,
            onpointerup,
            background: "{ background }",
            color: "#ffffff",
            width: "{ EXPLORER_WIDTH }",
            padding: "8",

            label {
                text_align: "left",

                "{ note_name }"
            }
        }
    }
}

