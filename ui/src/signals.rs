//! Global signals

use freya::prelude::*;

use vault::is_first_start;

pub static FIRST_START: GlobalSignal<bool> = Signal::global(|| {
    is_first_start()
});

