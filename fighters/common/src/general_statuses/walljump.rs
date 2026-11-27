use super::*;
use globals::*;

// claude code: a wall jump gives back the aerial specials used this airtime: the no-special-fall locks
// (function_hooks/lua_bind_hook.rs), and every fighter's own locks through the lock override below. hop flags
// (a special's lift only once per airtime) are left alone for now

// claude code: lock override. whatever a fighter keeps its "once per airtime" locks in, the game asks two things
// before an air special starts (L2CFighterCommon::sub_transition_group_check_air_special): is the special's
// transition term on, and does the fighter's USE_SPECIAL_* callback (global 0x38-0x3B) allow it. a lock has to
// answer no to one of them. so a special started in the air (AIR_USED) is marked REFRESHED by a wall jump, and
// until it's used again, both questions answer yes for it in the fighter's free air statuses. resource checks
// (Robin's tomes, Hero's MP, Steve's materials, Inkling's ink) answer through the same two, so a refreshed
// special skips those as well
const SPECIALS: [(i32, i32, i32); 4] = [
    (vars::common::instance::SPECIAL_N_AIR_USED, vars::common::instance::SPECIAL_N_REFRESHED, USE_SPECIAL_N_CALLBACK),
    (vars::common::instance::SPECIAL_S_AIR_USED, vars::common::instance::SPECIAL_S_REFRESHED, USE_SPECIAL_S_CALLBACK),
    (vars::common::instance::SPECIAL_HI_AIR_USED, vars::common::instance::SPECIAL_HI_REFRESHED, USE_SPECIAL_HI_CALLBACK),
    (vars::common::instance::SPECIAL_LW_AIR_USED, vars::common::instance::SPECIAL_LW_REFRESHED, USE_SPECIAL_LW_CALLBACK),
];

unsafe fn special_index(status_kind: i32) -> Option<usize> {
    [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW]
        .iter()
        .position(|kind| *kind == status_kind)
}

unsafe fn term_index(term: i32) -> Option<usize> {
    if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND].contains(&term) {
        Some(0)
    } else if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND].contains(&term) {
        Some(1)
    } else if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND].contains(&term) {
        Some(2)
    } else if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND].contains(&term) {
        Some(3)
    } else {
        None
    }
}

// a special status was asked for (lua_bind_hook.rs, the status request filter)
pub unsafe fn note_special_start(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let Some(index) = special_index(status_kind) else { return };
    let (air_used, refreshed, _) = SPECIALS[index];
    VarModule::off_flag(boma.object(), refreshed);
    if !boma.is_situation(*SITUATION_KIND_GROUND) {
        VarModule::on_flag(boma.object(), air_used);
    }
}

// landing, getting hit or dying (lua_bind_hook.rs no_special_fall): the fighter's own locks are back in charge
pub unsafe fn clear_air_specials(object: *mut BattleObject) {
    for (air_used, refreshed, _) in SPECIALS {
        VarModule::off_flag(object, air_used);
        VarModule::off_flag(object, refreshed);
    }
}

unsafe fn refresh_air_specials(object: *mut BattleObject) {
    for (air_used, refreshed, _) in SPECIALS {
        if VarModule::is_flag(object, air_used) {
            VarModule::on_flag(object, refreshed);
        }
        VarModule::off_flag(object, air_used);
    }
}

// the transition term side (function_hooks/transition.rs)
pub unsafe fn is_term_forced(boma: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    if !boma.is_fighter() || !VarModule::is_flag(boma.object(), vars::common::instance::SPECIALS_FORCED) {
        return false;
    }
    term_index(term).map_or(false, |index| VarModule::is_flag(boma.object(), SPECIALS[index].1))
}

