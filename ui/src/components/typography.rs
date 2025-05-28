use freya::prelude::*;

#[component]
pub fn Center(text: String) -> Element {
    rsx! {
        label {
            width: "100%",
            text_align: "center",
            font_weight: "regular",

            "{text}"
        }
    }
}

