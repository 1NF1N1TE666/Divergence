use super::*;

// claude code: the parry window length, common `shield_just_frame`, shared by both ends of shielding
// so the two windows always match
pub unsafe fn parry_window_frames(boma: &mut BattleObjectModuleAccessor) -> i32 {
    boma.get_param_int("common", "shield_just_frame")
}

// claude code: turns the shield into a parry for `shield_just_frame` frames. it reflects any projectile
// with no multipliers (1x damage, speed, lifetime, no hitlag) through the shield's own just-shield reflector
// group, like HDR's parry. the game only counts a shield hit as a parry while JUST_FRAME is above 0.
// `parry_window` below counts it down
pub unsafe fn open_parry_window(boma: &mut BattleObjectModuleAccessor) {
    let group = *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD;
    let frames = parry_window_frames(boma);
    boma.set_int(frames, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    ShieldModule::set_shield_type(boma, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
    ReflectorModule::set_status(boma, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), group);
    ReflectorModule::set_attack_mul(boma, 1.0, group);
    ReflectorModule::set_speed_mul(boma, 1.0, group);
    ReflectorModule::set_life_mul(boma, 1.0, group);
    ReflectorModule::set_attack_limit(boma, 999.9, group);
    ReflectorModule::set_hit_stop_mul(boma, 0.0);
    VarModule::set_int(boma.object(), vars::common::instance::PARRY_WINDOW_FRAME, frames);
}

// claude code: stops reflecting; a shield that's still up goes back to a normal shield
pub unsafe fn close_parry_window(boma: &mut BattleObjectModuleAccessor) {
    ReflectorModule::set_status(boma, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD]) {
        ShieldModule::set_shield_type(boma, ShieldType(*SHIELD_TYPE_GUARD), *FIGHTER_SHIELD_KIND_GUARD, 0);
        boma.set_int(0, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    }
    VarModule::set_int(boma.object(), vars::common::instance::PARRY_WINDOW_FRAME, 0);
}

// claude code: counts down the parry window (general_statuses/shield.rs) and closes it at 0, or
// right away once the fighter leaves the shield statuses (a landed parry, jumping out, etc.)
pub unsafe extern "C" fn parry_window(fighter: &mut L2CFighterCommon) {
    let frame = VarModule::get_int(fighter.battle_object, vars::common::instance::PARRY_WINDOW_FRAME);
    if frame <= 0 {
        return;
    }
    if frame <= 1 || !fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_GUARD_ON, 
        *FIGHTER_STATUS_KIND_GUARD, 
        *FIGHTER_STATUS_KIND_GUARD_OFF
    ]) {
        general_statuses::shield::close_parry_window(fighter.boma());
    } else {
        VarModule::dec_int(fighter.battle_object, vars::common::instance::PARRY_WINDOW_FRAME);
    }
}

// claude code: the parry reward. the defender heals half their missing shield and can act on the very
// next frame: hitlag is cancelled and they're forced to WAIT, which allows every grounded option. shield
// held through the parry gets no startup parry until it's released, or holding shield would chain
// parries through every hit of a multi-hit move. runs from sys_line_system_control_fighter, which
// still runs during hitlag
pub unsafe fn parry_reward(fighter: &mut L2CFighterCommon) {
    let is_shield_held = fighter.boma().is_button_on(Buttons::Guard | Buttons::GuardHold);
    if !is_shield_held {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::PARRY_SHIELD_HELD);
    }
    if !fighter.is_status(*FIGHTER_STATUS_KIND_GUARD_DAMAGE)
    || !fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        return;
    }
    let shield = fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let shield_max = fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    fighter.set_float(shield + (shield_max - shield) * 0.5, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    if is_shield_held {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::PARRY_SHIELD_HELD);
    }
    close_parry_window(fighter.boma());
    ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(*SHIELD_TYPE_GUARD), *FIGHTER_SHIELD_KIND_GUARD, 0);
    fighter.set_int(0, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    StopModule::cancel_hit_stop(fighter.module_accessor);
    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
}

// claude code: powershield: a parry window at the start of shielding (Melee/Brawl/Smash 4)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus)]
unsafe fn sub_ftStatusUniqProcessGuardOn_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = call_original!(fighter);
    if !VarModule::is_flag(fighter.battle_object, vars::common::instance::PARRY_SHIELD_HELD) {
        open_parry_window(fighter.boma());
    }
    ret
}

// claude code: release parry (Ultimate): vanilla's window is forced to the same param, and it
// reflects at 1x
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_GuardOff_Common)]
unsafe fn status_GuardOff_Common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = call_original!(fighter);
    open_parry_window(fighter.boma());
    ret
}

// claude code: held shield SDI (general_statuses/mod.rs) in place of vanilla's flick version. sideways
// only, like vanilla's, at common `hit_stop_delay_guard_mul` times the hit's SDI multiplier
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusGuard__check_hit_stop_delay_flick)]
unsafe fn FighterStatusGuard__check_hit_stop_delay_flick(fighter: &mut L2CFighterCommon, user_mul: L2CValue) -> L2CValue {
    if !StopModule::is_hit(fighter.module_accessor)
    || fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_DISABLE_HIT_STOP_DELAY_STICK) {
        return false.into();
    }
    let mul = fighter.get_float(user_mul.get_i32()) * fighter.get_param_float("common", "hit_stop_delay_guard_mul");
    match general_statuses::held_sdi(fighter, mul) {
        Some((x, _)) => {
            general_statuses::apply_sdi(fighter, x, 0.0);
            true.into()
        }
        None => false.into()
    }
}

