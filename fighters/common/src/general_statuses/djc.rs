use super::*;

// hdr: based on HDR's double jump cancel (fighters/common/src/djc.rs)
unsafe fn dj_cancel(fighter: &mut L2CFighterCommon) {
    fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS_2ND, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y.clamp(-2.5, 2.5));
}

// claude code: DJC for statuses other than aerials (Zair, specials). djc_inherit_pre goes in the
// status's Pre, while the double jump's motion is still playing, and keeps that motion going as a
// second motion. djc_inherit_frame runs every frame from the fighter's opff, after the status code:
// these statuses set their own air kinetic on their first frame and again in each status they chain
// into, so each time that happens it's saved and swapped for the double jump's kinetic, like an
// aerial's DJC. letting go of jump in the first frames, or the jump running out, gives the status
// its own kinetic back. `statuses` is the special's whole chain (start, holds, ends).
//
// the jump running out used to be read off the second motion reaching its end frame. a held status
// (PSI Magnet, PK Fire's...) can leave that motion parked a frame short of the end or not playing at
// all, so the check never passed and the double jump's kinetic, with nothing left to move it, stayed
// on: a float in place. humans never saw it, since letting go of jump early ends the DJC first; CPUs
// hold the button and hit it every time. now the jump's frames are counted down too (DJC_FRAMES_LEFT,
// set from what's left of the jump motion when the status starts, paused during hitstop), and the DJC
// ends when either says so
pub unsafe fn djc_inherit_pre(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::common::instance::DJC_KINETIC, -1);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR
    || (motion_kind != hash40("jump_aerial_f") && motion_kind != hash40("jump_aerial_b"))
    || fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
        return;
    }
    let frame = fighter.motion_frame();
    let held = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || frame < 2.0;
    if !held {
        return;
    }
    let frames_left = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(motion_kind)) - frame;
    if frames_left < 1.0 {
        return;
    }
    MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
    MotionModule::set_weight(fighter.module_accessor, 1.0, true);
    VarModule::set_int(fighter.battle_object, vars::common::instance::DJC_KINETIC, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
    VarModule::set_int(fighter.battle_object, vars::common::instance::DJC_FRAMES_LEFT, frames_left.ceil() as i32);
}

pub unsafe fn djc_inherit_frame(fighter: &mut L2CFighterCommon, statuses: &[i32]) {
    let status_kinetic = VarModule::get_int(fighter.battle_object, vars::common::instance::DJC_KINETIC);
    if status_kinetic < 0 {
        return;
    }
    let boma = fighter.module_accessor;
    if !statuses.contains(&StatusModule::status_kind(boma))
    || fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        VarModule::set_int(fighter.battle_object, vars::common::instance::DJC_KINETIC, -1);
        return;
    }
    let kinetic = KineticModule::get_kinetic_type(boma);
    let mut status_kinetic = status_kinetic;
    if kinetic != *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND {
        status_kinetic = kinetic;
        VarModule::set_int(fighter.battle_object, vars::common::instance::DJC_KINETIC, kinetic);
    }
    let mut frames_left = VarModule::get_int(fighter.battle_object, vars::common::instance::DJC_FRAMES_LEFT);
    if !StopModule::is_stop(boma) {
        frames_left -= 1;
        VarModule::set_int(fighter.battle_object, vars::common::instance::DJC_FRAMES_LEFT, frames_left);
    }
    let motion_2nd = MotionModule::motion_kind_2nd(boma);
    let jump_over = frames_left <= 0
        || (motion_2nd != hash40("jump_aerial_f") && motion_2nd != hash40("jump_aerial_b"))
        || MotionModule::frame_2nd(boma) + 1.0 >= MotionModule::end_frame_from_hash(boma, Hash40::new_raw(motion_2nd));
    let released = MotionModule::frame_2nd(boma) >= 2.0
        && fighter.global_table[CURRENT_FRAME].get_i32() <= 6
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP);
    if jump_over || released {
        VarModule::set_int(fighter.battle_object, vars::common::instance::DJC_KINETIC, -1);
        if kinetic == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND
        && status_kinetic != *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND {
            KineticModule::change_kinetic(boma, status_kinetic);
        }
    } else if kinetic != *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
    }
}

unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(false.into());
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_AttackAir_Main_common().get_bool() {
        return 1.into();
    }
    fighter.sub_air_check_superleaf_fall_slowly();
    if !fighter.global_table[IS_STOPPING].get_bool() {
        fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
    }
    0.into()
}

unsafe extern "C" fn attack_air_main_float(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
    }
    attack_air_main(fighter)
}

unsafe extern "C" fn attack_air_init_inherit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_init();
    0.into()
}

unsafe extern "C" fn attack_air_exec_inherit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec();
    0.into()
}

unsafe extern "C" fn attack_air_exit_inherit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exit();
    0.into()
}

unsafe extern "C" fn attack_air_pre_demon(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_init)]
unsafe fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_DEMON {
        return call_original!(fighter);
    }
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = fighter.motion_frame();

    fighter.sub_attack_air_kind();
    if (motion_kind == hash40("jump_aerial_f") || motion_kind == hash40("jump_aerial_b"))
    && !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
        MotionModule::set_weight(fighter.module_accessor, 1.0, true);
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || frame < 2.0 {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
        } else {
            dj_cancel(fighter);
        }
    } else {
        fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    let _ = fighter.sub_attack_air_uniq_process_init();
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec)]
unsafe fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND
    && fighter.global_table[FIGHTER_KIND] != FIGHTER_KIND_DEMON
    && MotionModule::frame_2nd(fighter.module_accessor) >= 2.0
    && fighter.global_table[CURRENT_FRAME].get_i32() <= 6
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        dj_cancel(fighter);
    }
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_init,
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    for kind in ["ness", "lucas", "mewtwo", "trail"] {
        smashline::Agent::new(kind)
            .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main)
            .install();
    }
    for kind in ["peach", "daisy"] {
        smashline::Agent::new(kind)
            .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main_float)
            .install();
    }
    smashline::Agent::new("yoshi")
        .status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_init_inherit)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main)
        .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_exec_inherit)
        .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_exit_inherit)
        .install();
    smashline::Agent::new("demon")
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre_demon)
        .install();
}
