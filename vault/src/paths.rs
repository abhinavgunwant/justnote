use std::path::PathBuf;

use dirs_next::data_local_dir;

use types::VaultError;

pub fn get_local_dir() -> Option<PathBuf> {
    if let Some(mut path) = data_local_dir() {
        path.push("justnote");
        return Some(path);
    }

    None
}

pub fn get_vault_root_dir() -> Option<PathBuf> {
    if let Some(mut path) = get_local_dir() {
        path.push("vaults");
        return Some(path);
    }

    None
}

/// Gets `vault_name`'s directory where `vault_name` is the name of a vault.
pub fn get_vault_dir(vault_name: String) -> Option<PathBuf> {
    if let Some(mut path) = get_vault_root_dir() {
        path.push(vault_name);
        return Some(path);
    }

    None
}

/// Gets the path of the index file of the vault.
pub fn get_vault_index_path(vault_name: &String) -> Option<PathBuf> {
    if let Some(mut path) = get_vault_dir(vault_name.clone()) {
        path.push("index");
        return Some(path);
    }

    None
}

/// Checks if vault exists.
///
/// Does this by checking if a directory with the vault name exists inside the
/// "vaults" directory in the justnote local directory and also checks if
/// "index" and "info" files are also present inside the vault directory.
pub fn vault_exists(name: &str) -> Result<(), VaultError> {
    if let Some(mut dir_path) = get_local_dir() {
        dir_path.push("vaults");
        dir_path.push(name);

        if !dir_path.as_path().exists() {
            return Err(VaultError::DoesNotExist);
        }

        dir_path.push("index");

        if !dir_path.as_path().exists() {
            return Err(VaultError::NoIndex(name.to_owned()));
        }

        dir_path.pop();
        dir_path.push("info");

        if !dir_path.as_path().exists() {
            return Err(VaultError::NoInfo);
        }

        return Ok(());

        // TODO: Show an error that says "vault is corrupted" to the user.
    }

    return Err(VaultError::NoLocalDir);
}

/// Gets the file path to the file that holds the name of the default vault
pub fn get_default_vault_file_path() -> Option<String> {
    if let Some(mut path) = get_local_dir() {
        path.push("default-vault");

        if let Some(path_str) = path.to_str() {
            return Some(String::from(path_str));
        }
    }

    None
}

