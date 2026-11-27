#![deny(deprecated)]
#![allow(unused)]
#![allow(unused_macros)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]
use smash::app::lua_bind::*;
use smash::lua2cpp::{*, L2CFighterCommon};
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use smash::app::*;
use smash::app;
use smash::hash40;
use crate::{
    *,
    ext::*,
    consts::*,
};
use smashline::*;


pub mod opff;
pub mod general_statuses;
pub mod function_hooks;
pub mod divergent_variants;
pub mod modes;

pub fn install() {
    opff::install();
    general_statuses::install();
    function_hooks::install();
    divergent_variants::install();
    modes::install();
}