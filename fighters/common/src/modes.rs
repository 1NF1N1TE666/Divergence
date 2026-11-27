// claude code: modes, roster-wide systems a player turns on in config.toml ([modes], src/config.rs).
// the faux air first, then aura; the hooks they need call in here, and every call checks the mode's
// switch first, so with a mode off the game runs as if it weren't there
use super::*;

// claude code: faux air, old Little Mac's air (2026-11-19) for everyone. ground statuses always see the
// ground (StatusModule::situation_kind, function_hooks/lua_bind_hook.rs), so every ground option works off it too:
// jabs, tilts, smashes, grabs and shield in midair, and no aerials. off the ground, they get the
// fighter's real gravity and fall speed (AIRBORNE + air_physics). hitstun, grabs, every special and
// every jump run in the real air (is_real_air_status), so specials keep their own air versions and jumps
// are the game's own: jumpsquat, short and full hops, tap jump, multijumps, delayed double jumps, all
// from the fighter's own params, animations and scripts (so slotted movesets and Dionysus clones keep
// theirs). the faux air takes back over the moment they do something else out of a jump (request,
// leave_jump_to_shield) or the jump ends (its fall becomes WAIT). Mac's own faux jumps (2026-11-19)
// worked out a speed from the jump height instead; that's gone. so are his side-B floor and his air
// side-B and down-B locks, which were his own
unsafe fn is_faux_air_on(boma: &mut BattleObjectModuleAccessor) -> bool {
    config::modes().faux_air && boma.is_fighter()
}

// claude code: statuses that run in the real air: hitstun, grabs, death and respawn, every special and
// fighter-specific status (SPECIAL_N and up, the same line IN_SPECIAL draws in lua_bind_hook.rs), and
// both jumps. they have to finish in the air (their fall becomes FALL, then WAIT, back in the faux air)
unsafe fn is_real_air_status(status_kind: i32) -> bool {
    status_kind >= *FIGHTER_STATUS_KIND_SPECIAL_N
    || [
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_THROWN,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH
    ].contains(&status_kind)
}

// claude code: the air statuses no one in faux air is ever in. falls, aerials and airdodges all become
// WAIT, the faux air's idle (request). out of a ground status none of these get asked for (its scripts
// see the ground), but out of a real-air one (a jump, hitstun, a special...) the air checks still run
unsafe fn is_air_only_status(status_kind: i32) -> bool {
    [
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR
    ].contains(&status_kind)
}

// claude code: true when a request for `status_kind` is one faux_air_request() turns into WAIT. the status hooks
// (lua_bind_hook.rs) then leave the input buffer alone, so the button that asked for an aerial is still
// there next frame, and WAIT's own checks turn it into the ground version (a jab, tilt...) in the faux air
pub unsafe fn keeps_buffer(boma: &mut BattleObjectModuleAccessor, status_kind: i32) -> bool {
    is_faux_air_on(boma) && is_air_only_status(status_kind)
}

// claude code: last line of defence. a status change that went around the request hooks (forced by the
// engine) and still landed them in an air-only status is swapped for WAIT before its main ever runs
// (this is called from sys_line, ahead of the status's main)
unsafe fn leave_air_only_status(boma: &mut BattleObjectModuleAccessor) -> bool {
    if !is_air_only_status(boma.status()) {
        return false;
    }
    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_WAIT, false);
    true
}

// claude code: a real jump has no way into shield (no air status does), so holding shield during one
// hands them back to the faux air's WAIT, which shields from there. everything else gets out of a jump
// by itself: aerial inputs become WAIT and its ground move (request), specials and further jumps are
// real statuses, and the jump's end falls into WAIT
unsafe fn leave_jump_to_shield(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL])
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
    }
}

// claude code: true when situation_kind should say ground: in a ground status
pub unsafe fn is_faux_ground(boma: &mut BattleObjectModuleAccessor) -> bool {
    is_faux_air_on(boma)
    && VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_GROUND_STATUS)
    && !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_REAL_SITUATION)
}

