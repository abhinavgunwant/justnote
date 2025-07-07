use freya::prelude::*;

use vault::files::vault::create_vault;
use crate::{
    app::View, signals::{ FIRST_START, VAULT_NAME, VIEW },
    styles::{ password_input_theme, PRIMARY_BUTTON },
};

/// Dialog to create a new vault
#[component]
pub fn CreateVault() -> Element {
    let mut vault_name = use_signal(String::default);
    let mut vault_pass = use_signal(String::default);
    let mut error = use_signal(String::default);
    let mut no_password = use_signal(|| false);

    let margin = if *FIRST_START.read() { "0" } else { "40 0 0 0" };

    rsx! {
        label {
            width: "100%",
            text_align: "center",
            font_size: "20",
            margin,

            "Create a Vault"
        }

        if !error.read().is_empty() {
            label {
                width: "100%",
                text_align: "center",
                font_size: "20",
                color: "#ff0000",

                "Error: { error }"
            }
        }

        rect {
            width: "100%",
            main_align: "center",
            cross_align: "center",
            spacing: "8",
            margin: "16 0 0 0",

            Input {
                width: "200",
                value: "{ vault_name }",
                placeholder: "Vault Name",
                auto_focus: true,
                onchange: move |e| vault_name.set(e)
            }

            Input {
                width: "200",
                value: "{ vault_pass }",
                placeholder: "Vault Password",
                mode: InputMode::Hidden('*'),
                theme: password_input_theme(*no_password.read()),
                onchange: move |e| {
                    if !*no_password.read() {
                        vault_pass.set(e);
                    }
                }
            }

            Tile {
                onselect: move |_| {
                    let new_no_password = !*no_password.read();
                    *no_password.write() = new_no_password;
                },

                leading: rsx! {
                    Checkbox {
                        selected: *no_password.read(),
                    }
                },

                label {
                    "Don't encrypt this vault"
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
                            if let Err(msg) = create_vault(
                                vault_name.cloned(),
                                vault_pass.cloned(),
                                true
                            ) {
                                *error.write() = msg;
                            };

                            *FIRST_START.write() = false;
                            *VAULT_NAME.write() = Some(vault_name.cloned());
                            *VIEW.write() = View::VaultSession;
                        },

                        label { "Create Vault" }
                    }
                }
            }
        }
    }
}

