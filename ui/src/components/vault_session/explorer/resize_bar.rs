use freya::prelude::*;

use crate::{
    colors::COLOR_DARK_2,
    signals::EXPLORER_WIDTH,
};

/// The vertical bar that is used to resize the explorer.
#[component]
pub fn ExplorerResizeBar() -> Element {
    let mut hovered = use_signal::<bool>(|| false);
    let mut dragging = use_signal::<bool>(|| false);

    let background = if *hovered.read() || *dragging.read() {
       COLOR_DARK_2
    } else {
       "#444444"
    };

    rsx! {
        rect {
            height: "fill",
            width: "3",
            background: "{ background }",
            onmouseenter: move |_| {
                *hovered.write() = true;
            },
            onmouseleave: move |_| {
                *hovered.write() = false;
            },
            onglobalmousemove: move |e| {
                if *dragging.read() {
                    let pos = e.get_screen_coordinates();

                    *EXPLORER_WIDTH.write() = pos.x as u16;
                }
            },
            onmousedown: move |_| {
                *dragging.write() = true;
            },
            onmouseup: move |_| {
                *dragging.write() = false;
            },

            label { "" }
        }
    }
}

