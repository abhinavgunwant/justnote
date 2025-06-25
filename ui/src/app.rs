use freya::prelude::*;

use crate::{
    startup::startup, colors::COLOR_DARK_0,
    signals::{ FIRST_START, SHOW_EXPLORER },
    components::{
        first_start::FirstStart,
        vault_session::VaultSession,
    },
};

/// Main component/element for this app.
pub fn app() -> Element {
    startup();

    let onglobalkeydown = move |e: KeyboardEvent| {
        if let Modifiers::CONTROL = e.data.modifiers {
            if let Key::Character(c) = e.data.key.clone() {
                if c == "e" || c == "E" {
                    println!("Toggle Explorer Command!");

                    let show = *SHOW_EXPLORER.read();

                    *SHOW_EXPLORER.write() = !show;
                    return;
                }
            }
        }
    };

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            background: COLOR_DARK_0,
            color: "#ffffff",
            font_family: "Inter",
            onglobalkeydown,

            if *FIRST_START.read() {
                FirstStart {}
            } else {
                VaultSession {}
            }
        }
    }
}