// the callback side: a refreshed special's USE_SPECIAL callback is taken out for the check, then put back
#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_special)]
unsafe fn sub_transition_group_check_air_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_free_air = fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_WALL_JUMP,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ]);
    if !is_free_air || !SPECIALS.iter().any(|(_, refreshed, _)| VarModule::is_flag(fighter.battle_object, *refreshed)) {
        return original!()(fighter);
    }
    let mut callbacks = vec![];
    for (_, refreshed, slot) in SPECIALS {
        if VarModule::is_flag(fighter.battle_object, refreshed) && fighter.global_table[slot].get_bool() {
            callbacks.push((slot, fighter.global_table[slot].get_ptr()));
            fighter.global_table[slot].assign(&L2CValue::Bool(false));
        }
    }
    VarModule::on_flag(fighter.battle_object, vars::common::instance::SPECIALS_FORCED);
    let ret = original!()(fighter);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::SPECIALS_FORCED);
    for (slot, callback) in callbacks {
        fighter.global_table[slot].assign(&L2CValue::Ptr(callback as _));
    }
    ret
}

unsafe fn restore_specials(fighter: &mut L2CFighterCommon) {
    refresh_air_specials(fighter.battle_object);
    for flag in function_hooks::lua_bind_hook::special_used_flags() {
        VarModule::off_flag(fighter.battle_object, flag);
    }
}

// the common WALL_JUMP Main, hooked the way HDR hooks common statuses. a smashline "fighter" status is
// skipped on any costume a moveset claims (Knuckles' slots), so it can't be relied on
#[skyline::hook(replace = L2CFighterCommon_status_wall_jump)]
unsafe fn status_wall_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    lasso::leave_wall(fighter);
    restore_specials(fighter);
    if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
        fighter.boma().dec_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    original!()(fighter)
}

// claude code: wall jumps out of the states that come right before special fall, and more. any air state, specials included,
// can wall jump whenever it has no hitbox out (a special's end, like PK Thunder 2's window between its attack and special fall,
// or an aerial's endlag). it runs the same wall jump test falling does (general_statuses::try_wall_jump: one wall jump per airtime,
// facing either way), and the wall jump itself gives the specials back like any other
// (general_statuses/walljump.rs). the faux air mode's ground statuses never read as air, so they're left out on their own.
//
// which air states: the fighter's own moves only, as an allow-list (wall_jump_anytime_status). the
// first version allowed every air state but a list of exceptions, and that let a fighter being carried
// by a command grab or an inhale (CATCHED_*, SWALLOWED_*: common statuses no list caught all of), or
// one in a Final Smash, wall jump out of it mid-link, which can crash the game; CPUs flick toward walls
// all the time. a fighter holding someone (capture link) or in a Final Smash is left out too
pub unsafe extern "C" fn wall_jump_anytime(fighter: &mut L2CFighterCommon) {
    if !fighter.is_situation(*SITUATION_KIND_AIR)
    || StatusModule::is_changing(fighter.module_accessor)
    || StopModule::is_stop(fighter.module_accessor)
    || !wall_jump_anytime_status(fighter.status())
    || fighter.boma().is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL)
    || LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32 != *BATTLE_OBJECT_ID_INVALID as u32
    || (0..8).any(|id| AttackModule::is_attack(fighter.module_accessor, id, false)) {
        return;
    }
    general_statuses::try_wall_jump(fighter);
}

// claude code: the air states wall_jump_anytime covers: falling, jumping, aerials and Zair, plus the
// fighter's specials. specials and every fighter's own statuses sit at SPECIAL_N and up; everything
// done TO a fighter (hitstun, grabs, command grabs, inhales, burying...) is a common status below it
fn wall_jump_anytime_status(status_kind: i32) -> bool {
    status_kind >= *FIGHTER_STATUS_KIND_SPECIAL_N
    || [
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_AIR_LASSO
    ].contains(&status_kind)
}

pub unsafe extern "C" fn wall_jump_count_reset(fighter: &mut L2CFighterCommon) {
    if !fighter.is_situation(*SITUATION_KIND_AIR)
    || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP
    ]) {
        fighter.set_int(0, *FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_COUNT);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(status_wall_jump, sub_transition_group_check_air_special);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
