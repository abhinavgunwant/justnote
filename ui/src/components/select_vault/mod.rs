pub mod vault_list_item;

use freya::prelude::*;

use vault::files::vault::get_vault_list;

use crate::{
    components::{
        create_vault::CreateVault,
        select_vault::vault_list_item::VaultListItem,
    },
    signals::VIEW, app::View,
};

#[component]
pub fn SelectVault() -> Element {
    if *VIEW.read() == View::CreateVault {
        return rsx! { CreateVault {} }
    }

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",

            label {
                font_size: "20",
                margin: "0 0 16 0",

                "Select Vault:"
            }

            ScrollView {
                width: "100%",
                height: "240",
                max_width: 200.0,

                for vault in get_vault_list().iter() {
                    VaultListItem {
                        vault_name: "{ vault }",
                    }
                }
            }

            Button {
                onpress: |_| {
                    *VIEW.write() = View::CreateVault;
                },
                label { "New Vault" }
            }
        }
    }
}

