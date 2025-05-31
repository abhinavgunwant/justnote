use freya::prelude::*;

#[component]
pub fn Editor() -> Element {
    let mut editor = use_editable(
        || EditableConfig::new(String::default()),
        EditableMode::MultipleLinesSingleEditor
    );

    let onmousedown = move |e: MouseEvent| {
        editor.process_event(&EditableEvent::MouseDown(e.data, 0));
    };

    let onmousemove = move |e: MouseEvent| {
        editor.process_event(&EditableEvent::MouseMove(e.data, 0));
    };

    let onclick = move |_: MouseEvent| {
        editor.process_event(&EditableEvent::Click);
    };

    let onglobalkeydown = move |e: KeyboardEvent| {
        editor.process_event(&EditableEvent::KeyDown(e.data));
    };

    let onglobalkeyup = move |e: KeyboardEvent| {
        editor.process_event(&EditableEvent::KeyUp(e.data));
    };

    rsx! {
        rect {
            width: "100%",

            paragraph {
                onglobalkeydown,
                onglobalkeyup,
                onmousedown,
                onmousemove,
                onclick,

                text { "{ editor.editor() }" }
            }
        }
    }
}

