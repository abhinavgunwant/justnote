use std::{ path::PathBuf, fs::File, io::Write };
use serde::Serialize;
use flexbuffers::FlexbufferSerializer;

use crate::{ auth::generate_password_hash, types::vault_info::VaultInfo };

pub fn create_vault_info_file(
    path: &PathBuf, name: &String, password: &String
) -> Result<(), String> {
    let mut info_path_buf = path.clone();
    info_path_buf.push("info");

    let info_path = if let Some(p) = info_path_buf.to_str() { p } else { "" };

    if !info_path.is_empty() {
        return match File::create(info_path) {
            Ok(mut file) => {
                match generate_password_hash(&password) {
                    Ok(pwd) => {
                        let info = VaultInfo {
                            name: name.clone(), password: pwd
                        };
                        let mut serializer = FlexbufferSerializer::new();

                        match info.serialize(&mut serializer) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }

                        match file.write(serializer.view()) {
                            Ok(_) => Ok(()),

                            Err(e) => {
                                eprintln!("{}", e);
                                Err(String::from(""))
                            }
                        }
                    }

                    Err(e) => Err(e),
                }
            }

            Err(e) => {
                eprintln!("{}", e);
                Err(String::from("Couldn't open file"))
            }
        };
    }

    Err(String::from("Invalid path"))
}

