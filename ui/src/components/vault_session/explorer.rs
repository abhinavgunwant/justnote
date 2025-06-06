use std::borrow::Cow;

use freya::prelude::*;

use crate::{
    colors::{ COLOR_DARK_0, COLOR_DARK_1, COLOR_DARK_2 },
    signals::{ VAULT_INDEX, VAULT_NAME }
};

#[component]
pub fn Explorer() -> Element {
    let vault_name = if let Some(vn) = VAULT_NAME.cloned() {
        vn
    } else {
        String::default()
    };

    let ExplorerButtonTheme = ButtonThemeWith {
        font_theme: Some(FontThemeWith {
            color: Some(Cow::Borrowed("white")),
        }),
        background: Some(Cow::Borrowed(COLOR_DARK_0)),
        hover_background: Some(Cow::Borrowed(COLOR_DARK_1)),
        border_fill: Some(Cow::Borrowed("none")),
        focus_border_fill: None,
        shadow: None,
        margin: None,
        corner_radius: None,
        width: None,
        height: None,
        padding: Some(Cow::Borrowed("8 15")),
    };

    let ResizeButtonTheme = ButtonThemeWith {
        font_theme: None,
        background: Some(Cow::Borrowed(COLOR_DARK_2)),
        hover_background: Some(Cow::Borrowed(COLOR_DARK_1)),
        border_fill: Some(Cow::Borrowed("none")),
        focus_border_fill: None,
        shadow: None,
        margin: None,
        corner_radius: None,
        width: Some(Cow::Borrowed("3")),
        height: Some(Cow::Borrowed("fill")),
        padding: Some(Cow::Borrowed("8 15")),
    };

    rsx! {
        rect {
            direction: "horizontal",
            height: "fill",

            rect {
                label { "Vault: { vault_name }" }

                rect {
                    for item in VAULT_INDEX.read().entries.iter() {
                        Button {
                            theme: ExplorerButtonTheme.clone(),

                            label { "{ item.name }" }
                        }
                    }
                }
            }

            Button {
                theme: ResizeButtonTheme.clone(),

                label { "" }
            }
        }
    }
}

