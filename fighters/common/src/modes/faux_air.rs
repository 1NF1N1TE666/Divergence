use super::*;

// claude code: faux air, old Little Mac's air (2026-11-19) for everyone. ground statuses always see the
// ground (StatusModule::situation_kind, function_hooks/mod.rs), so every ground option works off it too:
// jabs, tilts, smashes, grabs and shield in midair, and no aerials. off the ground, they get the
// fighter's real gravity and fall speed (AIRBORNE + air_physics). hitstun, grabs, every special and
// every jump run in the real air (is_real_air_status), so specials keep their own air versions and jumps
// are the game's own: jumpsquat, short and full hops, tap jump, multijumps, delayed double jumps, all
// from the fighter's own params, animations and scripts (so slotted movesets and Dionysus clones keep
// theirs). the faux air takes back over the moment they do something else out of a jump (request,
// leave_jump_to_shield) or the jump ends (its fall becomes WAIT). Mac's own faux jumps (2026-11-19)
// worked out a speed from the jump height instead; that's gone. so are his side-B floor and his air
// side-B and down-B locks, which were his own
unsafe fn is_on(boma: &mut BattleObjectModuleAccessor) -> bool {
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

// claude code: true when a request for `status_kind` is one request() turns into WAIT. the status hooks
// (lua_bind_hook.rs) then leave the input buffer alone, so the button that asked for an aerial is still
// there next frame, and WAIT's own checks turn it into the ground version (a jab, tilt...) in the faux air
pub unsafe fn keeps_buffer(boma: &mut BattleObjectModuleAccessor, status_kind: i32) -> bool {
    is_on(boma) && is_air_only_status(status_kind)
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
    is_on(boma)
    && VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_GROUND_STATUS)
    && !VarModule::is_flag(boma.object(), vars::common::instance::FAUX_AIR_REAL_SITUATION)
}

// claude code: true while a ground status is off the ground
pub unsafe fn is_airborne(boma: &mut BattleObjectModuleAccessor) -> bool {
    is_on(boma)
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
pub unsafe fn init_settings(boma: &mut BattleObjectModuleAccessor, situation: &mut SituationKind, ground_correct_kind: &mut u32) {
    if !is_on(boma) {
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
pub unsafe fn request(boma: &mut BattleObjectModuleAccessor, status_kind: i32) -> Option<i32> {
    if !is_on(boma) {
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
pub unsafe fn frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if !is_on(boma) {
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
