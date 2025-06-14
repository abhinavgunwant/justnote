mod note;
mod vault_info;

pub use note::Note;
pub use vault_info::VaultInfo;

#[derive(Debug, PartialEq)]
pub enum VaultError {
    /// No default vault or default vault file exists
    NoDefault,

    /// The first line of default vault file is empty
    DefaultFileFirstLineEmpty,

    /// Vault was not found
    DoesNotExist,

    /// No index found for the vault
    NoIndex(String),

    /// No info file found
    NoInfo,

    /// Index for the vault is corrupt
    CorruptIndex,

    /// No local directory was found
    NoLocalDir,

    OSError(String),
}

/// Follows semver
/// Spec versioning is different for note, vault, vault_index,
/// vault_index_entry and vault_info.
#[derive(Debug, Clone)]
pub struct SpecVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Default for SpecVersion {
    /// Default to the latest supported note spec version (0.1.0)
    fn default() -> Self { Self { major: 0, minor: 1, patch: 0 } }
}

