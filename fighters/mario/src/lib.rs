#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod opff;
pub mod status;

// articles

mod fireball;
mod pump;
mod pumpwater;

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
    Custom { name: "Fast Fireball", special: Special::Neutral, number: 2, statuses: None },
    Custom { name: "Fire Orb", special: Special::Neutral, number: 3, statuses: None },
    Custom { name: "Shocking Cape", special: Special::Side, number: 2, statuses: None },
    Custom { name: "Gust Cape", special: Special::Side, number: 3, statuses: None },
    Custom { name: "Super Jump", special: Special::Up, number: 2, statuses: None },
    Custom { name: "Explosive Punch", special: Special::Up, number: 3, statuses: None },
    Custom { name: "Scalding F.L.U.D.D.", special: Special::Down, number: 2, statuses: None },
    Custom { name: "High-Pressure F.L.U.D.D.", special: Special::Down, number: 3, statuses: None },
];

const CUSTOM_LISTS: &[(&str, Special)] = &[
    ("param_fireball", Special::Neutral),
    ("param_reflector", Special::Side),
    ("param_pump", Special::Down),
    ("param_pumpwater", Special::Down),
];

pub fn install() {
    config::register_customs(*FIGHTER_KIND_MARIO, "mario", CUSTOMS);
    config::custom_params(*FIGHTER_KIND_MARIO, "mario", CUSTOM_LISTS, include_str!("../../../romfs/fighter/mario/param/vl.prcxml"));
    let agent = &mut config::agent("mario");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    fireball::install();
    pump::install();
    pumpwater::install();
}