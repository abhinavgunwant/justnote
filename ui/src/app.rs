use freya::prelude::*;

use log::debug;

use crate::{
    colors::COLOR_DARK_0, startup::startup,
    components::{
        first_start::FirstStart, select_vault::SelectVault,
        vault_session::VaultSession,
    },
    signals::{ FIRST_START, SHOW_EXPLORER, VIEW },
};

#[derive(Debug, Default, PartialEq)]
pub enum View {
    #[default]
    VaultSession,
    SelectVault,
    CreateVault,
}

/// Main component/element for this app.
pub fn app() -> Element {
    let mut su = use_signal(||false);

    if !*su.read() {
        startup();
        *su.write() = true;
    }

    let onglobalkeydown = move |e: KeyboardEvent| {
        if let Modifiers::CONTROL = e.data.modifiers {
            if let Key::Character(c) = e.data.key.clone() {
                if c == "e" || c == "E" {
                    debug!("Toggle Explorer Command!");

                    let show = *SHOW_EXPLORER.read();

                    *SHOW_EXPLORER.write() = !show;
                    return;
                }

                if c == "g" || c == "G" {
                    debug!("Change Vault Command!");

                    *VIEW.write() = View::SelectVault;
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
                if *VIEW.read() == View::VaultSession {
                    VaultSession {}
                } else {
                    SelectVault {}
                }
            }
        }
    }
}

