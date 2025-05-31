use std::{ path::PathBuf, fs::{ create_dir_all, write }};

use flexbuffers::FlexbufferSerializer;
use serde::Serialize;

use crate::{ paths::get_vault_dir, types::note::Note };

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

fn get_note_path(vault_name: &String, note_id: u32) -> Result<String, std::io::Error>{
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

            Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
        }

        None => Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }
}

pub fn save_note_to_vault(
    vault_name: &String,
    note: &Note,
) -> Result<(), std::io::Error> {
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
            
            let mut serializer = FlexbufferSerializer::new();

            if let Err(e) = note.serialize(&mut serializer) {
                eprintln!("{}", e);
                return Err(std::io::Error::from(std::io::ErrorKind::Other));
            }

            match write(path.as_str(), serializer.view()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }

        Err(e) => Err(e),
    }
}

