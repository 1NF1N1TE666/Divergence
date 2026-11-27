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
use config::{Custom, Special};

const CUSTOMS: &[Custom] = &[
    Custom { name: "Warlock Blade", special: Special::Neutral, number: 2, statuses: Some(status::special_n::blade) },
    Custom { name: "Warlock Thrust", special: Special::Neutral, number: 3, statuses: None },
    Custom { name: "Flame Wave", special: Special::Side, number: 2, statuses: None },
    Custom { name: "Flame Chain", special: Special::Side, number: 3, statuses: None },
    Custom { name: "Dark Fists", special: Special::Up, number: 2, statuses: None },
    Custom { name: "Dark Vault", special: Special::Up, number: 3, statuses: None },
    Custom { name: "Wizard's Dropkick", special: Special::Down, number: 2, statuses: None },
    Custom { name: "Wizard's Assault", special: Special::Down, number: 3, statuses: None },
];

pub fn install() {
    config::register_customs(*FIGHTER_KIND_GANON, "ganon", CUSTOMS);
    let agent = &mut config::agent("ganon");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();
}