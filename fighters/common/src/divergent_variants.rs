// claude code: shared code for the divergent variants. each variant is a Dionysus clone of its base
// carrying another fighter's sword gimmick, while the base fighter stays as it is. Dionysus falls back to
// the base's files for anything a variant doesn't ship, so its romfs (romfs/fighter/<name>/) only holds
// what it changes. without Dionysus installed, registration fails quietly and no variant exists. a player
// can turn any variant off in config.toml ([variants], src/config.rs): it's then never registered, so it
// never reaches the select screen or takes up a fighter slot
//
// a variant is its own fighter folder (fighters/<base>_<donor>, behind a feature of the same name), laid
// out like Dionysus' fighter template (ssbu_dionysus, custom_fighters/template_v2) with the names our
// other fighters use: its acmd.rs is acmd/, status.rs is status/, frame.rs is opff.rs, hooks.rs is
// vtable.rs, and params.rs and hud.rs keep theirs. its lib.rs declares it the template's way:
//     fighter!(MARTH_REFLET, "marth_reflet");
//     divergent_variants::register(
//         &MARTH_REFLET,
//         Manifest::new("marth_reflet", "marth") // the variant, its base
//             .costumes(8)
//             .series("fireemblem")              // ui_series_<x>, its group on the select screen
//             .disp_order(22)                    // the base's own number; left unset, it's hidden
//             .hud("reflet"),                    // whose HUD to borrow, if any
//         Some(acmd::rewrite)                    // None if its own scripts are all it needs
//     );
// then installs an agent under its own name (Agent::new("marth_reflet")). its state goes in VarModule
// like any fighter's: that runs from a common status, so clones have it too
use super::*;
use dionysus_api::v2::{Fighter, Manifest};

// the script commands a variant can change in place, whichever script runs them: hitboxes, sword trails,
// effects and sounds. the arguments come in the order smash_script's macros push them
#[derive(Clone, Copy, PartialEq)]
pub enum Command {
    Attack,        // ATTACK: 3 the damage, 8 the size, 35 the region (ATTACK_REGION_SWORD for a sword)
    AfterImage,    // AFTER_IMAGE4_ON: 0 and 1 the trail textures, 12 the flare effect on the blade
    EffectFollow,  // EFFECT_FOLLOW: 0 the effect
    EffectOffKind, // EFFECT_OFF_KIND: 0 the effect
    PlaySe,        // PLAY_SE: 0 the sound
}

pub type Rewrite = unsafe fn(*mut BattleObjectModuleAccessor, Command, &mut Vec<L2CValue>);

// every registered variant that changes commands
static mut VARIANTS: Vec<(&'static Fighter, Rewrite)> = Vec::new();

pub fn register(variant: &'static Fighter, manifest: Manifest, rewrite: Option<Rewrite>) {
    if !config::config().variant_enabled(variant.identity()) {
        println!("[Divergence] variant {} is off (config.toml)", variant.identity());
        return;
    }
    match variant.register(manifest) {
        Ok(kind) => println!("[Divergence] variant {} registered as kind {}", variant.identity(), kind),
        Err(error) => {
            println!("[Divergence] variant {} refused: {:?}", variant.identity(), error);
            return;
        }
    }
    if let Some(rewrite) = rewrite {
        unsafe { VARIANTS.push((variant, rewrite)); }
    }
}

// None for anything but a registered variant, whose commands are left alone, arguments untouched on the
// lua stack: they run from slotted movesets' own status code too (Armstrong's aerial charge flash)
unsafe fn rewrite_of(boma: *mut BattleObjectModuleAccessor) -> Option<Rewrite> {
    VARIANTS.iter()
        .find(|(variant, _)| variant.is(boma as usize))
        .map(|(_, rewrite)| *rewrite)
}

// for the ATTACK hook (function_hooks/attack.rs), which has the arguments off the stack already. it runs
// before Divergence's universal changes, so a variant's hitboxes still get them
pub unsafe fn rewrite_attack(boma: *mut BattleObjectModuleAccessor, params: &mut Vec<L2CValue>) {
    if let Some(rewrite) = rewrite_of(boma) {
        rewrite(boma, Command::Attack, params);
    }
}

// takes a command's `count` arguments off the lua stack, lets the variant change them, and puts them back
unsafe fn rewrite_command(lua_state: u64, command: Command, count: i32) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let rewrite = match rewrite_of(boma) {
        Some(rewrite) => rewrite,
        None => return
    };
    let mut agent = L2CAgent::new(lua_state);
    let mut params: Vec<L2CValue> = (0..count).map(|i| agent.pop_lua_stack(i + 1)).collect();
    agent.clear_lua_stack();
    rewrite(boma, command, &mut params);
    for param in params.iter_mut() {
        agent.push_lua_stack(param);
    }
}

#[skyline::hook(replace = smash::app::sv_animcmd::AFTER_IMAGE4_ON_arg29)]
unsafe fn divergence_after_image4_on(lua_state: u64) {
    rewrite_command(lua_state, Command::AfterImage, 29);
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_animcmd::EFFECT_FOLLOW)]
unsafe fn divergence_effect_follow(lua_state: u64) {
    rewrite_command(lua_state, Command::EffectFollow, 10);
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_animcmd::EFFECT_OFF_KIND)]
unsafe fn divergence_effect_off_kind(lua_state: u64) {
    rewrite_command(lua_state, Command::EffectOffKind, 3);
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_animcmd::PLAY_SE)]
unsafe fn divergence_play_se(lua_state: u64) {
    rewrite_command(lua_state, Command::PlaySe, 1);
    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        divergence_after_image4_on,
        divergence_effect_follow,
        divergence_effect_off_kind,
        divergence_play_se
    );
}
