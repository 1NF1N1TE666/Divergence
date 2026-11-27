use super::*;

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
    check_asdi(fighter, hashmap["stop_delay_"].get_f32());
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
                fighter.boma().set_rate(rate);
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
                    fighter.boma().set_rate(rate);
                }
            }
        }
        WorkModule::set_int64(fighter.module_accessor, hash40("invalid"), *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
    }
    fighter.on_flag(*FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN);
    0.into()
}

// hdr: HDR's ASDI (check_asdi, fighters/common/src/general_statuses/damage.rs): as hitlag ends, the
// held stick moves the fighter by `hit_stop_delay_auto_mul` times the hit's SDI multiplier, on every hit.
// none after a throw, as in HDR. bounces get theirs from bounce_release
unsafe fn check_asdi(fighter: &mut L2CFighterCommon, sdi_mul: f32) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
    ])
    || fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_THROWN {
        return;
    }
    // claude code: not on electric hits
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("attr"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let attr = fighter.pop_lua_stack(1).get_u64();
    if attr == hash40("collision_attr_elec") {
        return;
    }
    asdi(fighter, sdi_mul);
}

// claude code: the ASDI nudge itself, shared by hits (check_asdi) and bounces (bounce_release)
unsafe fn asdi(fighter: &mut L2CFighterCommon, sdi_mul: f32) {
    let stick_x = fighter.left_stick_x();
    let stick_y = fighter.left_stick_y();
    if (stick_x * stick_x + stick_y * stick_y).sqrt() < fighter.get_param_float("common", "hit_stop_delay_stick") {
        return;
    }
    let speed_up_mul = if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP) {
        fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG)
    } else {
        1.0
    };
    let asdi = sdi_mul * fighter.get_param_float("common", "hit_stop_delay_auto_mul") * speed_up_mul;
    let mut pos = *PostureModule::pos(fighter.module_accessor);
    pos.x += asdi * stick_x;
    pos.y += asdi * stick_y;
    PostureModule::set_pos(fighter.module_accessor, &pos);
}

// claude code: DI, drift DI and SDI/ASDI on knockback collision. a launch that hits a wall, ceiling or
// floor bounces (DAMAGE_FLY_REFLECT_*), and each bounce works like a fresh hit at full strength
// (SDI multiplier 1): held SDI during the bounce's freeze, then DI and ASDI as it lets go
// (bounce_collision). drift DI already covers the bounce statuses (drift_di). the freeze isn't hitlag:
// vanilla's start_damage_reflect_stop (lua2cpp_common) switches the knockback energy off for
// clamp(remaining hitstun x `damage_fly_reflect_stop_frame_rate`, `_min`, `_max`) frames
pub fn is_bounce_status(status_kind: i32) -> bool {
    [
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
    ].contains(&status_kind)
}

// claude code: DI on a bounce, the same as on a hit: the stick turns the rebound by up to common
// `damage_fly_correction_max` degrees, toward the stick by how far it's held across the rebound's path
unsafe fn bounce_di(fighter: &mut L2CFighterCommon) {
    let speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    let speed = (speed_x * speed_x + speed_y * speed_y).sqrt();
    let mut stick_x = fighter.left_stick_x();
    let mut stick_y = fighter.left_stick_y();
    let stick = (stick_x * stick_x + stick_y * stick_y).sqrt();
    if speed <= 0.0 || stick <= 0.0 {
        return;
    }
    if stick > 1.0 {
        stick_x /= stick;
        stick_y /= stick;
    }
    let across = (speed_x * stick_y - speed_y * stick_x) / speed;
    let angle = (fighter.get_param_float("common", "damage_fly_correction_max") * across).to_radians();
    let (sin, cos) = angle.sin_cos();
    fighter.set_speed(Vector2f::new(speed_x * cos - speed_y * sin, speed_x * sin + speed_y * cos), *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
}

// claude code: in a bounce's freeze: a bounce status whose knockback energy is still switched off
pub unsafe fn is_bounce_freeze(fighter: &mut L2CFighterCommon) -> bool {
    is_bounce_status(fighter.status())
    && !VarModule::is_flag(fighter.battle_object, vars::common::status::BOUNCE_RELEASED)
    && !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE)
}

// claude code: held SDI through the freeze, then DI and ASDI once on the first frame out of it. runs
// every frame from sys_line_system_control_fighter (opff.rs)
pub unsafe fn bounce_collision(fighter: &mut L2CFighterCommon) {
    if !is_bounce_status(fighter.status())
    || VarModule::is_flag(fighter.battle_object, vars::common::status::BOUNCE_RELEASED) {
        return;
    }
    if is_bounce_freeze(fighter) {
        if let Some((x, y)) = general_statuses::held_sdi(fighter, 1.0) {
            general_statuses::apply_sdi(fighter, x, y);
        }
        return;
    }
    VarModule::on_flag(fighter.battle_object, vars::common::status::BOUNCE_RELEASED);
    bounce_di(fighter);
    asdi(fighter, 1.0);
}

// claude code: every launch that lands while still in hitstun floor-bounces, not just spikes. vanilla's
// floor check (sub_AirChkDamageReflectFloor, lua2cpp_common 0x71000fb4d0, decompiled 2026-11-27) only
// bounces a fighter on the ground whose knockback energy alone is moving down faster than common
// `damage_fly_reflect_d_speed`, whose last bounce wasn't off the floor, and who has bounced fewer than
// `damage_fly_reflect_max` times. a normal launch falls on gravity, not knockback (its knockback energy
// is still going up or level), so only a spike ever passes. this hands the launch's real fall speed
// (knockback + gravity, from its last airborne frame) to the knockback energy first, so the landing
// looks like a spike to the check and vanilla bounces it the same way. gravity is zeroed with it (its
// share is in the knockback energy now). the "last bounce wasn't off the floor" rule is dropped too, so
// floor bounces can chain; `damage_fly_reflect_max` still caps the total. KNOCKED_DOWN launches have
// the floor-bounce term off, so they still land
unsafe fn launch_statuses() -> [i32; 6] {
    [
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
    ]
}

