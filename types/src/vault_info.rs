#[derive(Debug, PartialEq)]
pub struct VaultInfo {
    /// Name of the vault.
    pub name: String,

    /// Vault password.
    /// Can be left blank if vault is un-encrypted.
    pub password: String,
}

impl VaultInfo {
    pub fn new(name: String, password: String) -> Self {
        Self { name, password }
    }
}

