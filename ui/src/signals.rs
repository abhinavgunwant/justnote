//! Global signals

use freya::prelude::*;

use vault::is_first_start;

use types::{ Note, VaultIndex };

use crate::components::vault_session::ActiveArea;

pub static FIRST_START: GlobalSignal<bool> = Signal::global(|| {
    is_first_start()
});

pub static SHOW_EXPLORER: GlobalSignal<bool> = Signal::global(|| true);

/// Vault opened in the current session.
pub static VAULT_NAME: GlobalSignal<Option<String>> = Signal::global(|| None);

pub static CURRENT_NOTE: GlobalSignal<Option<Note>> = Signal::global(|| None);

pub static VAULT_INDEX: GlobalSignal<VaultIndex> = Signal::global(||VaultIndex::default());

pub static ACTIVE_AREA: GlobalSignal<ActiveArea> = Signal::global(|| ActiveArea::Editor);

pub static EXPLORER_WIDTH: GlobalSignal<u16> = Signal::global(|| 200);

/// Represents whether the session has been authenticated for the `VAULT_NAME`
/// or not.
///
/// It is `true` when the sesison is authenticated.
///
/// In case when the vault in current session is not encrypted, it is set to
/// `true` as well.
pub static AUTHENTICATED: GlobalSignal<bool> = Signal::global(|| false);

pub static VAULT_KEY: GlobalSignal<[u8; 32]> = Signal::global(|| [0u8; 32]);

