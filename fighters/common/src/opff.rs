use super::*;

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

// claude code: dash landing, the wavedash-less waveland. in the air and free to act, a dash input
// (stick past dash_stick_x within dash_flick_x frames) while a soft platform crosses the ECB, even
// rising, snaps the fighter onto the platform and starts DASH toward the stick
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

// claude code: a slid-off move's scripts read the situation from the global table too, so it says
// ground there as well (see function_hooks/lua_bind_hook.rs, is_slide_off_air)
unsafe fn slide_off_ground_globals(fighter: &mut L2CFighterCommon) {
    if function_hooks::lua_bind_hook::is_slide_off_air(fighter.boma()) {
        fighter.global_table[globals::SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
        fighter.global_table[globals::PREV_SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
    }
}

// claude code: runs a slid-off move in the air (function_hooks/lua_bind_hook.rs). attacks end in a fall
// once they can be cancelled, throws play out. touching down, attacks land and throws carry on grounded
pub unsafe extern "C" fn slide_off_air(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status(*FIGHTER_STATUS_KIND_THROWN) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::SLIDE_OFF_THROWN);
    }
    if !VarModule::is_flag(fighter.battle_object, vars::common::instance::SLIDE_OFF_AIR) {
        return;
    }
    let status_kind = fighter.status();
    if !function_hooks::lua_bind_hook::is_slide_off_kind(status_kind) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::SLIDE_OFF_AIR);
        return;
    }
    let is_throw = status_kind == *FIGHTER_STATUS_KIND_THROW;
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::SLIDE_OFF_AIR);
        if is_throw {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        } else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
        }
        return;
    }
    if !is_throw
    && (MotionModule::is_end(fighter.module_accessor) || CancelModule::is_enable_cancel(fighter.module_accessor)) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        return;
    }
    function_hooks::lua_bind_hook::keep_slide_off_air(fighter.boma());
}

// claude code: counts down the parry window (general_statuses/parry.rs) and closes it at 0, or
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
        general_statuses::parry::close_parry_window(fighter.boma());
    } else {
        VarModule::dec_int(fighter.battle_object, vars::common::instance::PARRY_WINDOW_FRAME);
    }
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

// claude code: a wall jump gives back every aerial special, like landing would. clears the
// no-special-fall flags (function_hooks/lua_bind_hook.rs) and the game's own once-per-airtime
// flags and counters. those IDs are per fighter (echoes share their base fighter's), so each list
// only runs for its fighters. the vanilla list is picked by name, see wall_jump_vanilla_specials
pub unsafe extern "C" fn wall_jump_restore_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status(*FIGHTER_STATUS_KIND_WALL_JUMP)
    || !StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    for flag in function_hooks::lua_bind_hook::special_used_flags() {
        VarModule::off_flag(fighter.battle_object, flag);
    }
    let (flags, ints) = wall_jump_vanilla_specials(fighter.global_table[globals::FIGHTER_KIND].get_i32());
    for flag in flags {
        WorkModule::off_flag(fighter.module_accessor, flag);
    }
    for int in ints {
        WorkModule::set_int(fighter.module_accessor, 0, int);
    }
}

