use freya::prelude::*;

use vault::{
    files::{ notes::save_note_to_vault, vault_index::set_vault_index },
    types::vault_index_entry::{ VaultIndexEntry, VaultIndexEntryType },
};
use types::Note;

use crate::{
    colors::COLOR_DARK_4,
    components::vault_session::{ note_name::NoteName, ActiveArea },
    signals::{
        ACTIVE_AREA, CURRENT_NOTE, VAULT_INDEX, VAULT_NAME, EXPLORER_WIDTH,
    },
};

#[component]
pub fn Editor() -> Element {
    let platform = use_platform();

    let mut editor = use_editable(
        || EditableConfig::new(String::default()),
        EditableMode::MultipleLinesSingleEditor
    );

    let mut note_id = use_signal(|| 0);
    let mut note_name = use_signal(|| String::default());

    if let Some(note) = CURRENT_NOTE.cloned() {
        if note.id != *note_id.read() as u32 {
            *note_id.write() = note.id;

            if let Ok(title) = note.title() {
                *note_name.write() = title;
            }

            if let Ok(text) = note.text() {
                editor.editor_mut().write().set(&text);
            }
        }
    }

    let is_active: bool = *ACTIVE_AREA.read() == ActiveArea::Editor;

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Text);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::Default);
    };

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
                        let mut new_note = note.clone();
                        new_note.set_title(note_name.cloned());
                        new_note.set_text(editor.editor().peek().to_string());

                        let mut current_index = VAULT_INDEX.cloned();

                        for index_entry in current_index.entries.iter_mut() {
                            if index_entry.id == new_note.id {
                                if let Ok(title) = new_note.title() {
                                    index_entry.name = title;
                                    break;
                                }
                            }
                        }

                        match save_note_to_vault(&vault_name, &new_note) {
                            Ok(()) => {
                                let _ = set_vault_index(&vault_name, &current_index);

                                *VAULT_INDEX.write() = current_index;
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

                    let note = Note::new(id, name, String::default(), false);

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
        ScrollView {
            width: "calc(100% - { EXPLORER_WIDTH })",
            padding: "24",

            NoteName {
                onchange: move |text| { *note_name.write() = text; }
            }

            paragraph {
                onmouseenter,
                onmouseleave,
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

