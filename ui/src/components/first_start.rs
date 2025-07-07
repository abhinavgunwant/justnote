use freya::prelude::*;

use crate::{
    styles::{ PRIMARY_BUTTON, SECONDARY_BUTTON },
    components::create_vault::CreateVault,
};

#[derive(Debug, Default, PartialEq)]
enum FirstStartView {
    /// The first page in the first start view
    #[default]
    Start,

    /// The create vault dialog
    CreateVault
}

#[component]
pub fn FirstStart() -> Element {
    let version = env!("CARGO_PKG_VERSION");
    let mut view = use_signal(|| FirstStartView::default());

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
            main_align: "center",
            cross_align: "center",

            if *view.read() == FirstStartView::Start {
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
                        onclick: move |_| *view.write() = FirstStartView::CreateVault,

                        label { "Get Started" }
                    }
                }
            } else {
                CreateVault {}

                rect {
                    width: "100%",
                    text_align: "center",
                    main_align: "center",
                    cross_align: "center",

                    Button {
                        theme: SECONDARY_BUTTON,
                        onclick: move |_| *view.write() = FirstStartView::Start,

                        label { "Back" }
                    }
                }
            }
        }
    }
}

