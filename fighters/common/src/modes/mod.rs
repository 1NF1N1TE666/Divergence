// claude code: modes, roster-wide systems a player turns on in config.toml ([modes], src/config.rs).
// each mode is its own file here; the hooks it needs call into it, and every call checks the mode's
// switch first, so with a mode off the game runs as if it weren't there
use super::*;

pub mod faux_air;
pub mod aura;

pub fn install() {
    aura::install();
}
