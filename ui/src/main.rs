mod app;
mod components;
mod colors;
mod signals;
mod styles;
mod startup;

use freya::prelude::*;
use app::app;
use config::init_logging;

static INTER: &[u8] = include_bytes!("../../assets/fonts/inter/Inter-VariableFont_opsz,wght.ttf");

fn main() {
    init_logging();

    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_size(720.0, 480.0)
            .with_font("Inter", INTER)
            .with_title("Just Note")
    );
}

