#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod opff;
pub mod status;

// articles

mod drcapsule;

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
    Custom { name: "Fast Capsule", special: Special::Neutral, number: 2, statuses: None },
    Custom { name: "Mega Capsule", special: Special::Neutral, number: 3, statuses: None },
    Custom { name: "Shocking Sheet", special: Special::Side, number: 2, statuses: None },
    Custom { name: "Breezy Sheet", special: Special::Side, number: 3, statuses: None },
    Custom { name: "Super Jump", special: Special::Up, number: 2, statuses: None },
    Custom { name: "Ol' One-Two", special: Special::Up, number: 3, statuses: None },
    Custom { name: "Soaring Tornado", special: Special::Down, number: 2, statuses: None },
    Custom { name: "Clothesline Tornado", special: Special::Down, number: 3, statuses: None },
];

const CUSTOM_LISTS: &[(&str, Special)] = &[
    ("param_drcapsule", Special::Neutral),
    ("param_reflector", Special::Side),
];

pub fn install() {
    config::register_customs(*FIGHTER_KIND_MARIOD, "mariod", CUSTOMS);
    config::custom_params(*FIGHTER_KIND_MARIOD, "mariod", CUSTOM_LISTS, include_str!("../../../romfs/fighter/mariod/param/vl.prcxml"));
    let agent = &mut config::agent("mariod");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    drcapsule::install();
}