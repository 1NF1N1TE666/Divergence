use super::*;
use globals::*;

#[path = "walk-dash-run-turn.rs"]
mod walk_dash_run_turn;
mod attackx4;
pub mod damage;
pub mod djc;
pub mod down;
mod downdamage;
pub mod lasso;
pub mod shield;
pub mod walljump;

// claude code: held SDI, shared by hitstun (damage.rs) and shield (shield.rs). no mashing: holding the
// stick past common `hit_stop_delay_stick` nudges the fighter once every `hit_stop_delay_flick_frame`
// frames of hitlag, up to `hit_stop_delay_flick_max_count` nudges per hitlag. each nudge is the stick
// times `hit_stop_delay_flick_mul` times `mul` (the hit's own SDI multiplier, and the shield's
// `hit_stop_delay_guard_mul` on a shield). Some((x, y)) = nudge this frame. the counters reset whenever
// the fighter isn't in hitlag (sdi_reset)
pub unsafe fn held_sdi(fighter: &mut L2CFighterCommon, mul: f32) -> Option<(f32, f32)> {
    let wait = VarModule::get_int(fighter.battle_object, vars::common::instance::SDI_WAIT);
    if wait > 0 {
        VarModule::dec_int(fighter.battle_object, vars::common::instance::SDI_WAIT);
        return None;
    }
    let count = VarModule::get_int(fighter.battle_object, vars::common::instance::SDI_COUNT);
    if count >= fighter.get_param_int("common", "hit_stop_delay_flick_max_count") {
        return None;
    }
    let stick_x = fighter.left_stick_x();
    let stick_y = fighter.left_stick_y();
    if (stick_x * stick_x + stick_y * stick_y).sqrt() < fighter.get_param_float("common", "hit_stop_delay_stick") {
        return None;
    }
    VarModule::inc_int(fighter.battle_object, vars::common::instance::SDI_COUNT);
    let interval = fighter.get_param_int("common", "hit_stop_delay_flick_frame");
    VarModule::set_int(fighter.battle_object, vars::common::instance::SDI_WAIT, (interval - 1).max(0));
    let strength = fighter.get_param_float("common", "hit_stop_delay_flick_mul") * mul;
    Some((stick_x * strength, stick_y * strength))
}

// claude code: moves the fighter by an SDI nudge. a grounded fighter isn't pushed down into the floor
pub unsafe fn apply_sdi(fighter: &mut L2CFighterCommon, x: f32, y: f32) {
    let y = if fighter.is_situation(*SITUATION_KIND_GROUND) { y.max(0.0) } else { y };
    let mut pos = *PostureModule::pos(fighter.module_accessor);
    pos.x += x;
    pos.y += y;
    PostureModule::set_pos(fighter.module_accessor, &pos);
}

// claude code: held SDI's counters start over with each hitlag or bounce freeze. runs every frame from
// sys_line_system_control_fighter (opff.rs)
pub unsafe fn sdi_reset(fighter: &mut L2CFighterCommon) {
    if !StopModule::is_stop(fighter.module_accessor)
    && !damage::is_bounce_freeze(fighter) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::SDI_COUNT, 0);
        VarModule::set_int(fighter.battle_object, vars::common::instance::SDI_WAIT, 0);
    }
}

// claude code: the wall jump test itself, without the fighter's own callback (global 0x31) the hook
// below runs first. that callback belongs to the statuses that set it up; wall_jump_anytime
// (walljump.rs) calls this from any air state, and on 2026-11-19 a CPU Kazuya crashed the game there with
// a stack overflow, the callback looping back into the wall jump check. true = wall jumping now
pub unsafe fn try_wall_jump(fighter: &mut L2CFighterCommon) -> bool {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || fighter.is_status(*FIGHTER_STATUS_KIND_WALL_JUMP)
    || fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_COUNT) >= 1
    || fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_WALL_JUMP_FRAME) > 0 {
        return false;
    }

    let attach_wall_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attach_wall_type"), 0);
    if attach_wall_type == *FIGHTER_ATTACH_WALL_TYPE_NORMAL
    && fighter.sub_fighter_general_term_is_can_attach_wall().get_bool() {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACH_WALL.into(), true.into());
        return true;
    }

    let wall_jump_type = WorkModule::get_param_int(fighter.module_accessor, hash40("wall_jump_type"), 0);
    if wall_jump_type == *FIGHTER_WALL_JUMP_TYPE_NORMAL
    && fighter.boma().is_button_trigger(Buttons::Jump)
    && face_wall(fighter) {
        // the press is the wall jump's, so it can't also start a double jump this frame
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP_BUTTON);
        fighter.change_status(FIGHTER_STATUS_KIND_WALL_JUMP.into(), true.into());
        return true;
    }

    return false;
}

// claude code: Divergence's wall jump input (2026-11-27, Grant's design): a wall beside them, upright or
// slanted up to lasso::WALL_ANGLE_MAX off upright, and a press of the jump button. it replaces vanilla's
// check (sub_fighter_general_term_is_can_wall_jump, decompiled 2026-11-27), which wanted a flick away from
// the wall (wall_jump_stick_x within wall_jump_flick_x frames), a slow enough speed off it
// (wall_jump_enable_x_speed), and only took what the engine calls a wall touch (GroundModule::is_touch
// LEFT/RIGHT + is_attachable), which leaves slanted walls out. the wall is found the way the tether's
// lasso::find_wall finds one: a ray, the angle read off the wall's normal, here 4 and 10 units up from
// their feet and 8 out to each side (all times their scale). the nearest wall wins. wall clings
// (attach_wall_type, above) keep vanilla's check
const WALL_REACH: f32 = 8.0;
const WALL_HEIGHTS: [f32; 2] = [4.0, 10.0];

