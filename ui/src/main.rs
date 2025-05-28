mod components;
mod colors;
mod signals;

use freya::prelude::*;
use colors::COLOR_DARK_0;
use components::first_start::FirstStart;

static INTER: &[u8] = include_bytes!("../../assets/fonts/inter/Inter-VariableFont_opsz,wght.ttf");

fn is_first_start() -> bool {
    println!("is_first_start() called");

    true
}

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
    rsx! {
        rect {
            width: "100%",
            height: "100%",
            background: "{ COLOR_DARK_0 }",
            color: "#ffffff",
            font_family: "Inter",

            if is_first_start() {
                FirstStart {}
            } else {
                label { "Just Note" }
            }
        }
    }
}

