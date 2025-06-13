//! Everything related to the notes files.
//!
//! # How are the note files stored?
//!
//! The note id is of type u32(4 bytes). The note is stored in a path (relative
//! to the "notes" directory) containing the hex-encoded bytes separated by the
//! path separator.
//!
//! Examples:
//! | Note ID    | path (relative to the "notes" directory) |
//! |------------|------------------------------------------|
//! |         1  | 00/00/00/01                              |
//! | 2904774276 | ad/23/56/84                              |
//!
use std::{
    path::PathBuf, fs::{ create_dir_all, write, read },
    io::{ Error as IOError, ErrorKind as IOErrorKind },
};

use fb::note::{ note_to_bytes, bytes_to_note };
use types::Note;

use crate::paths::get_vault_dir;

pub fn create_vault_notes_directory(path: &PathBuf) -> Result<(), String> {
    let mut dir_path_buf = path.clone();
    dir_path_buf.push("notes");

    match dir_path_buf.to_str() {
        Some(p) => {
            match create_dir_all(p) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("{}", e);
                    Err(String::from("Couldn't create notes directory"))
                }
            }
        }

        None => Err(String::from("Invalid path string"))
    }
}

/// Gets the path of the note file relative to the vault directory.
fn get_relative_note_path(note_id: u32) -> Vec<String> {
    let mut path_str_vec: Vec<String> = Vec::with_capacity(4);

    let raw_str = format!("{:08x}", note_id);
    let mut str_chars = raw_str.chars();

    for _ in 0..4 {
        let mut s = String::with_capacity(2);

        if let Some(c) = str_chars.next() { s.push(c); }
        if let Some(c) = str_chars.next() { s.push(c); }

        path_str_vec.push(s);
    }

    path_str_vec
}

fn get_note_path(vault_name: &String, note_id: u32) -> Result<String, IOError>{
    match get_vault_dir(vault_name.clone()) {
        Some(mut path) => {
            path.push("notes");

            println!("got id: {}", note_id);

            for p in get_relative_note_path(note_id).iter() {
                path.push(p);
            }

            println!("note path: {:?}", path);

            if let Some(file_path) = path.to_str() {
                return Ok(file_path.to_owned());
            }

            Err(IOError::from(IOErrorKind::InvalidData))
        }

        None => Err(IOError::from(IOErrorKind::NotFound))
    }
}

/// Gets the note from the vault.
pub fn get_note(vault_name: &String, note_id: u32) -> Result<Note, IOError> {
    match get_note_path(vault_name, note_id) {
        Ok(path) => {
            match read(path) {
                Ok(bytes) => {
                    if bytes.is_empty() {
                        return Err(IOError::from(IOErrorKind::UnexpectedEof));
                    }

                    match bytes_to_note(bytes) {
                        Ok(note) => Ok(note),
                        Err(e) => Err(e),
                    }
                }

                Err(e) => Err(e),
            }
        }

        Err(e) => Err(e),
    }
}

pub fn save_note_to_vault(
    vault_name: &String,
    note: &Note,
) -> Result<(), IOError> {
    match get_note_path(vault_name, note.id) {
        Ok(path) => {
            let mut p = PathBuf::from(&path);
            p.pop();

            if !p.exists() {
                match create_dir_all(p) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error creating dirs: {}", e);
                        return Err(e);
                    }
                }
            }

            match write(path.as_str(), note_to_bytes(note).as_slice()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }

        Err(e) => Err(e),
    }
}