// claude code: shield breaks. there's no stun: the hit that broke the shield goes through it and hits
// the defender, the way a shield poke would, and the shield is gone until they respawn
// (is_shield_broken, transition.rs). a shield that runs out on its own (held too long) just breaks, with
// no hit. the break is caught as SHIELD_BREAK_FLY starts: the attacker's hitboxes are made over fresh,
// the way a multi-hit move clears and re-makes its hitboxes to hit again, so the same hitbox hits the
// now shieldless defender. SHIELD_BREAK_FLY's first frame hands control back, so FURAFURA never happens.
// (a set hit through ATTACK_ABS was tried first: it never lands on a fighter who isn't grabbed)

// claude code: the fighter (or projectile) whose hit just broke this shield. read before the new
// status clears its work values: the shield hit's attacker id (GUARD_DAMAGE's OBJECT_ID), if it holds
// one; otherwise the nearest fighter whose current move has hit a shield
unsafe fn shield_breaker(fighter: &mut L2CFighterCommon) -> Option<u32> {
    let own_id = (*fighter.battle_object).battle_object_id;
    let id = fighter.get_int(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_OBJECT_ID) as u32;
    if id != own_id && id != 0 && sv_battle_object::is_active(id) {
        return Some(id);
    }
    let manager = function_hooks::fighter_manager();
    if manager.is_null() {
        return None;
    }
    let own_x = PostureModule::pos_x(fighter.module_accessor);
    let own_y = PostureModule::pos_y(fighter.module_accessor);
    let mut nearest: Option<(u32, f32)> = None;
    for entry in 0..8 {
        let fighter_entry = lua_bind::FighterManager::get_fighter_entry(manager, FighterEntryID(entry));
        if fighter_entry == 0 {
            continue;
        }
        let id = lua_bind::FighterEntry::current_fighter_id(fighter_entry as *mut smash::app::FighterEntry) as u32;
        if id == own_id || !sv_battle_object::is_active(id) {
            continue;
        }
        let other = sv_battle_object::module_accessor(id);
        if !AttackModule::is_infliction_status(other, *COLLISION_KIND_MASK_SHIELD) {
            continue;
        }
        let dx = PostureModule::pos_x(other) - own_x;
        let dy = PostureModule::pos_y(other) - own_y;
        let distance = dx * dx + dy * dy;
        if nearest.map_or(true, |(_, best)| distance < best) {
            nearest = Some((id, distance));
        }
    }
    nearest.map(|(id, _)| id)
}

// claude code: makes every hitbox the attacker has out over fresh: the same data under the same id and
// group, with an empty hit log, so it can hit everything again, the defender included. a hitbox's data is
// copied out before AttackModule::clear, since the game's AttackData is bigger than the part skyline-smash
// knows (0x100 bytes covers it). false = nothing to make over
unsafe fn rehit_breaking_hitboxes(fighter: &mut L2CFighterCommon) -> bool {
    let Some(attacker_id) = shield_breaker(fighter) else { return false };
    let attacker = sv_battle_object::module_accessor(attacker_id);
    let mut count = 0;
    for id in 0..8 {
        if !AttackModule::is_attack(attacker, id, false) {
            continue;
        }
        let data = AttackModule::attack_data(attacker, id, false) as *const u64;
        let mut copy = [0u64; 0x20];
        std::ptr::copy_nonoverlapping(data, copy.as_mut_ptr(), copy.len());
        let group = AttackModule::group(attacker, id) as i32;
        AttackModule::clear(attacker, id, false);
        AttackModule::set_attack(attacker, id, group, copy.as_ptr() as *const smash::app::AttackData);
        count += 1;
    }
    count > 0
}

// claude code: SHIELD_BREAK_FLY is written over in place: a grounded standstill (no launch, no stun),
// whose first frame lands the stored hit or hands control back
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_ShieldBreakFly)]
unsafe fn status_pre_ShieldBreakFly(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::common::instance::SHIELD_BROKEN);
    rehit_breaking_hitboxes(fighter);
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

// claude code: vanilla's init launches the fighter upward; nothing happens here instead
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_ftStatusUniqProcessShieldBreakFly_initStatus)]
unsafe fn sub_ftStatusUniqProcessShieldBreakFly_initStatus(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_ShieldBreakFly)]
unsafe fn status_ShieldBreakFly(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(shield_break_main as *const () as _))
}

unsafe extern "C" fn shield_break_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next = if fighter.is_situation(*SITUATION_KIND_GROUND) {
        *FIGHTER_STATUS_KIND_WAIT
    } else {
        *FIGHTER_STATUS_KIND_FALL
    };
    fighter.change_status(next.into(), false.into());
    0.into()
}

// claude code: no shield for the rest of the stock. the shield inputs are switched off (transition.rs)
pub unsafe fn is_shield_broken(boma: &mut BattleObjectModuleAccessor) -> bool {
    VarModule::is_flag(boma.object(), vars::common::instance::SHIELD_BROKEN)
}

// claude code: a new stock gets its shield back. runs every frame from sys_line_system_control_fighter
// (opff.rs); shield_reset covers a fighter starting over without dying (a training reset, say)
pub unsafe fn shield_respawn(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_ENTRY
    ]) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::SHIELD_BROKEN);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftStatusUniqProcessGuardOn_initStatus,
            status_GuardOff_Common,
            FighterStatusGuard__check_hit_stop_delay_flick,
            status_pre_ShieldBreakFly,
            sub_ftStatusUniqProcessShieldBreakFly_initStatus,
            status_ShieldBreakFly
        );
    }
}

// claude code: a fighter starting (or restarting) gets its shield back
unsafe extern "C" fn shield_reset(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::SHIELD_BROKEN);
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    smashline::Agent::new("fighter")
        .on_init(shield_reset)
        .on_start(shield_reset)
        .install();
}
