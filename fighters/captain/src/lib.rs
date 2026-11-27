#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod opff;
pub mod status;

// articles

mod falconpunch;

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
    Custom { name: "Falcon Dash Punch", special: Special::Neutral, number: 2, statuses: None },
    Custom { name: "Mighty Falcon Punch", special: Special::Neutral, number: 3, statuses: None },
    Custom { name: "Heavy Raptor Boost", special: Special::Side, number: 2, statuses: None },
    Custom { name: "Wind-Up Raptor Boost", special: Special::Side, number: 3, statuses: None },
    Custom { name: "Falcon Strike", special: Special::Up, number: 2, statuses: None },
    Custom { name: "Explosive Falcon Dive", special: Special::Up, number: 3, statuses: None },
    Custom { name: "Falcon Kick Fury", special: Special::Down, number: 2, statuses: None },
    Custom { name: "Lightning Falcon Kick", special: Special::Down, number: 3, statuses: None },
];

pub fn install() {
    config::register_customs(*FIGHTER_KIND_CAPTAIN, "captain", CUSTOMS);
    let agent = &mut config::agent("captain");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    falconpunch::install();
}