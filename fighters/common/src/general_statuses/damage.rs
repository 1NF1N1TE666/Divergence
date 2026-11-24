use super::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    // hdr: HDR's patch (fighters/common/src/general_statuses/damage.rs)
    skyline::patching::Patch::in_text(0x63251c).nop();
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            FighterStatusUniqProcessDamage_leave_stop_hook,
            status_DamageFly_Main_hook,
            exec_damage_elec_hit_stop_hook,
            virtual_ftStatusUniqProcessDamage_exec_common,
            FighterStatusDamage__correctDamageVectorEffect,
            damage_fly_common_init
        );
    }
}

// hdr: based on HDR's version (fighters/common/src/general_statuses/damage.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusUniqProcessDamage_leave_stop)]
pub unsafe fn FighterStatusUniqProcessDamage_leave_stop_hook(fighter: &mut L2CFighterCommon, arg2: L2CValue, arg3: L2CValue) -> L2CValue {
    let status_kind = fighter.status();
    if !arg3.get_bool() {
        return 0.into();
    }
    let control_module = *(fighter.module_accessor as *const u64).offset(0x48 / 8) as *const u64;
    let vtable = *control_module;
    let control_module__update: extern "C" fn(*const u64, bool) = std::mem::transmute(*(((vtable as u64) + 0x148) as *const u64));
    control_module__update(control_module, false);
    ShakeModule::stop(fighter.module_accessor);
    let hashmap = fighter.local_func__fighter_status_damage_2();
    FighterUtil::cheer_damage(fighter.module_accessor);
    fighter.check_ryu_final_damage_03(L2CValue::Bool(true));
    let release_action = fighter.get_int(*FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
    if release_action == *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_GROUND_TO_AIR {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
        fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_AIR));
        fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    }
    fighter.set_int(*FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_NONE, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
    let mut damage_motion_kind = fighter.get_int64(*FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
    let mut start_frame = 0.0;
    if damage_motion_kind == hash40("damage_fly_roll") {
        if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
            damage_motion_kind = hash40("damage_fly_n");
        }
    }
    let damage_lr = fighter.get_float(*FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
    if damage_lr != 0.0 {
        if damage_lr * fighter.lr() >= 0.0 {
            PostureModule::set_lr(fighter.module_accessor, damage_lr);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        } else if [*FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
            PostureModule::set_lr(fighter.module_accessor, damage_lr);
            PostureModule::update_rot_y_lr(fighter.module_accessor);   
        } else {
            if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
                let lr = fighter.lr();
                TurnModule::set_turn(fighter.module_accessor, Hash40::new("back_damage"), lr, false, false, true);
                PostureModule::reverse_lr(fighter.module_accessor);
                let back_damage_effective_frame = fighter.get_param_int("common", "back_damage_effective_frame");
                fighter.set_int(back_damage_effective_frame, *FIGHTER_INSTANCE_WORK_ID_INT_BACK_DAMAGE_EFFECTIVE_FRAME);
            }
        }
        fighter.set_float(0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
    }
    if damage_motion_kind != hash40("invalid") {
        if damage_motion_kind == hash40("wall_damage") {
            start_frame = fighter.get_param_float("common", "wall_damage_start_frame");
            if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("wall_damage")) {
                start_frame -= 1.0;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            if fighter.global_table[DAMAGE_MOTION_KIND_CALLBACK].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(fighter.global_table[DAMAGE_MOTION_KIND_CALLBACK].get_ptr());
                damage_motion_kind = callable(fighter, L2CValue::U64(damage_motion_kind)).get_u64();
            }
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(damage_motion_kind), start_frame, 1.0, false, 0.0, false, false);
        if status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL {
            if [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
                let is_pierce = fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_TO_PIERCE);
                let rate = fighter.calc_damage_motion_rate(L2CValue::U64(damage_motion_kind), L2CValue::F32(start_frame), L2CValue::Bool(is_pierce)).get_f32();
                MotionModule::set_rate(fighter.module_accessor, rate);
                let damage_fly_angle_compose = fighter.sub_FighterStatusDamage_get_damage_fly_angle_compose().get_i32();
                let damage_fly_angle = FighterUtil::set_damage_fly_angle(fighter.module_accessor, 0.0, 1.0, 360.0, MotionNodeRotateCompose{_address: damage_fly_angle_compose as u8});
                fighter.set_float(damage_fly_angle, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_ROT_ANGLE);
                fighter.on_flag(*FIGHTER_STATUS_DAMAGE_FLAG_FLY_ROLL_SET_ANGLE);
                WorkModule::set_int64(fighter.module_accessor, hash40("invalid"), *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
                fighter.on_flag(*FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN);
                return 0.into();
            }
        } else {
            if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
                let damage_fly_angle_compose = fighter.sub_FighterStatusDamage_get_damage_fly_angle_compose().get_i32();
                let damage_fly_angle = FighterUtil::set_damage_fly_angle(fighter.module_accessor, 0.0, 1.0, 180.0, MotionNodeRotateCompose{_address: damage_fly_angle_compose as u8});
                fighter.set_float(damage_fly_angle, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_ROT_ANGLE);
                fighter.on_flag(*FIGHTER_STATUS_DAMAGE_FLAG_FLY_ROLL_SET_ANGLE);
            }
            let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(damage_motion_kind), true);
            if cancel_frame <= 0.0 {
                cancel_frame = MotionModule::end_frame(fighter.module_accessor);
            }
            let reaction_frame_mul_speed_up = fighter.reaction_frame_mul_speed_up().get_f32();
            if 0.0 < reaction_frame_mul_speed_up {
                if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
                    let something = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x255c556cd3);
                    let mut frame = reaction_frame_mul_speed_up - something;
                    frame %= cancel_frame;
                    if 0.0 < frame {
                        MotionModule::set_frame(fighter.module_accessor, frame, true);
                    }
                } else {
                    let rate = cancel_frame / reaction_frame_mul_speed_up;
                    MotionModule::set_rate(fighter.module_accessor, rate);
                }
            }
        }
        WorkModule::set_int64(fighter.module_accessor, hash40("invalid"), *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
    }
    fighter.on_flag(*FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN);
    0.into()
}

