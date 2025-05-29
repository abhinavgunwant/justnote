use freya::prelude::*;

use vault::files::vault::create_vault;
use crate::{
    signals::FIRST_START,
    styles::{ PRIMARY_BUTTON, SECONDARY_BUTTON }
};

#[derive(Debug, Default, PartialEq)]
enum View {
    #[default]
    Start,
    CreateVault
}

#[component]
pub fn FirstStart() -> Element {
    let version = env!("CARGO_PKG_VERSION");
    let mut view = use_signal(|| View::default());
    let mut vault_name = use_signal(String::default);
    let mut vault_pass = use_signal(String::default);
    let mut error = use_signal(String::default);

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
            main_align: "center",
            cross_align: "center",

            if *view.read() == View::Start {
                label {
                    width: "100%",
                    text_align: "center",
                    font_size: "30",

                    "Welcome to JustNotes!"
                }

                label {
                    width: "100%",
                    text_align: "center",
                    font_size: "14",
                    margin: "0 0 20 0",

                    "v{ version }"
                }

                label {
                    width: "100%",
                    text_align: "center",
                    margin: "0 0 8 0",

                    "Justnote is a secure, free and open source notes editing and management tool."
                }

                label {
                    width: "100%",
                    text_align: "center",
                    margin: "8 0 8 0",

                    "Notes are stored and organized inside \"Vaults\"."
                }

                label {
                    width: "100%",
                    text_align: "center",
                    margin: "0 0 4 0",

                    "Each \"Vault\" is secured by it's own unique password."
                }

                rect {
                    width: "100%",
                    text_align: "center",
                    main_align: "center",
                    cross_align: "center",
                    margin: "20 0 0 0",

                    Button {
                        theme: PRIMARY_BUTTON,
                        onclick: move |_| *view.write() = View::CreateVault,

                        label { "Get Started" }
                    }
                }
            } else {
                label {
                    width: "100%",
                    text_align: "center",
                    font_size: "20",

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
                        onchange: move |e| vault_name.set(e)
                    }

                    Input {
                        width: "200",
                        value: "{ vault_pass }",
                        placeholder: "Vault Password",
                        mode: InputMode::Hidden('*'),
                        onchange: move |e| vault_pass.set(e)
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
                                },

                                label { "Create Vault" }
                            }
                        }

                        rect {
                            width: "100%",
                            text_align: "center",
                            main_align: "center",
                            cross_align: "center",

                            Button {
                                theme: SECONDARY_BUTTON,
                                onclick: move |_| *view.write() = View::Start,

                                label { "Back" }
                            }
                        }
                    }
                }
            }
        }
    }
}

