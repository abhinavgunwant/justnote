//! Global signals

use freya::prelude::*;

pub static FIRST_START: GlobalSignal<bool> = Signal::global(|| true);

