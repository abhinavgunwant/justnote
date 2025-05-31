//! Global signals

use freya::prelude::*;

use vault::{ is_first_start, types::note::Note };

pub static FIRST_START: GlobalSignal<bool> = Signal::global(|| {
    is_first_start()
});

pub static VAULT_NAME: GlobalSignal<Option<String>> = Signal::global(|| None);

pub static CURRENT_NOTE: GlobalSignal<Option<Note>> = Signal::global(|| None);

