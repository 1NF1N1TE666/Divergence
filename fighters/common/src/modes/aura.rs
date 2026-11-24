use super::*;

// claude code: aura mode (2026-11-23), Lucario's Aura for everyone. every fighter but Lucario (who keeps
// his own, Force Palm and Extreme Speed formulas included) gets an Aura multiplier worked out the way the
// game works out his: from their percent, times Stock Aura from the stock standings, held between 0.6x
// and 1.8x. it multiplies:
// - damage: every hitbox they and their projectiles put out (ATTACK and ATTACK_ABS, function_hooks/attack.rs)
//   and any power a status sets on a hitbox afterwards (AttackModule::set_power), which is how counters
//   and charged projectiles get theirs. knockback follows the damage, like Lucario's
// - projectile size: a weapon's scale, the first time it puts out a hitbox (so props that never attack,
//   like bows, keep their size), and every scale it sets itself after that (charge shots growing).
//   hitboxes grow with the scale, like Aura Sphere's. summons and held weapons that attack (Luma, Pikmin,
//   Wario's bike, Arsene, whips) keep their size (NOT_PROJECTILES), their hits still get the damage
// - mobility of moves: how far attacks and specials carry them. animation movement (dash attacks,
//   sliding tilts, most recoveries) through the motion energy (function_hooks/energy.rs), and the speeds
//   a move's own code gives them through the stop energy (sv_kinetic_energy hooks, energy.rs): speed,
//   accel, brake and caps all get the multiplier, so the path stretches but the timing stays, and
//   speeds an ACMD script sets outright (SET_SPEED_EX). launches and hops a move gives the gravity
//   energy get √Aura, so with gravity unchanged they rise Aura times as high (2026-11-23). gravity
//   itself, drift and plain movement (walks, dashes, Kazuya's crouch dash, Peach's float) are left alone
// a weapon takes its owner's Aura when it spawns and keeps it, like Aura Sphere. items don't get any.
// the formula's numbers are Lucario's own (fighter/lucario/param/vl.prc, param_aurapower), read off
// the vanilla file, since other fighters can't read his params. Stamina's HP formula isn't done: stamina
// matches use the percent one with their damage taken
unsafe fn is_on() -> bool {
    config::modes().aura
}

// param_aurapower: 0.66x at 0%, 1x at 65%, 1.67x at 190%, straight lines between, flat past the ends
const MIN_AURAPOWER: f32 = 0.66;
const MAX_AURAPOWER: f32 = 1.67;
const BASIS_DAMAGE: f32 = 65.0;
const MAX_DAMAGE: f32 = 190.0;
// Stock Aura, 4+ players at rank +2 / -2 (handi_max / handi_min). per rank and per half-player-count
// that's 0.085 off when ahead and 0.2 on when behind
const HANDI_MAX: f32 = 0.66;
const HANDI_MIN: f32 = 1.8;
// the whole multiplier's floor and cap
const BOTTOM: f32 = 0.6;
const CAP: f32 = 1.8;
// Stock Aura eases to a new value over 375 frames per half-player-count (750 at 4+ players)
const STOCK_EASE_FRAMES: f32 = 375.0;
// param_aurascale: the hand effects' size, 0.4 at 0.6x Aura up to 1.2 at 1.35x
const MIN_SCALE: f32 = 0.4;
const MIN_SCALE_AURA: f32 = 0.6;
const MAX_SCALE: f32 = 1.2;
const MAX_SCALE_AURA: f32 = 1.35;

// a common effect that loops on a joint, the closest to Lucario's hand aura in common (his own,
// lucario_hadou, is in his effect bank, which nobody else has loaded)
const HAND_EFFECT: &str = "sys_final_aura2";

// weapons that attack but aren't projectiles: summons and held weapons keep their size
const NOT_PROJECTILES: [i32; 6] = [0xE6, 0x9D, 0x8A, 0x1EC, 0x1AD, 0x1BE]; // ROSETTA_TICO, PIKMIN_PIKMIN, WARIO_WARIOBIKE, JACK_DOYLE, SIMON_WHIP, RICHTER_WHIP

