use freya::prelude::*;

use crate::{
    colors::{ COLOR_DARK_0, COLOR_DARK_1 },
    signals::{ VAULT_NAME, VIEW, AUTHENTICATED, CURRENT_NOTE }, app::View,
};

#[component]
pub fn VaultListItem(vault_name: String) -> Element {
    let mut hover = use_signal(|| false);

    let vault_name_cloned = vault_name.clone();

    let onmouseenter = move |_| { *hover.write() = true; };
    let onmouseleave = move |_| { *hover.write() = false; };

    let onpointerup = move |_| {
        if *hover.read() {
            *VAULT_NAME.write() = Some(vault_name_cloned.clone());
            *VIEW.write() = View::VaultSession;
            *AUTHENTICATED.write() = false;
            *CURRENT_NOTE.write() = None;
        }
    };

    let background = if *hover.read() { COLOR_DARK_1 } else { COLOR_DARK_0 };

    rsx! {
        rect {
            onmouseenter,
            onmouseleave,
            onpointerup,
            background,
            width: "100%",
            padding: "8 16",

            label { "{ vault_name }" }
        }
    }
}

