// claude code: shared code for the divergent variants. each variant is its own fighter folder
// (fighters/<base>_<donor>, e.g. marth_reflet: lib.rs, opff.rs, vtable.rs, acmd/, status/, behind a
// feature of the same name), a Dionysus clone of its base carrying another fighter's
// sword gimmick, while the base fighter stays as it is. Dionysus falls back to the base's files for
// anything a variant doesn't ship, so a variant's romfs (romfs/fighter/<name>/) only holds what it
// changes. without Dionysus installed, registration fails quietly and no variant exists. a player can
// turn any variant off in config.toml ([variants], src/config.rs): it's then never registered, so it
// never reaches the select screen or takes up a fighter slot
use super::*;
use dionysus_api::v2::{Fighter, Manifest};

// registers a variant with Dionysus. series: the ui_series_<x> the select screen groups it under
// (Marth's is fireemblem). disp_order: its place on the select screen; the base's own number puts it
// right after the base (Marth's is 22). left unset, Dionysus uses -1, which hides it. hud: a fighter
// whose HUD to borrow by the portrait, driven by the variant
pub fn register(variant: &'static Fighter, base: &str, series: &str, disp_order: i8, hud: Option<&str>) {
    if !config::config().variant_enabled(variant.identity()) {
        println!("[Divergence] variant {} is off (config.toml)", variant.identity());
        return;
    }
    let mut manifest = Manifest::new(variant.identity(), base)
        .costumes(8)
        .series(series)
        .disp_order(disp_order);
    if let Some(donor) = hud {
        manifest = manifest.hud(donor);
    }
    match variant.register(manifest) {
        Ok(kind) => println!("[Divergence] variant {} registered as kind {}", variant.identity(), kind),
        Err(error) => println!("[Divergence] variant {} not registered: {:?}", variant.identity(), error)
    }
}

// variant state goes in VarModule like any fighter's: it runs from a common status, so clones have it too

// true for a hitbox swung with a sword: the ATTACK's region argument (stack index 35)
pub fn is_sword_hitbox(params: &[L2CValue]) -> bool {
    params[35].get_i32() == *ATTACK_REGION_SWORD
}

// the hooks below hand a variant the commands its scripts run. a variant adds itself to each one it
// needs with `#[cfg(feature = "<name>")] if crate::fighters::<name>::<HANDLE>.is(boma) { ... }`.
// the first one (Marth with Robin's swords, 2026-11-21) is in divergence-archive/marth-variants-2026-11-21

// true for a fighter that's a divergent variant. the effect and sound hooks (function_hooks/effect.rs)
// leave everyone else's commands alone, arguments untouched on the lua stack: those commands run from
// slotted movesets' own status code too (Armstrong's aerial charge flash), and taking them apart for
// nothing was a risk. a variant adds itself here like in the hooks below. none is in the build now
pub unsafe fn is_variant(boma: *mut BattleObjectModuleAccessor) -> bool {
    false
}

// called from the ATTACK hook (function_hooks/attack.rs) before Divergence's universal changes (so a
// variant's hitboxes still get them), so a variant can rewrite any hitbox it sets
pub unsafe fn on_attack(boma: *mut BattleObjectModuleAccessor, params: &mut Vec<L2CValue>) {
}

// the effect commands (function_hooks/effect.rs): AFTER_IMAGE4_ON (29 arguments: 0 and 1 the trail
// textures, 12 the blade flare), EFFECT_FOLLOW and EFFECT_OFF_KIND (0 the effect)
pub unsafe fn on_after_image(boma: *mut BattleObjectModuleAccessor, params: &mut Vec<L2CValue>) {
}

pub unsafe fn on_effect(boma: *mut BattleObjectModuleAccessor, params: &mut Vec<L2CValue>) {
}

// the sound commands (function_hooks/effect.rs): PLAY_SE (0 the sound)
pub unsafe fn on_sound(boma: *mut BattleObjectModuleAccessor, params: &mut Vec<L2CValue>) {
}