// claude code: true while a ground status is off the ground
pub unsafe fn is_airborne(boma: &mut BattleObjectModuleAccessor) -> bool {
    is_faux_air_on(boma)
    && VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_GROUND_STATUS)
    && VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_AIRBORNE)
}

// claude code: the real situation, past the situation_kind hook
unsafe fn real_situation(boma: &mut BattleObjectModuleAccessor) -> i32 {
    VarModule::on_flag(boma.object(), vars::common::instance::FAUX_AIR_REAL_SITUATION);
    let situation = StatusModule::situation_kind(boma);
    VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_REAL_SITUATION);
    situation
}

// claude code: faux gravity at the current faux speed. normal fall speed, or dive_speed_y fast-falling
unsafe fn apply_physics(boma: &mut BattleObjectModuleAccessor) {
    let fall_speed = if VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_DIVE) {
        WorkModule::get_param_float(boma, hash40("dive_speed_y"), 0)
    } else {
        WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0)
    };
    let speed_y = VarModule::get_float(boma.object(), vars::common::instance::FAUX_AIR_SPEED_Y).max(-fall_speed);
    VarModule::set_float(boma.object(), vars::common::instance::FAUX_AIR_SPEED_Y, speed_y);
    function_hooks::air_physics(boma, speed_y, fall_speed);
}

// claude code: leaves the ground (or keeps going in the air) at speed_y
unsafe fn take_off(boma: &mut BattleObjectModuleAccessor, speed_y: f32) {
    VarModule::on_flag(boma.object(), vars::common::instance::FAUX_AIR_AIRBORNE);
    VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_DIVE);
    VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_PASSING);
    VarModule::set_float(boma.object(), vars::common::instance::FAUX_AIR_SPEED_Y, speed_y);
    StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
    apply_physics(boma);
}

// claude code: back on the ground, the status carries on grounded. jumps come back
unsafe fn land(boma: &mut BattleObjectModuleAccessor) {
    VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_AIRBORNE);
    VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_DIVE);
    VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_PASSING);
    VarModule::set_float(boma.object(), vars::common::instance::FAUX_AIR_SPEED_Y, 0.0);
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_GROUND), true);
    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    GroundModule::attach_ground(boma, false);
    KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

// claude code: walking off an edge or dropping through a platform. falls from a standstill and loses
// the ground jump, like vanilla
unsafe fn start_fall(boma: &mut BattleObjectModuleAccessor) {
    let jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    WorkModule::set_int(boma, jump_count.max(1), *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    take_off(boma, 0.0);
}

// claude code: a status starting. ground statuses (asking for the ground) turn GROUND_STATUS on, air
// statuses and real-air ones turn it off. off the ground, a ground status keeps the air situation and
// correction instead of snapping down. coming out of hitstun, the faux fall starts at the speed then.
// coming out of a real-air status (a jump, a special), the faux fall starts at that status's vertical
// speed, so a jump cut short by a ground move keeps its momentum
pub unsafe fn faux_air_init_settings(boma: &mut BattleObjectModuleAccessor, situation: &mut SituationKind, ground_correct_kind: &mut u32) {
    if !is_faux_air_on(boma) {
        return;
    }
    if is_real_air_status(boma.status()) || situation.0 == *SITUATION_KIND_AIR {
        VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_GROUND_STATUS);
        VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_AIRBORNE);
        VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_DIVE);
        return;
    }
    if situation.0 == *SITUATION_KIND_GROUND {
        VarModule::on_flag(boma.object(), vars::common::instance::FAUX_AIR_GROUND_STATUS);
    }
    if !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_GROUND_STATUS)
    || real_situation(boma) != *SITUATION_KIND_AIR {
        return;
    }
    if !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_AIRBORNE) {
        VarModule::on_flag(boma.object(), vars::common::instance::FAUX_AIR_AIRBORNE);
        let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        VarModule::set_float(boma.object(), vars::common::instance::FAUX_AIR_SPEED_Y, speed_y);
        let prev = StatusModule::prev_status_kind(boma, 0);
        VarModule::set_flag(boma.object(), vars::common::instance::FAUX_AIR_FROM_HITSTUN, [
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL
        ].contains(&prev));
    }
    *situation = SituationKind(*SITUATION_KIND_NONE);
    *ground_correct_kind = *GROUND_CORRECT_KIND_AIR as u32;
}

