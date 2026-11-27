use super::*;

// hdr: the two stick flick counters are based on HDR's (fighters/common/src/opff/other.rs)
pub extern "C" fn left_stick_flick_counter(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.left_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X, u8::MAX as i32 - 1);
        } else if fighter.left_stick_x().signum() != fighter.prev_left_stick_x().signum() || fighter.prev_left_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X);
        }
        
        if fighter.left_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y, u8::MAX as i32 - 1);
        } else if fighter.left_stick_y().signum() != fighter.prev_left_stick_y().signum()
        || fighter.prev_left_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y);
        }
    }
}

pub extern "C" fn right_stick_flick_counter(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.right_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X, u8::MAX as i32 - 1);
        } else if fighter.right_stick_x().signum() != fighter.prev_right_stick_x().signum() || fighter.prev_right_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X);
        }
        
        if fighter.right_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y, u8::MAX as i32 - 1);
        } else if fighter.right_stick_y().signum() != fighter.prev_right_stick_y().signum() || fighter.prev_right_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y);
        }
    }
}

pub unsafe extern "C" fn dash_land(fighter: &mut L2CFighterCommon) {
    if !fighter.is_situation(*SITUATION_KIND_AIR)
    || !(fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL
    ]) || CancelModule::is_enable_cancel(fighter.module_accessor)) {
        return;
    }
    let stick_x = fighter.left_stick_x();
    if stick_x.abs() < fighter.get_param_float("common", "dash_stick_x")
    || VarModule::get_int(fighter.object(), vars::common::instance::LEFT_STICK_FLICK_X) > fighter.get_param_int("common", "dash_flick_x") {
        return;
    }
    let pos = *PostureModule::pos(fighter.module_accessor);
    let bottom_offset = VarModule::get_float(fighter.object(), vars::common::instance::ECB_BOTTOM_Y_OFFSET);
    let center_offset = VarModule::get_float(fighter.object(), vars::common::instance::ECB_CENTER_Y_OFFSET);
    let top = Vector2f::new(pos.x, pos.y + (center_offset * 2.0) - bottom_offset);
    let bottom = Vector2f::new(pos.x, pos.y + bottom_offset - 0.01);
    let platform_pos = &mut Vector2f::zero();
    let stage_pos = &mut Vector2f::zero();
    let hit_any = GroundModule::line_segment_check(fighter.module_accessor, &top, &bottom, &Vector2f::zero(), platform_pos, true);
    let hit_stage = GroundModule::line_segment_check(fighter.module_accessor, &top, &bottom, &Vector2f::zero(), stage_pos, false);
    if hit_any == 0 as *const *const u64
    || hit_stage != 0 as *const *const u64 {
        return;
    }
    GroundModule::clear_pass_floor(fighter.module_accessor);
    PostureModule::set_pos(fighter.module_accessor, &Vector3f::new(pos.x, platform_pos.y + 0.1, pos.z));
    GroundModule::attach_ground(fighter.module_accessor, false);
    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    PostureModule::set_lr(fighter.module_accessor, stick_x.signum());
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.change_status_req(*FIGHTER_STATUS_KIND_DASH, true);
}

pub unsafe extern "C" fn pass_statuses(fighter: &mut L2CFighterCommon) {
    if GroundModule::is_passable_ground(fighter.module_accessor)
    && VarModule::get_int(fighter.object(), vars::common::instance::LEFT_STICK_FLICK_Y) < 4
    && fighter.left_stick_y() < fighter.get_param_float("common", "pass_stick_y")
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD_OFF
    ]) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
    }

    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && VarModule::get_int(fighter.object(), vars::common::instance::LEFT_STICK_FLICK_Y) < 4
    && fighter.left_stick_y() < fighter.get_param_float("common", "pass_stick_y") {
        GroundModule::clear_pass_floor(fighter.module_accessor);
    }
}

pub unsafe extern "C" fn attack_air_fastfall_hitfall(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) 
    && !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.left_stick_y() <= -0.6 && VarModule::get_int(fighter.object(), vars::common::instance::LEFT_STICK_FLICK_Y) < 4
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            fighter.on_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        } else {
            fighter.sub_air_check_dive();
        }
    }
}

// hdr: based on HDR's sys_line hook (fighters/common/src/opff/mod.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sys_line_system_control_fighter)]
pub unsafe fn sys_line_system_control_fighter_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::common::instance::STATUS_REQUESTS_THIS_FRAME, 0);
    let ret = original!()(fighter);
    left_stick_flick_counter(fighter);
    right_stick_flick_counter(fighter);
    general_statuses::shield::parry_reward(fighter);
    general_statuses::damage::bounce_collision(fighter);
    general_statuses::damage::record_launch_speed(fighter);
    general_statuses::down::forced_down(fighter);
    general_statuses::sdi_reset(fighter);
    general_statuses::shield::shield_respawn(fighter);
    function_hooks::lua_bind_hook::slide_off_ground_globals(fighter);
    modes::faux_air_frame(fighter);
    ret
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sys_line_system_control_fighter_hook
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    smashline::Agent::new("fighter")
        .on_line(Main, dash_land)
        .on_line(Main, function_hooks::lua_bind_hook::slide_off_air)
        .on_line(Main, function_hooks::lua_bind_hook::slide_off_own_status)
        .on_line(Main, pass_statuses)
        .on_line(Main, general_statuses::shield::parry_window)
        .on_line(Main, general_statuses::walljump::wall_jump_count_reset)
        .on_line(Main, general_statuses::walljump::wall_jump_anytime)
        .on_line(Main, function_hooks::lua_bind_hook::no_special_fall)
        .on_line(Main, general_statuses::damage::non_tumble_di)
        .on_line(Main, general_statuses::damage::drift_di)
        .on_line(Main, attack_air_fastfall_hitfall)
        .on_line(Main, modes::aura_fighter_frame)
        .install();
}
