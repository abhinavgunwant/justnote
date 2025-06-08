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

