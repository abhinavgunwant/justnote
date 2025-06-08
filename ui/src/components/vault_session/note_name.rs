use freya::prelude::*;

use crate::{
    signals::{ CURRENT_NOTE, ACTIVE_AREA },
    colors::COLOR_DARK_4,
    components::vault_session::ActiveArea,
};

#[component]
pub fn NoteName(onchange: EventHandler<String>) -> Element {
    let mut editable = use_editable(|| {
            EditableConfig::new(if let Some(note) = CURRENT_NOTE.cloned() {
                note.title
            } else {
                String::default()
            })
        },
        EditableMode::MultipleLinesSingleEditor
    );

    let mut note_id = use_signal(|| 0);

    let is_active: bool = *ACTIVE_AREA.read() == ActiveArea::NoteName;

    if let Some(note) = CURRENT_NOTE.cloned() {
        if note.id != *note_id.read() as u32 {
            editable.editor_mut().write().set(note.title.as_str());
            *note_id.write() = note.id;
        }
    }

    let onmousedown = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseDown(e.data, 0));
    };

    let onmousemove = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseMove(e.data, 0));
    };

    let onclick = move |_: MouseEvent| {
        editable.process_event(&EditableEvent::Click);
    };

    let onglobalkeydown = move |e: KeyboardEvent| {
        if is_active {
            if let Some(_) = *CURRENT_NOTE.read() {
                editable.process_event(&EditableEvent::KeyDown(e.data));

                onchange.call(editable.editor().peek().to_string());
            }
        }
    };

    let cursor_mode = if is_active { "editable" } else { "readonly" };
    let cursor_reference = editable.cursor_attr();
    let cursor_index = if is_active {
        Some(editable.editor().read().cursor_pos().to_string())
    } else {
        None
    };
    let cursor_id = if is_active { Some("0") } else { None };

    rsx! {
        rect {
            color: "#ffffff",
            width: "fill",
            margin: "0 0 24 0",
            onclick: |_| {
                *ACTIVE_AREA.write() = ActiveArea::NoteName;
            },

            paragraph {
                onmousedown,
                onmousemove,
                onclick,
                onglobalkeydown,
                width: "fill",
                cursor_mode,
                cursor_reference,
                cursor_index,
                cursor_id,
                cursor_color: "{ COLOR_DARK_4 }",
                line_height: "1",
                font_size: "32",
                highlights: editable.highlights_attr(0),

                text { "{ editable.editor() }" }
            }
        }
    }
}

