use freya::prelude::*;

use vault::{
    files::{ notes::save_note_to_vault, vault_index::set_vault_index },
    types::{
        vault_index_entry::{ VaultIndexEntry, VaultIndexEntryType },
        note::Note,
    },
};

use crate::{
    colors::COLOR_DARK_4,
    components::vault_session::{ note_name::NoteName, ActiveArea },
    signals::{ ACTIVE_AREA, CURRENT_NOTE, VAULT_INDEX, VAULT_NAME },
};

#[component]
pub fn Editor() -> Element {
    let mut editor = use_editable(
        || EditableConfig::new(String::default()),
        EditableMode::MultipleLinesSingleEditor
    );

    let is_active: bool = *ACTIVE_AREA.read() == ActiveArea::Editor;

    let onmousedown = move |e: MouseEvent| {
        editor.process_event(&EditableEvent::MouseDown(e.data, 0));
    };

    let onmousemove = move |e: MouseEvent| {
        editor.process_event(&EditableEvent::MouseMove(e.data, 0));
    };

    let onclick = move |_: MouseEvent| {
        *ACTIVE_AREA.write() = ActiveArea::Editor;
        editor.process_event(&EditableEvent::Click);
    };

    let onglobalkeydown = move |e: KeyboardEvent| {
        if !is_active { return; }

        if let Modifiers::CONTROL = e.data.modifiers {
            if let Key::Character(c) = e.data.key.clone() {
                if c == "s" || c == "S" {
                    println!("Save Command!");
                    let Some(vault_name) = VAULT_NAME.cloned() else {
                        return;
                    };

                    if let Some(note) = CURRENT_NOTE.cloned() {
                        match save_note_to_vault(&vault_name, &note) {
                            Ok(()) => {
                                let _ = set_vault_index(&vault_name, &VAULT_INDEX.read());
                            }

                            Err(_) => {}
                        }
                    }
                    return;
                }

                if c == "n" || c == "N" {
                    println!("New Command!");
                    let Some(vault_name) = VAULT_NAME.cloned() else {
                        return;
                    };

                    let id = VAULT_INDEX.read().last_id + 1;

                    let name = String::from("Untitled Note");

                    VAULT_INDEX.write().entries.push(VaultIndexEntry {
                        id,
                        name: name.clone(),
                        entry_type: VaultIndexEntryType::Note,
                        parent_folder: None,
                    });

                    let note = Note::new(id, name, String::default());

                    let _ = save_note_to_vault(&vault_name, &note);

                    VAULT_INDEX.write().last_id = id;

                    *CURRENT_NOTE.write() = Some(note);
                    return;
                }
            }
        }

        if let Some(_) = *CURRENT_NOTE.read() {
            editor.process_event(&EditableEvent::KeyDown(e.data));
        }
    };

    let onglobalkeyup = move |e: KeyboardEvent| {
        if is_active {
            editor.process_event(&EditableEvent::KeyUp(e.data));
        }
    };

    if let None = *CURRENT_NOTE.read() {
        return rsx! {
            rect {
                onglobalkeydown,
                width: "fill",
                height: "fill",
                main_align: "center",
                cross_align: "center",

                label { "Shortcuts" }
                label { "Ctrl + N: New note" }
                label { "Ctrl + S: Save note" }
            }
        }
    }

    rsx! {
        rect {
            width: "100%",
            padding: "24",

            NoteName {}

            paragraph {
                onglobalkeydown,
                onglobalkeyup,
                onmousedown,
                onmousemove,
                onclick,
                width: "100%",
                cursor_mode: "editable",
                cursor_index: editor.editor().read().cursor_pos().to_string(),
                cursor_id: "0",
                cursor_reference: editor.cursor_attr(),
                cursor_color: "{ COLOR_DARK_4 }",
                highlights: editor.highlights_attr(0),
                line_height: "1",

                for line in editor.editor().read().lines() {
                    text { "{ line }" }
                }
            }
        }
    }
}