// claude code: a new status resets gravity with its kinetic type, so it's put back right away
pub unsafe fn keep_falling(boma: &mut BattleObjectModuleAccessor) {
    if !is_airborne(boma) {
        return;
    }
    apply_physics(boma);
}

// claude code: out of hitstun (shield after tumble, which ends it into the faux air), the ground
// status picks up the knockback's horizontal speed, and nothing in the faux air slows it down. on the
// first frame there, once the status has set up its kinetics, every speed is wiped and the knockback
// energy switched off, then the faux gravity put back: they come out of hitstun with no horizontal speed
unsafe fn stop_hitstun_drift(boma: &mut BattleObjectModuleAccessor) {
    if !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_FROM_HITSTUN) {
        return;
    }
    VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_FROM_HITSTUN);
    let gravity = boma.get_gravity_energy() as *mut smash::app::FighterKineticEnergyGravity;
    let speed_y = lua_bind::KineticEnergy::get_speed_y(gravity as *mut smash::app::KineticEnergy);
    KineticModule::clear_speed_all(boma);
    KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    lua_bind::FighterKineticEnergyGravity::set_speed(gravity, speed_y);
}

// claude code: status requests. air-only statuses (is_air_only_status: falls, aerials, airdodges) become
// WAIT, the faux air's idle. in a ground status on the ground, jumps go ahead as they are (JUMP_SQUAT,
// then the real JUMP). off the ground, a ground status asking to jump (its scripts see the ground, so
// it asks for JUMP_SQUAT) gets the real JUMP_AERIAL instead, if any aerial jumps are left
// (jump_count_max; JUMP_AERIAL counts the jump itself), else nothing and the press is used up. platform
// drops are done here instead of by a status
pub unsafe fn faux_air_request(boma: &mut BattleObjectModuleAccessor, status_kind: i32) -> Option<i32> {
    if !is_faux_air_on(boma) {
        return Some(status_kind);
    }
    if keeps_buffer(boma, status_kind) {
        return Some(*FIGHTER_STATUS_KIND_WAIT);
    }
    if !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_GROUND_STATUS) {
        return Some(status_kind);
    }
    if status_kind == *FIGHTER_STATUS_KIND_PASS {
        if !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_AIRBORNE) {
            start_fall(boma);
            GroundModule::pass_floor(boma);
            VarModule::on_flag(boma.object(), vars::common::instance::FAUX_AIR_PASSING);
        }
        return None;
    }
    if !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_AIRBORNE) {
        return Some(status_kind);
    }
    if [
        *FIGHTER_STATUS_KIND_JUMP_SQUAT,
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL
    ].contains(&status_kind) {
        let jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        if jump_count < WorkModule::get_param_int(boma, hash40("jump_count_max"), 0) {
            return Some(*FIGHTER_STATUS_KIND_JUMP_AERIAL);
        }
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP_BUTTON);
        return None;
    }
    Some(status_kind)
}

