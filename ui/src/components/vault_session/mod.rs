pub mod explorer;
pub mod editor;
pub mod note_name;

use freya::prelude::*;

use log::{ info, error, debug };

use vault::{auth::{ authenticate_vault, AuthenticationError }, files::vault_info::get_vault_info};

use crate::{
    signals::{ AUTHENTICATED, VAULT_NAME, VAULT_KEY, SHOW_EXPLORER, VIEW },
    styles::{ password_input_theme, PRIMARY_BUTTON, SECONDARY_BUTTON },
    components::vault_session::{ explorer::Explorer, editor::Editor },
    app::View,
};

#[derive(Debug, PartialEq)]
pub enum ActiveArea {
    NoteName,
    Editor,
}

#[component]
pub fn VaultSession() -> Element {
    let mut password = use_signal(String::default);

    let vault_name = if let Some(v) = VAULT_NAME.read().clone() {
        v.clone()
    } else {
        String::default()
    };

    let vault_name_cloned = vault_name.clone();

    use_effect(move || {
        if let Ok(info) = get_vault_info(vault_name_cloned.as_str()) {
            // is the vault unencrypted? make it authenticated if yes.
            if info.password.is_empty() {
                debug!("Vault is unencrypted, marking it authenticated");
                *AUTHENTICATED.write() = true;
            }
        }
    });

    if !*AUTHENTICATED.read() {
        debug!("Vault is unauthenticated");

        return rsx! {
            rect {
                width: "100%",
                height: "100%",
                main_align: "center",
                cross_align: "center",
                spacing: "8",
                margin: "16 0 0 0",

                label { "Current Vault: { vault_name }" }

                text { "Enter password for this vault in order to proceed" }

                Input {
                    width: "200",
                    value: "{ password }",
                    placeholder: "Vault Password",
                    mode: InputMode::Hidden('*'),
                    theme: password_input_theme(false),
                    onchange: move |e| {
                        password.set(e);
                    }
                }

                rect {
                    direction: "vertical",
                    spacing: "4",
                    margin: "20 0 0 0",

                    rect {
                        width: "100%",
                        text_align: "center",
                        main_align: "center",
                        cross_align: "center",

                        Button {
                            theme: PRIMARY_BUTTON,

                            onclick: move |_| {
                                match authenticate_vault(
                                    vault_name.as_str(),
                                    (*password.read()).as_str(),
                                ) {
                                    Ok(key) => {
                                        *VAULT_KEY.write() = key;
                                        *AUTHENTICATED.write() = true;
                                    }

                                    Err(e) => {
                                        if e == AuthenticationError::VaultIsUnencrypted {
                                            *AUTHENTICATED.write() = true;
                                            *VAULT_KEY.write() = [0u8; 32];
                                        }
                                        error!("{:?}", e);
                                        info!("password did not match");
                                    }
                                }
                            },

                            label { "Proceed" }
                        }
                    }

                    rect {
                        width: "100%",
                        text_align: "center",
                        main_align: "center",
                        cross_align: "center",

                        Button {
                            theme: SECONDARY_BUTTON,
                            onclick: move |_| *VIEW.write() = View::SelectVault,

                            label { "Change Vault" }
                        }
                    }
                }
            }
        };
    }

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            main_align: "start",
            direction: "horizontal",

            if *SHOW_EXPLORER.read() {
                Explorer {}
            }

            Editor {},
        }
    }
}

