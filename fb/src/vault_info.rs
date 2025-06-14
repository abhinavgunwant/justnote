use std::io::{ Error as IOError, ErrorKind as IOErrorKind };
use types::VaultInfo;
use flatbuffers::FlatBufferBuilder;

use crate::generated::vault_info_generated::vault_info::{
    VaultInfo as VaultInfoFB, VaultInfoArgs, root_as_vault_info,
};

pub fn vault_info_to_bytes(vault_info: &VaultInfo) -> Vec<u8> {
    let mut fb = FlatBufferBuilder::new();

    let name = Some(fb.create_string(vault_info.name.as_str()));

    let password = Some(fb.create_string(vault_info.password.as_str()));

    let vault_info = VaultInfoFB::create(&mut fb, &VaultInfoArgs {
        name, password
    });

    fb.finish(vault_info, None);

    fb.finished_data().to_owned()
}

pub fn bytes_to_vault_info(bytes: Vec<u8>) -> Result<VaultInfo, IOError> {
    if bytes.is_empty() {
        return Err(IOError::new(
            IOErrorKind::UnexpectedEof,
            "Vault Info File Empty"
        ));
    }

    match root_as_vault_info(bytes.as_slice()) {
        Ok(vault_info_fb) => {
            let name: String = if let Some(name_fb) = vault_info_fb.name() {
                name_fb.to_owned()
            } else {
                return Err(IOError::new(
                    IOErrorKind::InvalidData,
                    "Note file possibly corrupted"
                ));
            };

            let password: String = if let Some(password_fb) = vault_info_fb.password() {
                password_fb.to_owned()
            } else {
                return Err(IOError::new(
                    IOErrorKind::InvalidData,
                    "Note file possibly corrupted"
                ));
            };

            Ok(VaultInfo { name, password })
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