// true = a wall is beside them, and they're turned to face it, the way a wall jump starts
unsafe fn face_wall(fighter: &mut L2CFighterCommon) -> bool {
    let boma = fighter.module_accessor;
    let scale = PostureModule::scale(boma);
    let x = PostureModule::pos_x(boma);
    let mut nearest: Option<(f32, f32)> = None; // (distance, side: 1 = wall on the right)
    for side in [-1.0f32, 1.0] {
        for height in WALL_HEIGHTS {
            let start = Vector2f::new(x, PostureModule::pos_y(boma) + height * scale);
            let ray = Vector2f::new(WALL_REACH * side * scale, 0.0);
            let pos = &mut Vector2f::zero();
            let normal = &mut Vector2f::zero();
            if GroundModule::ray_check_hit_pos_normal(boma, &start, &ray, pos, normal, true) & 1 != 0
            && normal.x * side < 0.0
            && normal.y.atan2(normal.x.abs()).to_degrees().abs() < lasso::WALL_ANGLE_MAX {
                let distance = (pos.x - x).abs();
                if nearest.map_or(true, |(best, _)| distance < best) {
                    nearest = Some((distance, side));
                }
            }
        }
    }
    let Some((_, side)) = nearest else { return false };
    PostureModule::set_lr(boma, side);
    PostureModule::update_rot_y_lr(boma);
    true
}

// hdr: based on HDR's version (fighters/common/src/general_statuses/mod.rs)
#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_wall_jump)]
unsafe fn sub_transition_group_check_air_wall_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x31].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x31].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    try_wall_jump(fighter).into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Landing_MainSub)]
pub unsafe fn status_Landing_MainSub(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    call_original!(fighter)
}

// hdr: HDR's version (fighters/common/src/general_statuses/mod.rs)
#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground)]
unsafe fn sub_transition_group_check_ground(fighter: &mut L2CFighterCommon, to_squat_wait: L2CValue) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let cat2 = fighter.global_table[CMD_CAT2].get_i32();
        if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
            return true.into();
        }
        if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
            return true.into();
        }
        if cat2 & (*FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
            return true.into();
        }
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
            return true.into();
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) {
            fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT)
        && fighter.sub_check_command_squat().get_bool() {
            let status = if to_squat_wait.get_bool() {
                FIGHTER_STATUS_KIND_SQUAT_WAIT
            }
            else {
                FIGHTER_STATUS_KIND_SQUAT
            };
            fighter.change_status(status.into(), true.into());
            return true.into();
        }

        if fighter.left_stick_x() * fighter.lr() <= fighter.get_param_float("common", "turn_stick_x")
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN) {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK)
        && fighter.sub_check_command_walk().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

// hdr: based on HDR's version (fighters/common/src/general_statuses/mod.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_FallSub)]
pub unsafe fn status_FallSub_hook(fighter: &mut L2CFighterCommon, arg2: L2CValue) {
    call_original!(fighter, arg2);
    let move_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    if move_speed * fighter.lr() < 0.0
    && !fighter.boma().is_motion(Hash40::new("fall")) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
        fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL);
    }
}

// hdr: based on HDR's version (fighters/common/src/general_statuses/mod.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_end_pass_ground)]
pub unsafe fn end_pass_ground(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_DASH
    && (fighter.kind() != *FIGHTER_KIND_RYU || fighter.global_table[PREV_STATUS_KIND] != FIGHTER_RYU_STATUS_KIND_DASH_BACK)
    && (fighter.kind() != *FIGHTER_KIND_KEN || fighter.global_table[PREV_STATUS_KIND] != FIGHTER_RYU_STATUS_KIND_DASH_BACK)
    && (fighter.kind() != *FIGHTER_KIND_DOLLY || fighter.global_table[PREV_STATUS_KIND] != FIGHTER_DOLLY_STATUS_KIND_DASH_BACK)
    && (fighter.kind() != *FIGHTER_KIND_DEMON || fighter.global_table[PREV_STATUS_KIND] != FIGHTER_DEMON_STATUS_KIND_DASH_BACK) {
        fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    }
    call_original!(fighter)
}

// hdr: based on HDR's version (fighters/common/src/general_statuses/mod.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_is_dive)]
pub unsafe fn sub_is_dive(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
        return false.into();
    }

    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let status_kind = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();

    if [*FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
    ].contains(&status_kind) {
        return false.into();
    }

    if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    || KineticModule::is_suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
        return false.into();
    }

    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_y >= 0.0 {
        return false.into();
    }

    let dive_cont_value = fighter.get_param_float("common", "dive_cont_value");
    let dive_flick_frame_value = fighter.get_param_int("common", "dive_flick_frame_value");

    if fighter.left_stick_y() > dive_cont_value
    || VarModule::get_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y) >= dive_flick_frame_value {
        return false.into();
    }

    let dive_speed_y = fighter.get_param_float("dive_speed_y", "");
    if speed_y < -dive_speed_y {
        return false.into();
    }

    if [*FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION,
        *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND,
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor))
    {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        app::sv_kinetic_energy::enable(fighter.lua_state_agent);

        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    }

    true.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_Landing_MainSub,
            sub_transition_group_check_ground,
            status_FallSub_hook,
            end_pass_ground,
            sub_is_dive,
            sub_transition_group_check_air_wall_jump
        );
    }
}

pub fn install() {
    walk_dash_run_turn::install();
    attackx4::install();
    damage::install();
    djc::install();
    down::install();
    downdamage::install();
    lasso::install();
    shield::install();
    walljump::install();

    skyline::nro::add_hook(nro_hook);
}
