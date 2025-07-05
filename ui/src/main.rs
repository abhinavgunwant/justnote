mod app;
mod components;
mod colors;
mod signals;
mod styles;
mod startup;
mod logging;

use freya::prelude::*;
use app::app;

static INTER: &[u8] = include_bytes!("../../assets/fonts/inter/Inter-VariableFont_opsz,wght.ttf");

fn main() {
    logging::init_logging();

    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_size(720.0, 480.0)
            .with_font("Inter", INTER)
            .with_title("Just Note")
    );
}