// claude code: every frame (sys_line, opff.rs): the launch's vertical speed while it's airborne
pub unsafe fn record_launch_speed(fighter: &mut L2CFighterCommon) {
    if !fighter.is_situation(*SITUATION_KIND_AIR)
    || !fighter.is_status_one_of(&launch_statuses()) {
        return;
    }
    let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE) + fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    VarModule::set_float(fighter.battle_object, vars::common::instance::LAUNCH_SPEED_Y, speed_y);
}

#[skyline::hook(replace = L2CFighterCommon_sub_AirChkDamageReflectFloor)]
unsafe fn sub_AirChkDamageReflectFloor_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_status_one_of(&launch_statuses())
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D) {
        let fall_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::LAUNCH_SPEED_Y);
        let speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        if fall_speed < speed_y {
            fighter.set_speed(Vector2f::new(speed_x, fall_speed), *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            // the knockback energy now carries the gravity's share of the fall too, so gravity starts
            // over from 0. left alone it kept pulling down after the bounce and ate the rebound
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        }
        // the last bounce being off the floor doesn't stop this one. put back if there's no bounce
        let last_dir = fighter.get_int(*FIGHTER_STATUS_WORK_ID_INT_DAMAGE_REFLECT_LAST_DIR);
        if last_dir == *FIGHTER_STATUS_DAMAGE_REFLECT_DIR_DOWN {
            fighter.set_int(*FIGHTER_STATUS_DAMAGE_REFLECT_DIR_NONE, *FIGHTER_STATUS_WORK_ID_INT_DAMAGE_REFLECT_LAST_DIR);
            let ret = call_original!(fighter);
            if !ret.get_bool() {
                fighter.set_int(last_dir, *FIGHTER_STATUS_WORK_ID_INT_DAMAGE_REFLECT_LAST_DIR);
            }
            return ret;
        }
    }
    call_original!(fighter)
}

// hdr: based on HDR's version (fighters/common/src/general_statuses/damage.rs)
#[skyline::hook(replace = L2CFighterCommon_status_DamageFly_Main)]
unsafe fn status_DamageFly_Main_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
        if fighter.motion_frame() >= (MotionModule::end_frame(fighter.module_accessor) - 1.0) && MotionModule::rate(fighter.module_accessor) != 0.0 {
            fighter.boma().set_rate(0.0);
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
        // claude code: held SDI (general_statuses/mod.rs) in place of vanilla's flick SDI. a bounce's
        // freeze has its own (bounce_sdi)
        if !is_paralyze && !is_bounce_status(fighter.status()) {
            let sdi_mul = hashmap["stop_delay_"].get_f32();
            if let Some((x, y)) = general_statuses::held_sdi(fighter, sdi_mul) {
                general_statuses::apply_sdi(fighter, x, y);
            }
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

// claude code: a FORCE_KNOCKDOWN hit marks its launch KNOCKED_DOWN, which lasts through the launch's
// wall and ceiling bounces (they run this again) and ends on any other new launch. a KNOCKED_DOWN
// launch never floor-bounces, so it lands in the getup-less down (down.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_DamageFlyCommon_init)]
pub unsafe fn damage_fly_common_init(fighter: &mut L2CFighterCommon) {
    let force_knockdown = VarModule::is_flag(fighter.battle_object, vars::common::instance::FORCE_KNOCKDOWN);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::FORCE_KNOCKDOWN);
    if force_knockdown {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::KNOCKED_DOWN);
    } else if !is_bounce_status(fighter.status()) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::KNOCKED_DOWN);
    }
    let knocked_down = VarModule::is_flag(fighter.battle_object, vars::common::instance::KNOCKED_DOWN);
    if knocked_down {
        fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D);
    }
    original!()(fighter);
    // claude code: and again once the original has run
    if knocked_down {
        fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D);
    }
}

pub unsafe extern "C" fn non_tumble_di(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR) {
        fighter.FighterStatusDamage__correctDamageVectorEffect(L2CValue::Bool(false));
    }
}

pub unsafe extern "C" fn drift_di(fighter: &mut L2CFighterCommon) {
    let damage_statuses = &[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE
    ];
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status_one_of(damage_statuses) 
    && !StopModule::is_stop(fighter.module_accessor) {
        let damage_speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let damage_speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let mut initial_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_X);
        let mut initial_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_Y);
        if initial_speed_x == 0.0 && initial_speed_y == 0.0 {
            VarModule::set_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_X, damage_speed_x);
            VarModule::set_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_Y, damage_speed_y);
            initial_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_X);
            initial_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_Y);
        }
        let drift_value = fighter.left_stick_x() * (0.0075 *  (1.0 - (initial_speed_x.abs() / 3.0).clamp(0.0, 1.0)));
        fighter.set_speed(Vector2f::new(damage_speed_x + drift_value, damage_speed_y), *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            FighterStatusUniqProcessDamage_leave_stop_hook,
            status_DamageFly_Main_hook,
            sub_AirChkDamageReflectFloor_hook,
            exec_damage_elec_hit_stop_hook,
            virtual_ftStatusUniqProcessDamage_exec_common,
            FighterStatusDamage__correctDamageVectorEffect,
            damage_fly_common_init
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    // hdr: HDR's patch (fighters/common/src/general_statuses/damage.rs)
    skyline::patching::Patch::in_text(0x63251c).nop();
}
