#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod opff;
pub mod status;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use crate::{
    *,
    ext::*,
    consts::*,
};
use smashline::*;

pub fn install() {
    let agent = &mut config::agent("ness");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();
}