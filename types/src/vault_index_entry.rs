use std::convert::{ From, Into };

#[derive(Debug, Default, PartialEq, Clone)]
pub enum VaultIndexEntryType {
    #[default]
    Note,
    Folder,
}

impl From<u8> for VaultIndexEntryType {
    fn from(num: u8) -> Self {
        match num {
            1 => VaultIndexEntryType::Folder,
            _ => VaultIndexEntryType::Note,
        }
    }
}

impl Into<u8> for VaultIndexEntryType {
    fn into(self) -> u8 {
        match self {
            VaultIndexEntryType::Note => 0,
            VaultIndexEntryType::Folder => 1,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct VaultIndexEntry {
    pub id: u32,
    pub name: String,
    pub entry_type: VaultIndexEntryType,
    pub parent_folder: Option<u32>,
}

impl VaultIndexEntry {
    pub fn new_note(id: u32, name: String) -> Self {
        Self { id, name, ..Default::default() }
    }

    pub fn new_folder(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            entry_type: VaultIndexEntryType::Folder,
            ..Default::default()
        }
    }
}

