use super::*;

// claude code: the parry window length, common `shield_just_frame`, shared by both ends of shielding
// so the two windows always match
pub unsafe fn parry_window_frames(boma: &mut BattleObjectModuleAccessor) -> i32 {
    boma.get_param_int("common", "shield_just_frame")
}

// claude code: turns the shield into a parry for `shield_just_frame` frames. it reflects any projectile
// with no multipliers (1x damage, speed, lifetime, no hitlag) through the shield's own just-shield reflector
// group, like HDR's parry. the game only counts a shield hit as a parry while JUST_FRAME is above 0.
// the opff `parry_window` counts it down
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

// claude code: the parry reward. the defender heals half their missing shield and can act on the very
// next frame: hitlag is cancelled and they're forced to WAIT, which allows every grounded option. shield
// held through the parry gets no startup parry until it's released, or holding shield would chain
// parries through every hit of a multi-hit move. runs from sys_line_system_control_fighter, which
// still runs during hitlag
pub unsafe fn parry_reward(fighter: &mut L2CFighterCommon) {
    let is_shield_held = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD);
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

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftStatusUniqProcessGuardOn_initStatus,
            status_GuardOff_Common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
