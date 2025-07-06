use freya::prelude::*;

use log::{ error, debug };

use vault::files::{ notes::save_note_to_vault, vault_index::set_vault_index };
use types::{ Note, VaultIndexEntry, VaultIndexEntryType };

use crate::{
    colors::COLOR_DARK_4,
    components::vault_session::{ note_name::NoteName, ActiveArea },
    signals::{
        ACTIVE_AREA, CURRENT_NOTE, VAULT_INDEX, VAULT_NAME, EXPLORER_WIDTH,
        VAULT_KEY, SHOW_EXPLORER,
    },
};

/// Does the "control navigation" part for the editor cursor.
///
/// #### Params
/// - `editor` - the editor
/// - `right` - whether to move right or not (`false` in case of left arrow).
fn cursor_ctrl_nav(editor_mut: &mut Write<'_, RopeEditor>, right: bool) {
    let cursor = editor_mut.cursor();
    let text = editor_mut.to_string();

    let mut new_pos: usize = cursor.pos();
    let mut text_iter = text.chars().skip(new_pos);
    let mut rev_text_iter = text.chars().rev().skip(text.len() - new_pos);
    let mut i: usize = 0;

    // represents the characters at the "head" of the cursor after
    // loop ends.
    let mut head_char: Option<char> = None;

    loop {
        if right {
            if let Some(c) = text_iter.next() {
                println!("--> {}", c);

                if c == ' ' || c == '.' || c == ',' {
                    println!("found target chars!");

                    head_char = Some(c);

                    if c == ' ' || i != 0 {
                        break;
                    }
                }
            } else {
                error!("There was an error peeking the text to the right");
                break;
            }

            new_pos += 1;
        } else {
            if let Some(c) = rev_text_iter.next() {
                println!("--> {}", c);

                if c == ' ' || c == '.' || c == ',' {
                    println!("found target chars!");

                    head_char = Some(c);

                    if c == ' ' || i != 0 {
                        break;
                    }
                }
            } else {
                error!("There was an error peeking the text to the right");
                break;
            }

            if new_pos > 0 {
                new_pos -= 1;
            } else {
                error!("cursor at pos 0 when peeking left");
                break;
            }
        }

        if i > 1000 {
            error!("Max iter condition reached!");
            break;
        }

        i += 1;
    }

    if let Some(c) = head_char {
        println!("previous loop was successful");
        if c != ' ' {
            new_pos -= 1;
        }
    }

    editor_mut.cursor_mut().set(new_pos);
}

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
                if c == "e" || c == "E" { return; }

                if c == "s" || c == "S" {
                    debug!("Save Command!");
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

                        match save_note_to_vault(&vault_name, &new_note, *VAULT_KEY.read()) {
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
                    debug!("New Command!");
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

                    let key = *VAULT_KEY.read();

                    let _ = save_note_to_vault(&vault_name, &note, key);

                    VAULT_INDEX.write().last_id = id;

                    *CURRENT_NOTE.write() = Some(note);
                    return;
                }
            }

            if let Key::ArrowRight = e.data.key.clone() {
                let mut editor_mut = editor.editor_mut().write();

                cursor_ctrl_nav(&mut editor_mut, true);
            }

            if let Key::ArrowLeft = e.data.key.clone() {
                let mut editor_mut = editor.editor_mut().write();

                cursor_ctrl_nav(&mut editor_mut, false);
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
                label { "Ctrl + E: Show/hide explorer" }
            }
        }
    }

    rsx! {
        ScrollView {
            width: if *SHOW_EXPLORER.read() {
                "calc(100% - { EXPLORER_WIDTH })"
            } else {
                "100%"
            },

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

