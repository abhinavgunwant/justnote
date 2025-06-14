#[derive(Debug, Default, PartialEq, Clone)]
pub enum VaultIndexEntryType {
    #[default]
    Note,
    Folder,
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

