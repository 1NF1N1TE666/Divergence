#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod opff;
pub mod status;

// articles

mod fireball;

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
    Custom { name: "Bouncing Fireball", special: Special::Neutral, number: 2, statuses: None },
    Custom { name: "Iceball", special: Special::Neutral, number: 3, statuses: None },
    Custom { name: "Floating Missile", special: Special::Side, number: 2, statuses: None },
    Custom { name: "Quick Missile", special: Special::Side, number: 3, statuses: None },
    Custom { name: "Fiery Jump Punch", special: Special::Up, number: 2, statuses: None },
    Custom { name: "Burial Header", special: Special::Up, number: 3, statuses: None },
    Custom { name: "Mach Cyclone", special: Special::Down, number: 2, statuses: None },
    Custom { name: "Clothesline Cyclone", special: Special::Down, number: 3, statuses: None },
];

const CUSTOM_LISTS: &[(&str, Special)] = &[
    ("param_fireball", Special::Neutral),
];

pub fn install() {
    config::register_customs(*FIGHTER_KIND_LUIGI, "luigi", CUSTOMS);
    config::custom_params(*FIGHTER_KIND_LUIGI, "luigi", CUSTOM_LISTS, include_str!("../../../romfs/fighter/luigi/param/vl.prcxml"));
    let agent = &mut config::agent("luigi");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    fireball::install();
}
