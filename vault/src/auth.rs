use argon2:: {
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2, Algorithm, Version, Params,
};

use log::error;

use types::VaultError;

use crate::{ files::vault_info::get_vault_info, paths::vault_exists };

#[derive(Debug, PartialEq)]
pub enum AuthenticationError {
    WrongPassword,
    VaultDoesNotExist,
    VaultIsUnencrypted,
    OtherError,
}

fn get_argon<'a>() -> Argon2<'a> {
    Argon2::new(
        Algorithm::Argon2id,// Algorithm: Argon2id
        Version::V0x13,     // Version: 19
        Params::new(
            16384,      // m = 16MB
            8,          // t = 2
            1,          // p = 1
            Some(32)    // Output size in bytes
        ).unwrap()
    )
}

pub fn generate_password_hash(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = get_argon();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => {
            error!("Error while generating password hash: {}", e);

            Err(String::from("Some error occured while storing password."))
        }
    }
}

/// Authenticates access to the vault by verifying the password
///
/// Note: It's okay to not have index
pub fn authenticate_vault(
    name: &str, password: &str
) -> Result<[u8; 32], AuthenticationError> {
    if let Err(e) = vault_exists(name) {
        let VaultError::NoIndex(_) = e else {
            return Err(AuthenticationError::VaultDoesNotExist);
        };
    }

    match get_vault_info(name) {
        Ok(vault_info) => {
            // debug!("{}", vault_info.password);

            if vault_info.password.is_empty() {
                return Err(AuthenticationError::VaultIsUnencrypted);
            }

            match PasswordHash::new(&vault_info.password) {
                Ok(parsed_hash) => {
                    let argon = get_argon();

                    match argon.verify_password(
                        password.as_bytes(), &parsed_hash
                    ) {
                        Ok(()) => {
                            let mut key: [u8; 32] = [0u8; 32];

                            if let Some(salt) = parsed_hash.salt {
                                if let Err(e) = argon.hash_password_into(
                                    password.as_bytes(),
                                    salt.as_ref().as_bytes(),
                                    &mut key,
                                ) {
                                    error!("Error while hashing password into: {}", e);
                                } else {
                                    return Ok(key);
                                }
                            }
                        }

                        Err(e) => { error!("{}", e); }
                    }

                    return Err(AuthenticationError::WrongPassword);
                }

                Err(e) => { error!("{}", e); }
            }
        }

        Err(e) => { error!("{}", e); }
    }

    Err(AuthenticationError::OtherError)
}

