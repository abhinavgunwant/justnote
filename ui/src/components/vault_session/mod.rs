pub mod explorer;
pub mod editor;

use freya::prelude::*;

use explorer::Explorer;
use editor::Editor;

use crate::signals::CURRENT_NOTE;

#[component]
pub fn VaultSession() -> Element {
    let onkeydown = move |e: KeyboardEvent| {
        if let Modifiers::CONTROL = e.data.modifiers {
            if let Key::Character(c) = e.data.key.clone() {
                if c == "n" || c == "N" {
                    println!("New Command!");
                }
            }

            return;
        }
    };

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

