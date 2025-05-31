pub mod explorer;
pub mod editor;

use freya::prelude::*;

use explorer::Explorer;
use editor::Editor;

#[component]
pub fn VaultSession() -> Element {
    rsx! {
        rect {
            width: "100%",
            height: "100%",
            main_align: "start",
            direction: "horizontal",
            onkeydown: |e| {
                println!("key down: {:?}", e);
            },

            Explorer {},

            Editor {},
        }
    }
}