// hdr: based on HDR's version (fighters/common/src/general_statuses/damage.rs)
#[skyline::hook(replace = L2CFighterCommon_status_DamageFly_Main)]
unsafe fn status_DamageFly_Main_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
        if fighter.motion_frame() >= (MotionModule::end_frame(fighter.module_accessor) - 1.0) && MotionModule::rate(fighter.module_accessor) != 0.0 {
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL) 
        && fighter.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
            return 0.into();
        }
        if fighter.sub_DamageFlyCommon().get_bool() {
            return 0.into();
        }
        if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
            if fighter.sub_AirChkDamageReflectWall().get_bool()
            || fighter.sub_AirChkDamageReflectCeil().get_bool()
            || fighter.sub_AirChkDamageReflectFloor().get_bool() {
                return 0.into();
            }
        }
        fighter.FighterStatusDamage__correctDamageVectorEffect(L2CValue::Bool(false));
    } else {
        if !fighter.status_DamageFinishCamera_exec().get_bool() {
            return 0.into();
        }
        fighter.status_DamageFly_Common();
        fighter.off_flag(*FIGHTER_STATUS_DAMAGE_FLAG_ADJUST_VECTOR);
    }
    0.into()
}

// hdr: based on HDR's version (fighters/common/src/general_statuses/damage.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_exec_damage_elec_hit_stop)]
pub unsafe fn exec_damage_elec_hit_stop_hook(fighter: &mut L2CFighterCommon) {
    let status_kind = fighter.status();
    let hit_stop_frame = fighter.get_int(*FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME);
    if hit_stop_frame > 0 {
        fighter.dec_int(*FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME);
    }
    fighter.sub_FighterStatusDamage_correctDamageVectorExecStop();
    if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_KOZUKATA_DAMAGE) {
        let clatter_time = ControlModule::get_clatter_time(fighter.module_accessor, 0);
        if clatter_time <= 0.0 {
            fighter.set_int(0, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME);
            ShakeModule::stop(fighter.module_accessor);
        }
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("absolute") as u64);
    sv_information::damage_log_value(fighter.lua_state_agent);
    let is_paralyze = fighter.pop_lua_stack(1).get_bool();
    let hashmap = fighter.local_func__fighter_status_damage_2();
    if hit_stop_frame > 0 {
        if is_paralyze
        && fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_PARALYZE_STOP) {
            return;
        }
    } else {
        ShakeModule::stop(fighter.module_accessor);
        fighter.off_flag(*FIGHTER_STATUS_DAMAGE_FLAG_ELEC);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        if fighter.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_KINE_GRAVITY) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        let release_action = fighter.get_int(*FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
        if release_action == *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_GROUND_TO_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
            fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_AIR));
            fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
        fighter.set_int(*FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_NONE, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
        fighter.virtual_ftStatusUniqProcessDamage_init(L2CValue::Bool(true));
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new_raw(0x244371e88f));
        smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        let damage_lr = fighter.get_float(*FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
        if damage_lr != 0.0 {
            if damage_lr * fighter.lr() >= 0.0 {
                PostureModule::set_lr(fighter.module_accessor, damage_lr);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            } else if [*FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
                PostureModule::set_lr(fighter.module_accessor, damage_lr);
                PostureModule::update_rot_y_lr(fighter.module_accessor);   
            } else {
                if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
                    let lr = fighter.lr();
                    TurnModule::set_turn(fighter.module_accessor, Hash40::new("back_damage"), lr, false, false, true);
                    PostureModule::reverse_lr(fighter.module_accessor);
                    let back_damage_effective_frame = fighter.get_param_int("common", "back_damage_effective_frame");
                    fighter.set_int(back_damage_effective_frame, *FIGHTER_INSTANCE_WORK_ID_INT_BACK_DAMAGE_EFFECTIVE_FRAME);
                }
            }
            fighter.set_float(0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
        }
        if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_PARALYZE_EFFECT) {
            fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_PARALYZE_EFFECT);
        }
        fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_PARALYZE_STOP);
        fighter.on_flag(*FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN);
    }
}