// the percent half of the multiplier
fn percent_aura(percent: f32) -> f32 {
    let percent = percent.clamp(0.0, MAX_DAMAGE);
    if percent <= BASIS_DAMAGE {
        1.0 + (percent - BASIS_DAMAGE) * (1.0 - MIN_AURAPOWER) / BASIS_DAMAGE
    } else {
        1.0 + (percent - BASIS_DAMAGE) * (MAX_AURAPOWER - 1.0) / (MAX_DAMAGE - BASIS_DAMAGE)
    }
}

// fighters who get the mode: everyone but Lucario, who has the real thing
unsafe fn is_aura_fighter(boma: &mut BattleObjectModuleAccessor) -> bool {
    boma.is_fighter() && boma.kind() != *FIGHTER_KIND_LUCARIO
}

// Stock Aura's target, and how many frames it eases over. like the game: only humans count (a CPU gets
// none, and CPUs aren't counted against), rank is how many stocks they're ahead of last place plus how
// many they're behind first place, held to -2..2. time and coin matches have no stocks to compare, so
// they get none here
unsafe fn stock_target(boma: &mut BattleObjectModuleAccessor) -> (f32, f32) {
    let none = (1.0, STOCK_EASE_FRAMES);
    let manager = function_hooks::fighter_manager();
    if manager.is_null() || function_hooks::is_cpu(boma) {
        return none;
    }
    let own_entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let own_info = lua_bind::FighterManager::get_fighter_information(manager, FighterEntryID(own_entry));
    if own_info.is_null() {
        return none;
    }
    let own = lua_bind::FighterInformation::stock_count(own_info) as i32;
    let mut players = 1;
    let mut most = i32::MIN;
    let mut fewest = i32::MAX;
    for entry in 0..8 {
        if entry == own_entry || lua_bind::FighterManager::get_fighter_entry(manager, FighterEntryID(entry)) == 0 {
            continue;
        }
        let info = lua_bind::FighterManager::get_fighter_information(manager, FighterEntryID(entry));
        if info.is_null() || lua_bind::FighterInformation::is_operation_cpu(info) {
            continue;
        }
        let stocks = lua_bind::FighterInformation::stock_count(info) as i32;
        players += 1;
        most = most.max(stocks);
        fewest = fewest.min(stocks);
    }
    if players < 2 {
        return none;
    }
    let rank = ((own - fewest).max(0) + (own - most).min(0)).clamp(-2, 2) as f32;
    let half_players = players.min(4) as f32 / 2.0;
    let target = if rank > 0.0 {
        1.0 - (1.0 - HANDI_MAX) / 4.0 * rank * half_players
    } else {
        1.0 - (HANDI_MIN - 1.0) / 4.0 * rank * half_players
    };
    (target, STOCK_EASE_FRAMES * half_players)
}

// eases the Stock Aura multiplier toward the standings' target, like the game does
unsafe fn update_stock_aura(boma: &mut BattleObjectModuleAccessor) -> f32 {
    let object = boma.object();
    let mut current = VarModule::get_float(object, vars::common::instance::AURA_STOCK_MUL);
    if current == 0.0 || boma.is_status(*FIGHTER_STATUS_KIND_ENTRY) {
        current = 1.0;
        VarModule::set_float(object, vars::common::instance::AURA_STOCK_TARGET, 1.0);
        VarModule::set_float(object, vars::common::instance::AURA_STOCK_STEP, 0.0);
    }
    let (target, frames) = stock_target(boma);
    if (target - VarModule::get_float(object, vars::common::instance::AURA_STOCK_TARGET)).abs() > 0.0001 {
        VarModule::set_float(object, vars::common::instance::AURA_STOCK_TARGET, target);
        VarModule::set_float(object, vars::common::instance::AURA_STOCK_STEP, (target - current).abs() / frames);
    }
    let step = VarModule::get_float(object, vars::common::instance::AURA_STOCK_STEP);
    current = if current < target { (current + step).min(target) } else { (current - step).max(target) };
    VarModule::set_float(object, vars::common::instance::AURA_STOCK_MUL, current);
    current
}

