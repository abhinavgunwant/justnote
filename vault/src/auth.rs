use argon2:: {
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2, Algorithm, Version, Params,
};

use types::VaultError;

use crate::{ files::vault_info::get_vault_info, paths::vault_exists };

fn get_argon<'a>() -> Argon2<'a> {
    Argon2::new(
        Algorithm::Argon2id,// Algorithm: Argon2id
        Version::V0x13,     // Version: 19
        Params::new(
            16384,      // m = 16MB
            8,          // t = 2
            1,          // p = 1
            Some(64)    // Output size in bytes
        ).unwrap()
    )
}

pub fn generate_password_hash(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = get_argon();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => {
            eprintln!("Error while generating password hash: {}", e);

            Err(String::from("Some error occured while storing password."))
        }
    }
}

/// Authenticates access to the vault by verifying the password
///
/// Note: It's okay to not have index
pub fn authenticate_vault(name: &str, password: &str) -> bool {
    if let Err(e) = vault_exists(name) {
        let VaultError::NoIndex(_) = e else {
            return false;
        };
    }

    match get_vault_info(name) {
        Ok(vault_info) => {
            match PasswordHash::new(&vault_info.password) {
                Ok(parsed_hash) => {
                    return get_argon()
                        .verify_password(
                            password.as_bytes(),
                            &parsed_hash
                        )
                        .is_ok();
                }

                Err(e) => { eprintln!("{}", e); }
            }
        }

        Err(e) => { eprintln!("{}", e); }
    }

    return false;
}