// claude code: every frame in a ground status (opff.rs sys_line hook). the global table says ground
// too, since status scripts read it. on the ground, notices walking off an edge; in the
// air, lands once falling onto the ground, fast-falls on a down flick past the peak (dive_cont_value
// within dive_flick_frame_value frames, like vanilla), and keeps the faux gravity on through status changes
pub unsafe fn faux_air_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if !is_faux_air_on(boma) {
        return;
    }
    if leave_air_only_status(boma) {
        return;
    }
    leave_jump_to_shield(boma);
    if !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_GROUND_STATUS) {
        return;
    }
    fighter.global_table[globals::SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
    fighter.global_table[globals::PREV_SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
    let boma = fighter.boma();

    if !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_AIRBORNE) {
        if real_situation(boma) == *SITUATION_KIND_AIR {
            start_fall(boma);
            return;
        }
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        return;
    }

    stop_hitstun_drift(boma);
    let gravity = boma.get_gravity_energy() as *mut smash::app::FighterKineticEnergyGravity;
    if GroundModule::get_correct(boma) == *GROUND_CORRECT_KIND_AIR
    && KineticModule::is_enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
        let speed_y = lua_bind::KineticEnergy::get_speed_y(gravity as *mut smash::app::KineticEnergy);
        VarModule::set_float(boma.object(), vars::common::instance::FAUX_AIR_SPEED_Y, speed_y);
    }
    let speed_y = VarModule::get_float(boma.object(), vars::common::instance::FAUX_AIR_SPEED_Y);
    let on_ground = real_situation(boma) == *SITUATION_KIND_GROUND || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_DOWN as u32);
    if VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_PASSING) {
        if !on_ground {
            VarModule::off_flag(boma.object(), vars::common::instance::FAUX_AIR_PASSING);
        }
    } else if speed_y <= 0.0 && on_ground {
        land(boma);
        return;
    }
    if !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_DIVE)
    && speed_y <= 0.0
    && ControlModule::get_stick_y(boma) <= -WorkModule::get_param_float(boma, hash40("common"), hash40("dive_cont_value"))
    && VarModule::get_int(boma.object(), vars::common::instance::LEFT_STICK_FLICK_Y) < WorkModule::get_param_int(boma, hash40("common"), hash40("dive_flick_frame_value")) {
        VarModule::on_flag(boma.object(), vars::common::instance::FAUX_AIR_DIVE);
        let dive_speed_y = WorkModule::get_param_float(boma, hash40("dive_speed_y"), 0);
        VarModule::set_float(boma.object(), vars::common::instance::FAUX_AIR_SPEED_Y, -dive_speed_y);
    }
    apply_physics(boma);
}

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
// - mobility of moves: how far attacks and specials carry them (move_farther). falling, drift and plain
//   movement (walks, dashes, Kazuya's crouch dash, Peach's float) are left alone
// a weapon takes its owner's Aura the first time it's needed and keeps it. items don't get any.
// the formula's numbers are Lucario's own (fighter/lucario/param/vl.prc, param_aurapower), read off
// the vanilla file, since other fighters can't read his params. Stamina's HP formula isn't done: stamina
// matches use the percent one with their damage taken
unsafe fn is_aura_on() -> bool {
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
const AURA_EFFECT: &str = "sys_final_aura2";

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
    if !is_aura_on() || !is_aura_fighter(boma) {
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

// a weapon's Aura, its owner's the first time it's asked for. not when it spawns: it doesn't know its owner
// yet there (the ID reads 0, which is player 1), so everyone's weapons took player 1's
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
    if !is_aura_on() {
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
    is_aura_on() && boma.is_weapon() && !NOT_PROJECTILES.contains(&boma.kind())
    && VarModule::has_var_module(boma.object())
}

// called as a weapon puts out a hitbox (attack.rs): the first time, its size takes its Aura. the scale
// is set back to itself, and the set_scale hook (below, scale_mul) does the multiplying
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

// carries a move farther: on top of what it moves by itself, they're pushed by the rest of its Aura share,
// from outside, the way HDR adds speed to a move (KineticModule::add_speed_outside, fighters/buddy/src/opff.rs
// and others). the move's own speeds are left alone, so code that reads them back (Pac-Man's side special,
// Steve's glide) sees vanilla's numbers. animation movement and the speeds a move's own code sets (motion,
// stop) count both ways; gravity only while rising, so a launch goes Aura times as high and falls as usual
unsafe fn move_farther(boma: &mut BattleObjectModuleAccessor, aura: f32) {
    if aura == 1.0 || StopModule::is_stop(boma) || !is_in_move(boma) {
        return;
    }
    let mut speed = |id: i32| {
        if !KineticModule::is_enable_energy(boma, id) {
            return (0.0, 0.0);
        }
        let energy = KineticModule::get_energy(boma, id) as *mut smash::app::KineticEnergy;
        (lua_bind::KineticEnergy::get_speed_x(energy), lua_bind::KineticEnergy::get_speed_y(energy))
    };
    let (motion_x, motion_y) = speed(*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let (stop_x, stop_y) = speed(*FIGHTER_KINETIC_ENERGY_ID_STOP);
    let (_, gravity_y) = speed(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let x = motion_x + stop_x;
    let y = if boma.is_situation(*SITUATION_KIND_AIR) { motion_y + stop_y + gravity_y.max(0.0) } else { 0.0 };
    let push = Vector3f::new(x * (aura - 1.0), y * (aura - 1.0), 0.0);
    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &push);
}

// where the aura goes: both hands, both feet if the model has no hands, or the middle of the body if it
// has neither. kept in AURA_SPOT, 0 = not worked out yet
const SPOT_HANDS: i32 = 1;
const SPOT_FEET: i32 = 2;
const SPOT_CENTER: i32 = 3;

// the model a mod folder has for this fighter's costume, found by the fighter's own name
unsafe fn modded_model(boma: &mut BattleObjectModuleAccessor) -> Option<std::path::PathBuf> {
    let agent = boma.object().agent_kind_hash.hash;
    let model = format!("model/body/c{:02}/model.numshb", WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR));
    config::entries("sd:/ultimate/mods")
        .filter(|folder| !folder.file_name().to_string_lossy().starts_with('.'))
        .flat_map(|folder| config::entries(folder.path().join("fighter")))
        .filter(|fighter| hash40(&format!("fighter_kind_{}", fighter.file_name().to_string_lossy())) == agent)
        .map(|fighter| fighter.path().join(&model))
        .find(|path| path.exists())
}

// a model file lists the bones its vertices follow, so one with no hands never names a hand. the names:
// the game's own bones, plus the usual words on custom skeletons. read a piece at a time (a model can be
// 25 MB), keeping each piece's tail so a name cut in two is still found
fn model_spot(path: &std::path::Path) -> Option<i32> {
    use std::io::Read;
    const HANDS: [&[u8]; 4] = [b"handl\0", b"handr\0", b"finger", b"wrist"];
    const FEET: [&[u8]; 3] = [b"footl\0", b"footr\0", b"ankle"];
    const TAIL: usize = 8;
    let mut file = std::fs::File::open(path).ok()?;
    let mut buffer = vec![0; 0x10000 + TAIL];
    let mut kept = 0;
    let mut has_feet = false;
    loop {
        let read = file.read(&mut buffer[kept..]).ok()?;
        if read == 0 {
            return Some(if has_feet { SPOT_FEET } else { SPOT_CENTER });
        }
        let end = kept + read;
        buffer[kept..end].make_ascii_lowercase();
        let has = |name: &&[u8]| buffer[..end].windows(name.len()).any(|window| window == *name);
        if HANDS.iter().any(has) {
            return Some(SPOT_HANDS);
        }
        has_feet = has_feet || FEET.iter().any(has);
        kept = end.min(TAIL);
        buffer.copy_within(end - kept..end, 0);
    }
}

// a modded costume's model decides for itself; vanilla's handless models are known
unsafe fn aura_spot(boma: &mut BattleObjectModuleAccessor) -> i32 {
    let spot = VarModule::get_int(boma.object(), vars::common::instance::AURA_SPOT);
    if spot != 0 {
        return spot;
    }
    let spot = match modded_model(boma).and_then(|path| model_spot(&path)) {
        Some(spot) => spot,
        None if [*FIGHTER_KIND_KIRBY, *FIGHTER_KIND_PURIN].contains(&boma.kind()) => SPOT_FEET,
        None => SPOT_HANDS
    };
    VarModule::set_int(boma.object(), vars::common::instance::AURA_SPOT, spot);
    spot
}

// the joints it follows there. R.O.B.'s skeleton names its hands differently
unsafe fn aura_joints(boma: &mut BattleObjectModuleAccessor) -> &'static [&'static str] {
    match aura_spot(boma) {
        SPOT_FEET => &["footl", "footr"],
        SPOT_CENTER => &["hip"],
        _ if boma.kind() == *FIGHTER_KIND_ROBOT => &["havel", "haver"],
        _ => &["handl", "handr"]
    }
}

unsafe fn kill_aura_effects(boma: &mut BattleObjectModuleAccessor) {
    for id in [vars::common::instance::AURA_EFFECT_L, vars::common::instance::AURA_EFFECT_R] {
        let handle = VarModule::get_int(boma.object(), id) as u32;
        if handle != 0 {
            EffectModule::kill(boma, handle, false, false);
            VarModule::set_int(boma.object(), id, 0);
        }
    }
}

// the aura on their hands (or feet, or center), always on while they're alive, sized by their Aura like
// Lucario's
unsafe fn aura_effects(boma: &mut BattleObjectModuleAccessor, aura: f32) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY]) {
        kill_aura_effects(boma);
        return;
    }
    let t = ((aura - MIN_SCALE_AURA) / (MAX_SCALE_AURA - MIN_SCALE_AURA)).clamp(0.0, 1.0);
    let size = MIN_SCALE + (MAX_SCALE - MIN_SCALE) * t;
    let ids = [vars::common::instance::AURA_EFFECT_L, vars::common::instance::AURA_EFFECT_R];
    for (id, joint) in ids.iter().zip(aura_joints(boma)) {
        let mut handle = VarModule::get_int(boma.object(), *id) as u32;
        if handle == 0 || !EffectModule::is_exist_effect(boma, handle) {
            handle = EffectModule::req_follow(boma, Hash40::new(AURA_EFFECT), Hash40::new(joint), &Vector3f::zero(), &Vector3f::zero(), size, false, 0, 0, 0, 0, 0, false, false) as u32;
            VarModule::set_int(boma.object(), *id, handle as i32);
        }
        if handle != 0 {
            EffectModule::set_scale(boma, handle, &Vector3f::new(size, size, size));
        }
    }
}

