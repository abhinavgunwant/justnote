use std::{
    io::{ Error as IOError, ErrorKind as IOErrorKind },
    convert::{ From, Into },
};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use types::{ VaultIndex, VaultIndexEntry, VaultIndexEntryType };

use crate::generated::vault_index_generated::vault_index::{
    VaultIndex as VaultIndexFB, VaultIndexArgs,
    VaultIndexEntry as VaultIndexEntryFB, VaultIndexEntryArgs,
    VaultIndexEntryType as VaultIndexEntryTypeFB, root_as_vault_index,
};

pub fn vault_index_to_bytes(vault_index: &VaultIndex) -> Vec<u8> {
    let mut fb = FlatBufferBuilder::new();

    let mut entries_vec: Vec<WIPOffset<VaultIndexEntryFB>>
        = Vec::with_capacity(vault_index.entries.len());

    for entry in vault_index.entries.iter() {
        let name = Some(fb.create_string(entry.name.as_str()));

        let entry_type = VaultIndexEntryTypeFB(
            entry.entry_type.clone().into()
        );

        entries_vec.push(VaultIndexEntryFB::create(
            &mut fb,
            &VaultIndexEntryArgs {
                id: entry.id,
                name,
                entry_type,
                parent_folder: entry.parent_folder,
        }));
    }

    let entries = Some(fb.create_vector(entries_vec.as_slice()));

    let vault_index_fb = VaultIndexFB::create(&mut fb, &VaultIndexArgs {
        last_id: vault_index.last_id,
        entries,
    });

    fb.finish(vault_index_fb, None);

    fb.finished_data().to_owned()
}

pub fn bytes_to_vault_index(bytes: Vec<u8>) -> Result<VaultIndex, IOError> {
    if bytes.is_empty() {
        return Err(IOError::new(
            IOErrorKind::UnexpectedEof,
            "Vault Info File Empty"
        ));
    }

    match root_as_vault_index(bytes.as_slice()) {
        Ok(vault_index_fb) => {
            let last_id = vault_index_fb.last_id();
            let mut entries: Vec<VaultIndexEntry>;

            if let Some(entries_fb) = vault_index_fb.entries() {
                entries = Vec::with_capacity(entries_fb.len());

                for i in 0..entries_fb.len() {
                    let entry = entries_fb.get(i);

                    let name = if let Some(_name) = entry.name() {
                        _name.to_owned()
                    } else {
                        String::from("NONAME")
                    };

                    entries.push(VaultIndexEntry {
                        id: entry.id(),
                        name,
                        entry_type: VaultIndexEntryType::from(
                            entry.entry_type().0
                        ),
                        parent_folder: entry.parent_folder(),
                    });
                }
            } else {
                entries = vec![];
            }

            Ok(VaultIndex { last_id, entries })
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

