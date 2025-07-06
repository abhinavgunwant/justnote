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

use log::{ debug, error };

use aes::{
    Aes256,
    cipher::{
        consts::U16, generic_array::GenericArray, KeyInit, BlockDecrypt,
        BlockEncrypt,
    },
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
                    error!("{}", e);
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

            debug!("got id: {}", note_id);

            for p in get_relative_note_path(note_id).iter() {
                path.push(p);
            }

            debug!("note path: {:?}", path);

            if let Some(file_path) = path.to_str() {
                return Ok(file_path.to_owned());
            }

            Err(IOError::from(IOErrorKind::InvalidData))
        }

        None => Err(IOError::from(IOErrorKind::NotFound))
    }
}

/// Gets the note from the vault.
pub fn get_note(
    vault_name: &String,
    note_id: u32,
    key: [u8; 32],
) -> Result<Note, IOError> {
    let encrypt = key != [0u8; 32];

    match get_note_path(vault_name, note_id) {
        Ok(path) => {
            match read(path) {
                Ok(bytes) => {
                    if bytes.is_empty() {
                        return Err(IOError::from(IOErrorKind::UnexpectedEof));
                    }

                    if encrypt {
                        return match bytes_to_note(bytes) {
                            Ok(mut note) => {
                                note.title = decrypt_data(note.title.as_slice(), key);
                                note.text = decrypt_data(note.text.as_slice(), key);
                                Ok(note)
                            },
                            Err(e) => Err(e),
                        }
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

/// Divides the bytes into generic arrays of 16 bytes.
///
/// Used when encrypting data.
fn create_blocks(data: &[u8]) -> Vec<GenericArray<u8, U16>> {
    let mut block_vec: Vec<GenericArray<u8, U16>> = vec![];
    let mut last_block: Vec<u8> = Vec::with_capacity(16);

    let mut i = 0;

    while i + 16 < data.len() {
        block_vec.push(*GenericArray::from_slice(&data[i..(i+16)]));
        i += 16;
    }

    if data.len() > 0 && i < data.len() - 1 {
        while i < data.len() {
            last_block.push(data[i]);
            i += 1;
        }

        while last_block.len() < 16 {
            last_block.push(0);
        }

        block_vec.push(*GenericArray::from_slice(last_block.as_slice()));
    }

    block_vec
}

/// Encrypts data using key
fn encrypt_data(data: &[u8], key: [u8; 32]) -> Vec<u8> {
    let cipher = Aes256::new(GenericArray::from_slice(&key));

    let mut blocks_vec = create_blocks(data);
    let blocks: &mut [GenericArray<u8, U16>] = blocks_vec.as_mut_slice();

    cipher.encrypt_blocks(blocks.as_mut());

    let mut result_data: Vec<u8> = Vec::with_capacity(blocks.len() * 16);

    for block in blocks {
        for i in 0..16 {
            result_data.push(block[i]);
        }
    }

    result_data
}

/// Decrypts data using key
fn decrypt_data(data: &[u8], key: [u8; 32]) -> Vec<u8> {
    let cipher = Aes256::new(GenericArray::from_slice(&key));

    let mut blocks_vec = create_blocks(data);
    let blocks: &mut [GenericArray<u8, U16>] = blocks_vec.as_mut_slice();

    cipher.decrypt_blocks(blocks.as_mut());

    let mut result_data: Vec<u8> = Vec::with_capacity(blocks.len() * 16);

    for block in blocks {
        for i in 0..16 {
            result_data.push(block[i]);
        }
    }

    result_data
}

pub fn save_note_to_vault(
    vault_name: &String,
    note: &Note,
    key: [u8; 32],
) -> Result<(), IOError> {
    let encrypt = key != [0u8; 32];

    match get_note_path(vault_name, note.id) {
        Ok(path) => {
            let mut p = PathBuf::from(&path);
            p.pop();

            if !p.exists() {
                match create_dir_all(p) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error creating dirs: {}", e);
                        return Err(e);
                    }
                }
            }

            if encrypt {
                let mut new_note = note.clone();

                new_note.text = encrypt_data(note.text.as_slice(), key);
                new_note.title = encrypt_data(note.title.as_slice(), key);

                return match write(path.as_str(), note_to_bytes(&new_note).as_slice()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                };
            }

            match write(path.as_str(), note_to_bytes(note).as_slice()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }

        Err(e) => Err(e),
    }
}

