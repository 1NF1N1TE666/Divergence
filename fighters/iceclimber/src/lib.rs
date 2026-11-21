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
    let popo = &mut movesets::agent("popo");
    acmd::install(popo);
    opff::install(popo);
    status::install(popo);
    popo.install();

    let nana = &mut movesets::agent("nana");
    acmd::install(nana);
    opff::install(nana);
    status::install(nana);
    nana.install();
}