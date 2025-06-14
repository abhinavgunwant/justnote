use std::io::{ Error as IOError, ErrorKind as IOErrorKind };
use flatbuffers::FlatBufferBuilder;

use types::{ Note, SpecVersion };

use crate::generated::note_generated::note::{
    Note as NoteFB, NoteArgs, SpecVersion as SpecVersionFB, SpecVersionArgs,
    root_as_note,
};

pub fn note_to_bytes(note: &Note) -> Vec<u8> {
    let mut fb = FlatBufferBuilder::new();

    let spec_version = Some(SpecVersionFB::create(&mut fb, &SpecVersionArgs {
        major: note.spec_version.major,
        minor: note.spec_version.minor,
        patch: note.spec_version.patch,
    }));

    let title = Some(fb.create_vector(note.title.as_slice()));

    let text = Some(fb.create_vector(note.text.as_slice()));

    let note_fb = NoteFB::create(&mut fb, &NoteArgs {
        id: note.id,
        spec_version,
        enc: note.enc,
        title,
        text,
    });

    fb.finish(note_fb, None);

    fb.finished_data().to_owned()
}

pub fn bytes_to_note(bytes: Vec<u8>) -> Result<Note, IOError> {
    if bytes.is_empty() {
        return Err(IOError::new(
            IOErrorKind::UnexpectedEof,
            "Vault Info File Empty"
        ));
    }

    match root_as_note(bytes.as_slice()) {
        Ok(note_fb) => {
            let spec_version = if let Some(specv) = note_fb.spec_version() {
                SpecVersion {
                    major: specv.major(),
                    minor: specv.minor(),
                    patch: specv.patch(),
                }
            } else {
                SpecVersion::default()
            };

            let title: Vec<u8> = if let Some(title_fb) = note_fb.title() {
                title_fb.bytes().to_owned()
            } else {
                Vec::new()
            };

            let text: Vec<u8> = if let Some(text_fb) = note_fb.text() {
                text_fb.bytes().to_owned()
            } else {
                Vec::new()
            };

            Ok(Note {
                id: note_fb.id(),
                spec_version,
                enc: note_fb.enc(),
                title,
                text,
            })
        }

        Err(e) => {
            eprintln!("{}", e);

            Err(IOError::new(
                IOErrorKind::InvalidData,
                "Note file possibly corrupted"
            ))
        }
    }
}