// a fighter's Aura right now (1.0 with the mode off, or for Lucario)
pub unsafe fn fighter_aura(boma: &mut BattleObjectModuleAccessor) -> f32 {
    if !is_on() || !is_aura_fighter(boma) {
        return 1.0;
    }
    let aura = VarModule::get_float(boma.object(), vars::common::instance::AURA_POWER);
    if aura > 0.0 { aura } else { percent_aura(DamageModule::damage(boma, 0)) }
}

// the fighter a weapon belongs to (the founder, past any weapons that made it), if it's an aura fighter
unsafe fn weapon_owner(boma: &mut BattleObjectModuleAccessor) -> Option<&'static mut BattleObjectModuleAccessor> {
    let id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if !sv_battle_object::is_active(id) || sv_battle_object::category(id) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        return None;
    }
    let owner = &mut *sv_battle_object::module_accessor(id);
    if is_aura_fighter(owner) { Some(owner) } else { None }
}

// a weapon's Aura: its owner's when it spawned (weapon_start), or its owner's now if that missed
unsafe fn weapon_aura(boma: &mut BattleObjectModuleAccessor) -> f32 {
    if !VarModule::has_var_module(boma.object()) {
        return 1.0;
    }
    let aura = VarModule::get_float(boma.object(), vars::common::instance::AURA_POWER);
    if aura > 0.0 {
        return aura;
    }
    match weapon_owner(boma) {
        Some(owner) => {
            let aura = fighter_aura(owner);
            VarModule::set_float(boma.object(), vars::common::instance::AURA_POWER, aura);
            aura
        }
        None => 1.0
    }
}

// what a hitbox's damage gets multiplied by (attack.rs)
pub unsafe fn power_mul(boma: &mut BattleObjectModuleAccessor) -> f32 {
    if !is_on() {
        return 1.0;
    }
    if boma.is_fighter() {
        fighter_aura(boma)
    } else if boma.is_weapon() {
        weapon_aura(boma)
    } else {
        1.0
    }
}

unsafe fn is_projectile(boma: &mut BattleObjectModuleAccessor) -> bool {
    is_on() && boma.is_weapon() && !NOT_PROJECTILES.contains(&boma.kind())
    && VarModule::has_var_module(boma.object())
}

// called as a weapon puts out a hitbox (attack.rs): the first time, its size takes its Aura. the scale
// is set back to itself, and the set_scale hook (function_hooks/mod.rs, scale_mul) does the multiplying
pub unsafe fn scale_projectile(boma: &mut BattleObjectModuleAccessor) {
    if !is_projectile(boma)
    || VarModule::is_flag(boma.object(), vars::common::instance::AURA_WEAPON_SCALED)
    || weapon_aura(boma) == 1.0 {
        return;
    }
    VarModule::on_flag(boma.object(), vars::common::instance::AURA_WEAPON_SCALED);
    PostureModule::set_scale(boma, PostureModule::scale(boma), false);
}

// what a scale a weapon sets gets multiplied by, once its size has taken its Aura
pub unsafe fn scale_mul(boma: &mut BattleObjectModuleAccessor) -> f32 {
    if !is_projectile(boma) || !VarModule::is_flag(boma.object(), vars::common::instance::AURA_WEAPON_SCALED) {
        return 1.0;
    }
    weapon_aura(boma)
}

// the attacks every fighter shares
unsafe fn is_common_attack(status_kind: i32) -> bool {
    [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_100,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status_kind)
}

// true while they're in an attack or a special. common statuses: the attacks above. the four specials.
// a fighter's own statuses (SPECIAL_N and up) count when they follow on from a move (AURA_IN_MOVE,
// carried from status to status by fighter_frame), or once they put out a hitbox (Ryu's command
// normals, Kazuya's moves out of crouch dash). crouch dash itself, Peach's float and the like follow
// on from plain movement and never attack, so they don't count
unsafe fn is_in_move(boma: &mut BattleObjectModuleAccessor) -> bool {
    let status_kind = boma.status();
    if status_kind < *FIGHTER_STATUS_KIND_SPECIAL_N {
        return is_common_attack(status_kind);
    }
    [
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW
    ].contains(&status_kind)
    || VarModule::is_flag(boma.object(), vars::common::instance::AURA_IN_MOVE)
    || VarModule::is_flag(boma.object(), vars::common::status::AURA_STATUS_ATTACKED)
}