// claude code: per fighter kind, the game's flags to turn off and counters to zero so its
// aerial specials work again (side-B hops, "disable air special" locks, use counts)
unsafe fn wall_jump_vanilla_specials(kind: i32) -> (Vec<i32>, Vec<i32>) {
    if kind == *FIGHTER_KIND_MARIO {
        (vec![*FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP], vec![])
    } else if kind == *FIGHTER_KIND_MARIOD {
        (vec![*FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY], vec![])
    } else if kind == *FIGHTER_KIND_LUIGI {
        (vec![*FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY], vec![])
    } else if kind == *FIGHTER_KIND_YOSHI {
        (vec![*FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP], vec![*FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_SPECIAL_S_JUMP_COUNT])
    } else if kind == *FIGHTER_KIND_PURIN {
        (vec![*FIGHTER_PURIN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP], vec![])
    } else if kind == *FIGHTER_KIND_POPO || kind == *FIGHTER_KIND_NANA {
        (vec![*FIGHTER_POPO_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_S_FALL], vec![])
    } else if kind == *FIGHTER_KIND_SHEIK {
        (vec![*FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW], vec![])
    } else if kind == *FIGHTER_KIND_MARTH || kind == *FIGHTER_KIND_LUCINA {
        (vec![*FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP], vec![])
    } else if kind == *FIGHTER_KIND_ROY || kind == *FIGHTER_KIND_CHROM {
        (vec![*FIGHTER_ROY_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP], vec![])
    } else if kind == *FIGHTER_KIND_MEWTWO {
        (vec![*FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BUOYANCY], vec![])
    } else if kind == *FIGHTER_KIND_METAKNIGHT {
        (vec![*FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP], vec![])
    } else if kind == *FIGHTER_KIND_PFUSHIGISOU {
        (vec![*FIGHTER_PFUSHIGISOU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_HOP], vec![])
    } else if kind == *FIGHTER_KIND_PLIZARDON {
        (vec![*FIGHTER_PLIZARDON_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HOP], vec![])
    } else if kind == *FIGHTER_KIND_ROCKMAN {
        (vec![*FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL], vec![])
    } else if kind == *FIGHTER_KIND_LITTLEMAC {
        (vec![*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP], vec![])
    } else if kind == *FIGHTER_KIND_GEKKOUGA {
        (vec![*FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_IS_DISABLE, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_DISABLE, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP], vec![])
    } else if kind == *FIGHTER_KIND_PALUTENA {
        (vec![*FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP], vec![])
    } else if kind == *FIGHTER_KIND_PACMAN {
        (vec![*FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FALL], vec![*FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_HI_JUMP_NUM])
    } else if kind == *FIGHTER_KIND_KOOPAJR {
        (vec![*FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE], vec![])
    } else if kind == *FIGHTER_KIND_RYU || kind == *FIGHTER_KIND_KEN {
        (vec![*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N_HOP], vec![])
    } else if kind == *FIGHTER_KIND_CLOUD {
        (vec![*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N_HOP, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_S_HOP], vec![])
    } else if kind == *FIGHTER_KIND_KAMUI {
        (vec![*FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SPECIAL_N_BUOYANCY], vec![])
    } else if kind == *FIGHTER_KIND_BAYONETTA {
        (vec![*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S], vec![*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT])
    } else if kind == *FIGHTER_KIND_INKLING {
        (vec![*FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S], vec![])
    } else if kind == *FIGHTER_KIND_GAOGAEN {
        (vec![*FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_INVALID_SPECIAL_AIR_S], vec![])
    } else if kind == *FIGHTER_KIND_JACK {
        (vec![*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_HOP], vec![])
    } else if kind == *FIGHTER_KIND_BRAVE {
        (vec![*FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE], vec![])
    } else if kind == *FIGHTER_KIND_BUDDY {
        (vec![], vec![*FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_LW_HOP_COUNT])
    } else if kind == *FIGHTER_KIND_DOLLY {
        (vec![*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE], vec![])
    } else if kind == *FIGHTER_KIND_MASTER {
        (vec![*FIGHTER_MASTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE], vec![])
    } else if kind == *FIGHTER_KIND_PICKEL {
        (vec![], vec![*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_SPECIAL_S_JUMP_COUNT_ON_AIR])
    } else if kind == *FIGHTER_KIND_DEMON {
        (vec![*FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S], vec![])
    } else if kind == *FIGHTER_KIND_TRAIL {
        (vec![*FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N1_HOP, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N3_HOP], vec![])
    } else if kind == *FIGHTER_KIND_MIISWORDSMAN {
        (vec![*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_REVERSE_SLASH_HOP], vec![])
    } else if kind == *FIGHTER_KIND_MIIGUNNER {
        (vec![*FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_FLAG_GRAVITY_BRAKE_USED], vec![])
    } else {
        (vec![], vec![])
    }
}

// claude code: per-frame side of the universal no special fall (function_hooks/lua_bind_hook.rs).
// Backs up the hooks (remembers specials, turns any leftover FALL_SPECIAL into FALL) and gives
// specials back on landing, getting hit or dying
pub unsafe extern "C" fn no_special_fall(fighter: &mut L2CFighterCommon) {
    use function_hooks::lua_bind_hook::*;
    let status_kind = fighter.status();

    if special_used_flag(status_kind).is_some() {
        VarModule::set_int(fighter.battle_object, vars::common::instance::LAST_SPECIAL_KIND, status_kind);
    }

    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        lock_last_special(fighter.boma());
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        return;
    }

    let is_hit_or_dead = fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH
    ]);
    let is_landed = !fighter.is_situation(*SITUATION_KIND_AIR);
    if is_landed || is_hit_or_dead {
        for flag in special_used_flags() {
            VarModule::off_flag(fighter.battle_object, flag);
        }
    }
    if is_hit_or_dead
    || (fighter.is_situation(*SITUATION_KIND_GROUND) && status_kind < *FIGHTER_STATUS_KIND_SPECIAL_N) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::LAST_SPECIAL_KIND, 0);
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

// claude code: wall jumps out of the states that come right before special fall, and more. Grant's rule
// (2026-11-19): any air state, specials included, can wall jump whenever it has no hitbox out (a
// special's end, like PK Thunder 2's window between its attack and special fall, or an aerial's
// endlag). it runs the same wall jump test falling does (general_statuses::try_wall_jump: one wall jump per airtime,
// facing either way), and the wall jump itself gives the specials back like any other
// (wall_jump_restore_specials).
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
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL)
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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sys_line_system_control_fighter)]
pub unsafe fn sys_line_system_control_fighter_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::common::instance::STATUS_REQUESTS_THIS_FRAME, 0);
    let ret = original!()(fighter);
    left_stick_flick_counter(fighter);
    right_stick_flick_counter(fighter);
    general_statuses::parry::parry_reward(fighter);
    slide_off_ground_globals(fighter);
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
        .on_line(Main, slide_off_air)
        .on_line(Main, parry_window)
        .on_line(Main, pass_statuses)
        .on_line(Main, wall_jump_count_reset)
        .on_line(Main, wall_jump_anytime)
        .on_line(Main, wall_jump_restore_specials)
        .on_line(Main, no_special_fall)
        .on_line(Main, non_tumble_di)
        .on_line(Main, drift_di)
        .on_line(Main, attack_air_fastfall_hitfall)
        .install();
}