// hdr: HDR's version (fighters/common/src/general_statuses/mod.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_virtual_ftStatusUniqProcessDamage_exec_common)]
pub unsafe fn virtual_ftStatusUniqProcessDamage_exec_common(fighter: &mut L2CFighterCommon) {
    if [*FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_LUIGI_FINAL_SHOOT,
        ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        fighter.ftStatusUniqProcessDamageFly_exec_common();
    }
}

// hdr: HDR's version (fighters/common/src/general_statuses/mod.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusDamage__correctDamageVectorEffect)]
pub unsafe fn FighterStatusDamage__correctDamageVectorEffect(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.global_table[STATUS_KIND_INTERRUPT] != FIGHTER_STATUS_KIND_DAMAGE_AIR {
        return call_original!(fighter, param_1);
    }
    fighter.global_table[STATUS_KIND_INTERRUPT].assign(&L2CValue::I32(*FIGHTER_STATUS_KIND_DAMAGE_FLY));
    let ret = call_original!(fighter, param_1);
    fighter.global_table[STATUS_KIND_INTERRUPT].assign(&L2CValue::I32(*FIGHTER_STATUS_KIND_DAMAGE_AIR));
    ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_DamageFlyCommon_init)]
pub unsafe fn damage_fly_common_init(fighter: &mut L2CFighterCommon) {
    let force_knockdown = VarModule::is_flag(fighter.battle_object, vars::common::instance::FORCE_KNOCKDOWN);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::FORCE_KNOCKDOWN);
    if force_knockdown {
        fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D);
    }
    original!()(fighter);
    // claude code: FORCE_KNOCKDOWN also takes away the tech for this launch, so it always ends lying down
    if force_knockdown {
        fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D);
        fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
        fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
    }
}
