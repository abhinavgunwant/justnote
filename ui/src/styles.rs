//! Contains common styles

use freya::prelude::*;

use std::borrow::Cow;
use crate::colors::{ COLOR_DARK_3, COLOR_DARK_4 };

pub const PRIMARY_BUTTON: ButtonThemeWith = ButtonThemeWith {
    font_theme: None,
    background: Some(Cow::Borrowed(COLOR_DARK_3)),
    hover_background: Some(Cow::Borrowed(COLOR_DARK_4)),
    border_fill: None,
    focus_border_fill: None,
    shadow: None,
    margin: None,
    corner_radius: Some(Cow::Borrowed("4")),
    width: None,
    height: None,
    padding: Some(Cow::Borrowed("8 12")),
};

pub const SECONDARY_BUTTON: ButtonThemeWith = ButtonThemeWith {
    font_theme: None,
    background: Some(Cow::Borrowed("#dddddd")),
    hover_background: Some(Cow::Borrowed("#eeeeee")),
    border_fill: None,
    focus_border_fill: None,
    shadow: None,
    margin: None,
    corner_radius: Some(Cow::Borrowed("4")),
    width: None,
    height: None,
    padding: Some(Cow::Borrowed("8 12")),
};

pub fn password_input_theme(no_password: bool) -> InputThemeWith {
    let background = if no_password {
        Some(Cow::Borrowed("#aaaaaa"))
    } else {
        None
    };

    InputThemeWith {
        font_theme: None,
        placeholder_font_theme: None,
        background,
        hover_background: None,
        border_fill: None,
        focus_border_fill: None,
        shadow: None,
        margin: None,
        corner_radius: None,
    }
}

