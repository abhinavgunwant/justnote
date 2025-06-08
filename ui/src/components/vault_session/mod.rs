pub mod explorer;
pub mod editor;
pub mod note_name;
pub mod explorer_note_entry;

use freya::prelude::*;

use explorer::Explorer;
use editor::Editor;

#[derive(Debug, PartialEq)]
pub enum ActiveArea {
    NoteName,
    Editor,
    // None,
}

#[component]
pub fn VaultSession() -> Element {
    rsx! {
        rect {
            width: "100%",
            height: "100%",
            main_align: "start",
            direction: "horizontal",

            Explorer {},

            Editor {},
        }
    }
}