// a fighter starting a match: where its aura goes is worked out again
unsafe extern "C" fn aura_fighter_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::common::instance::AURA_SPOT, 0);
}

// every frame, every fighter (opff.rs): works out their Aura, keeps track of whether they're in a move,
// carries their moves farther, and keeps the aura effects up
pub unsafe extern "C" fn aura_fighter_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if !is_aura_on() || !is_aura_fighter(boma) {
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

    move_farther(boma, aura);
    aura_effects(boma, aura);
}

// a weapon spawning: it has no Aura yet, and its size hasn't taken it. weapons are reused, so this clears
// what the last one left
pub unsafe extern "C" fn aura_weapon_start(weapon: &mut L2CWeaponCommon) {
    let boma = &mut *weapon.module_accessor;
    if !VarModule::has_var_module(boma.object()) {
        return;
    }
    VarModule::off_flag(boma.object(), vars::common::instance::AURA_WEAPON_SCALED);
    VarModule::set_float(boma.object(), vars::common::instance::AURA_POWER, 0.0);
}

// claude code: aura mode, power a status sets on a hitbox after it's out (counters, charged
// projectiles) gets the Aura too
#[skyline::hook(replace = AttackModule::set_power)]
unsafe fn set_power_hook(boma: &mut BattleObjectModuleAccessor, id: i32, power: f32, arg4: bool) {
    let mul = power_mul(boma);
    original!()(boma, id, power * mul, arg4)
}

// claude code: aura mode, a projectile's size takes its Aura
#[skyline::hook(replace = PostureModule::set_scale)]
unsafe fn set_scale_hook(boma: &mut BattleObjectModuleAccessor, scale: f32, arg3: bool) {
    let mul = scale_mul(boma);
    original!()(boma, scale * mul, arg3)
}

pub fn install() {
    smashline::Agent::new("fighter")
        .on_start(aura_fighter_start)
        .install();
    smashline::Agent::new("weapon")
        .on_start(aura_weapon_start)
        .install();
    skyline::install_hooks!(
        set_power_hook,
        set_scale_hook
    );
}
