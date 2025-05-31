mod components;
mod colors;
mod signals;
mod styles;
mod startup;

use freya::prelude::*;
use colors::COLOR_DARK_0;
use components::{ first_start::FirstStart, vault_session::VaultSession };
use signals::FIRST_START;
use startup::startup;

static INTER: &[u8] = include_bytes!("../../assets/fonts/inter/Inter-VariableFont_opsz,wght.ttf");

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_size(720.0, 480.0)
            .with_font("Inter", INTER)
            .with_title("Just Note")
    );
}

fn app() -> Element {
    startup();

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            background: COLOR_DARK_0,
            color: "#ffffff",
            font_family: "Inter",

            if *FIRST_START.read() {
                FirstStart {}
            } else {
                VaultSession {}
            }
        }
    }
}