// what a move's movement gets multiplied by (energy.rs): their Aura during a move, else 1
pub unsafe fn mobility_mul(boma: &mut BattleObjectModuleAccessor) -> f32 {
    if !is_on() || !is_aura_fighter(boma) || !is_in_move(boma) {
        return 1.0;
    }
    fighter_aura(boma)
}

unsafe fn kill_hand_effects(boma: &mut BattleObjectModuleAccessor) {
    for id in [vars::common::instance::AURA_EFFECT_L, vars::common::instance::AURA_EFFECT_R] {
        let handle = VarModule::get_int(boma.object(), id) as u32;
        if handle != 0 {
            EffectModule::kill(boma, handle, false, false);
            VarModule::set_int(boma.object(), id, 0);
        }
    }
}

// the aura on both hands, always on while they're alive, sized by their Aura like Lucario's
unsafe fn hand_effects(boma: &mut BattleObjectModuleAccessor, aura: f32) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY]) {
        kill_hand_effects(boma);
        return;
    }
    let t = ((aura - MIN_SCALE_AURA) / (MAX_SCALE_AURA - MIN_SCALE_AURA)).clamp(0.0, 1.0);
    let size = MIN_SCALE + (MAX_SCALE - MIN_SCALE) * t;
    for (id, joint) in [(vars::common::instance::AURA_EFFECT_L, "handl"), (vars::common::instance::AURA_EFFECT_R, "handr")] {
        let mut handle = VarModule::get_int(boma.object(), id) as u32;
        if handle == 0 || !EffectModule::is_exist_effect(boma, handle) {
            handle = EffectModule::req_follow(boma, Hash40::new(HAND_EFFECT), Hash40::new(joint), &Vector3f::zero(), &Vector3f::zero(), size, false, 0, 0, 0, 0, 0, false, false) as u32;
            VarModule::set_int(boma.object(), id, handle as i32);
        }
        if handle != 0 {
            EffectModule::set_scale(boma, handle, &Vector3f::new(size, size, size));
        }
    }
}

// every frame, every fighter (opff.rs): works out their Aura, keeps track of whether they're in a move,
// and keeps the hand effects up
pub unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if !is_on() || !is_aura_fighter(boma) {
        return;
    }
    let stock = update_stock_aura(boma);
    let aura = (percent_aura(DamageModule::damage(boma, 0)) * stock).clamp(BOTTOM, CAP);
    VarModule::set_float(boma.object(), vars::common::instance::AURA_POWER, aura);

    if boma.status() >= *FIGHTER_STATUS_KIND_SPECIAL_N
    && (0..8).any(|id| AttackModule::is_attack(boma, id, false)) {
        VarModule::on_flag(boma.object(), vars::common::status::AURA_STATUS_ATTACKED);
    }
    let in_move = is_in_move(boma);
    VarModule::set_flag(boma.object(), vars::common::instance::AURA_IN_MOVE, in_move);

    hand_effects(boma, aura);
}

// a weapon spawning: it takes its owner's Aura now and keeps it, and its size hasn't taken it yet.
// weapons are reused, so this clears what the last one left
pub unsafe extern "C" fn weapon_start(weapon: &mut L2CWeaponCommon) {
    let boma = &mut *weapon.module_accessor;
    if !VarModule::has_var_module(boma.object()) {
        return;
    }
    VarModule::off_flag(boma.object(), vars::common::instance::AURA_WEAPON_SCALED);
    VarModule::set_float(boma.object(), vars::common::instance::AURA_POWER, 0.0);
    if !is_on() {
        return;
    }
    if let Some(owner) = weapon_owner(boma) {
        let aura = fighter_aura(owner);
        VarModule::set_float(boma.object(), vars::common::instance::AURA_POWER, aura);
    }
}

pub fn install() {
    smashline::Agent::new("weapon")
        .on_start(weapon_start)
        .install();
}
