// status imports
use super::*;

// ============================================================
// WALK
// ============================================================

#[skyline::hook(replace = L2CFighterCommon_status_pre_Walk)]
unsafe fn status_pre_walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_brake = fighter.get_param_float("ground_brake", "");
	let mut initial_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);
	if ![*FIGHTER_STATUS_KIND_DASH].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}
	VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);
    call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_Walk_Common)]
unsafe fn status_walk_common(fighter: &mut L2CFighterCommon) {
    call_original!(fighter);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
}

#[skyline::hook(replace = L2CFighterCommon_status_Walk_Main_common)]
unsafe extern "C" fn status_walk_main_common(fighter: &mut L2CFighterCommon, arg1: L2CValue, arg2: L2CValue, arg3: L2CValue, arg4: L2CValue) -> L2CValue {
    let walk_accel_mul = fighter.get_param_float("walk_accel_mul", "");
    let walk_accel_add = fighter.get_param_float("walk_accel_add", "");
    let ground_brake = fighter.get_param_float("ground_brake", "");
    let walk_speed_max = fighter.get_param_float("walk_speed_max", "");
	let stick_x = fighter.global_table[STICK_X].get_f32();
	let prev_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);
	let mut lr_modifier = 1.0;
    if fighter.kind() != *FIGHTER_KIND_PICKEL {
        if [hash40("walk_slow_b"), hash40("walk_middle_b"), hash40("walk_fast_b")].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
            lr_modifier = -1.0;
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        if prev_speed * fighter.lr() * lr_modifier < 0.0 {
            let applied_speed = (stick_x.signum() * ((walk_accel_mul + (walk_accel_add * stick_x.abs())))) + prev_speed;
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed - speed_motion);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, applied_speed);
        } else if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            app::sv_kinetic_energy::unable(fighter.lua_state_agent);
        }
    }
    call_original!(fighter, arg1, arg2, arg3, arg4)
}

// ============================================================
// DASH
// ============================================================

pub fn wm_param_to_int(boma: *mut app::BattleObjectModuleAccessor, param_cat: u64, param: u64, what: i32) {
    unsafe {
        WorkModule::set_int(boma, WorkModule::get_param_int(boma, param_cat, param), what);
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_DashCommon)]
unsafe fn status_DashCommon(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    wm_param_to_int(fighter.module_accessor, hash40("common"), hash40("turn_dash_frame"), *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
    wm_param_to_int(fighter.module_accessor, hash40("common"), hash40("retry_turn_dash_frame"), *FIGHTER_STATUS_DASH_WORK_INT_RETRY_TURN_DASH_FRAME);
    wm_param_to_int(fighter.module_accessor, hash40("common"), hash40("dash_enable_attack_frame"), *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_RUN_BRAKE {
        let escape_frame = fighter.get_param_int("common", "run_brake_attack_escape_frame");
        let invalid_frame = escape_frame - fighter.global_table[0x25].get_i32();
        fighter.set_int(invalid_frame, *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME);
        if 0 < fighter.get_int(*FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME) {
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT);
        }
    }
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
}

macro_rules! interrupt {
    () => { return L2CValue::I32(1) };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}

macro_rules! interrupt_if {
    ($e:expr) => {{
        if $e { interrupt!() }
    }};
    ($fighter:ident, $status:expr, $repeat:expr, $e:expr) => {{
        if $e {
            interrupt!($fighter, $status, $repeat);
        }
    }}
}

macro_rules! ok {
    () => { return L2CValue::I32(0) };
}

macro_rules! ok_if {
    ($e:expr) => {{
        if $e { ok!() }
    }}
}

#[skyline::hook(replace = L2CFighterCommon_status_Dash_Main_common)]
unsafe extern "C" fn status_dash_main_common(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    let dash_stick_x: f32 = fighter.get_param_float("common", "dash_stick_x");
    let stick_x = fighter.global_table[STICK_X].get_f32();
    if fighter.global_table[DASH_CALLBACK].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[DASH_CALLBACK].get_ptr());
        interrupt_if!(callable(fighter).get_bool());
    }
    interrupt_if!(fighter, FIGHTER_STATUS_KIND_FALL, false, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    interrupt_if!(CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(false.into()).get_bool());
    interrupt_if!(fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool());
    let can_s4 = if fighter.is_flag(*FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.global_table[STICK_X].get_f32() * fighter.lr() < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x206138766c)
    } else {
        true
    };
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && can_s4 {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        app::sv_module_access::item(fighter.lua_state_agent);
        let should_throw = if fighter.pop_lua_stack(1).get_bool() {
            true
        } else {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
            app::sv_module_access::item(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool() {
                ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0
            } else {
                false
            }
        };
        interrupt_if!(fighter, FIGHTER_STATUS_KIND_ITEM_THROW, false, should_throw);
    }
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_THROW_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH) &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
            app::sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } &&
        fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    );
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_THROW,
        false,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE) &&
        ItemModule::is_have_item(fighter.module_accessor, 0) &&
        fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            app::sv_module_access::item(fighter.lua_state_agent);
            !fighter.pop_lua_stack(1).get_bool()
        }
    );
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_THROW_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH) &&
        ItemModule::is_have_item(fighter.module_accessor, 0) &&
        fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            app::sv_module_access::item(fighter.lua_state_agent);
            !fighter.pop_lua_stack(1).get_bool()
        }
    );
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_CATCH_TURN,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) &&
        fighter.global_table[STICK_X].get_f32() * fighter.lr() <= fighter.get_param_float("common", "turn_run_stick_x") &&
        fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0  &&
        !ItemModule::is_have_item(fighter.module_accessor, 0)
    );
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_CATCH_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) &&
        fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 &&
        !ItemModule::is_have_item(fighter.module_accessor, 0)
    );
    interrupt_if!(fighter.sub_transition_group_check_ground_guard().get_bool());
    if fighter.sub_transition_group_check_special_command().get_bool()
    || fighter.sub_transition_group_check_ground_special().get_bool()
    || fighter.sub_transition_specialflag_hoist().get_bool()
    {
        return L2CValue::I32(1);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4)
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        app::sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    }
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !fighter.is_flag(*FIGHTER_STATUS_DASH_FLAG_NO_S4)
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
        return L2CValue::I32(1);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4)
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        app::sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    }
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !fighter.is_flag(*FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        if ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
        }
        return L2CValue::I32(1);
    }
    interrupt_if!(
        fighter.global_table[0x57].get_bool() &&
        std::mem::transmute::<*const skyline::libc::c_void, extern "C" fn(&mut L2CFighterCommon) -> L2CValue>(
            fighter.global_table[0x57].get_ptr()
        )(fighter).get_bool()
    );
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START)
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !fighter.is_flag(*FIGHTER_STATUS_DASH_FLAG_NO_S4)
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
        return L2CValue::I32(1);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
        return L2CValue::I32(1);
    }
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_SWING_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH) &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
            app::sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } &&
        fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    );
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ATTACK_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) &&
        fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    );
    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return L2CValue::I32(1);
    }
    if fighter.get_int(*FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME) > 0
    && (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || app::FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false)) {
        if fighter.global_table[CMD_CAT1].get_i32() & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
        ) != 0 {
            if {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY);
                app::sv_module_access::item(fighter.lua_state_agent);
                fighter.pop_lua_stack(1).get_bool()
            }
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
                return L2CValue::I32(1);
            } else if ItemModule::get_pickable_item_size(fighter.module_accessor) == ITEM_SIZE_LIGHT
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH) &&
            {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
                app::sv_module_access::item(fighter.lua_state_agent);
                fighter.pop_lua_stack(1).get_bool()
            } {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
                return L2CValue::I32(1);
            }
        }
    }
    let is_dash_input: bool = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0;
    let is_backdash_input: bool = fighter.global_table[STICK_X].get_f32() * fighter.lr() <= -0.55
    && fighter.global_table[FLICK_X].get_i32() <= fighter.get_param_int("common", "dash_flick_x");
    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_AFTER_DASH_TO_RUN_FRAME)  // if after dash-to-run transition frame
    && fighter.get_param_float("common", "run_stick_x") <= fighter.global_table[STICK_X].get_f32() * fighter.lr()  // AND stick_x is >= run threshold
    && !is_dash_input {  // AND you haven't input another dash
        interrupt!(fighter, FIGHTER_STATUS_KIND_RUN, true);
    }
    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_DASH_TO_RUN_FRAME) {
        VarModule::off_flag(fighter.battle_object, vars::common::status::IS_DASH_TO_RUN_FRAME);
        VarModule::on_flag(fighter.battle_object, vars::common::status::IS_AFTER_DASH_TO_RUN_FRAME);
    }
    interrupt_if!(fighter.sub_transition_group_check_ground_jump().get_bool());
    // claude code: HDR moonwalk, reworked to work for the whole DASH. holding the stick down past -0.5
    // blocks the dashback so you can moonwalk instead of turning around
    let is_moonwalk = fighter.global_table[STICK_Y].get_f32() <= -0.5;
    // claude code: HDR perfect pivot, reworked to work for the whole DASH. HDR only allowed it in the first
    // 3 frames (and the vanilla turn-dash window). now any backdash input, on any dash frame, smash-turns
    // with the perfect pivot armed; letting go of the stick in TURN then pivots (see turn.rs)
    if !is_moonwalk && is_backdash_input {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
        interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH)
    && fighter.get_param_int("common", "re_dash_frame") as f32 <= fighter.motion_frame()  // if current frame is after redash frame
    && is_dash_input {
        interrupt!(fighter, FIGHTER_STATUS_KIND_DASH, true);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) {
        if fighter.get_param_float("common", "run_stick_x") <= fighter.global_table[STICK_X].get_f32() * fighter.lr() {
            if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN) {
                interrupt!(fighter, FIGHTER_STATUS_KIND_WALK, true);
            } else {
                interrupt!(fighter, FIGHTER_STATUS_KIND_RUN, true);
            }
        }
    }
    interrupt_if!(
        arg.get_bool() &&
        std::mem::transmute::<*const skyline::libc::c_void, extern "C" fn(&mut L2CFighterCommon) -> L2CValue>(
            arg.get_ptr()
        )(fighter).get_bool()
    );
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_WALK_BRAKE,
        false,
        GroundModule::get_down_friction(fighter.module_accessor) < 1.0 &&
        FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true)
    );
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_WAIT,
        false,
        MotionModule::is_end(fighter.module_accessor)
    );
    interrupt_if!(fighter.sub_ground_check_stop_wall().get_bool());
    if fighter.global_table[CURRENT_FRAME].get_i32() == 1
    && StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_TURN
    && StatusModule::prev_status_kind(fighter.module_accessor, 1) == *FIGHTER_STATUS_KIND_DASH
    && stick_x.abs() < dash_stick_x {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_LATE_PIVOT);
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
    }
    ok!()
}

#[skyline::hook(replace = L2CFighterCommon_sub_dash_uniq_process_main_internal)]
unsafe fn sub_dash_uniq_process_main_internal(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if !WorkModule::is_enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) {
        let stick_x = if fighter.global_table[0x2].get_i32() == *FIGHTER_KIND_DEMON {
            fighter.global_table[STICK_X].get_f32()
        } else {
            fighter.left_stick_x()
        };
        let walk_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x206138766c);
        let lr = fighter.lr();
        let is_backdash = if param_1.get_bool() { -1.0 } else { 1.0 };
        if stick_x * lr * is_backdash < walk_threshold {
            WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
            WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
            WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
    }
    let dash_count = fighter.get_int(*FIGHTER_STATUS_DASH_WORK_INT_COUNT);
    fighter.set_int(dash_count + 1, *FIGHTER_STATUS_DASH_WORK_INT_COUNT);
    let dash_count = fighter.get_int(*FIGHTER_STATUS_DASH_WORK_INT_COUNT);
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_TURN_DASH
    && fighter.is_flag(*FIGHTER_STATUS_DASH_FLAG_TURN_DASH_ON) {
        fighter.inc_int(*FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FROM_DASH_COUNT);
    }
    if !fighter.is_flag(*FIGHTER_STATUS_DASH_FLAG_NO_S4)
    && FighterControlModuleImpl::get_param_dash_s4_frame(fighter.module_accessor) as i32 <= dash_count {
        fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
        fighter.on_flag(*FIGHTER_STATUS_DASH_FLAG_NO_S4);
    }
    if fighter.is_flag(*FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE) {
        let dash_escape_frame = fighter.get_param_float("common", "dash_escape_frame");
        if dash_escape_frame <= dash_count as f32 {
            fighter.on_flag(*FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE);
        }
    }
    if !fighter.is_flag(*FIGHTER_STATUS_DASH_FLAG_NO_SLIP) {
        fighter.on_flag(*FIGHTER_STATUS_DASH_FLAG_NO_SLIP);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) {
        fighter.inc_int(*FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
    }
    let turn_dash_frame = fighter.get_int(*FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
    if 0 <= turn_dash_frame {
        fighter.dec_int(*FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
        if turn_dash_frame - 1 < 0 {
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
            fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        }
    }
    let retry_dash_frame = fighter.get_int(*FIGHTER_STATUS_DASH_WORK_INT_RETRY_TURN_DASH_FRAME);
    if 0 <= retry_dash_frame {
        fighter.dec_int(*FIGHTER_STATUS_DASH_WORK_INT_RETRY_TURN_DASH_FRAME);
        if retry_dash_frame - 1 < 0 {
            fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
            fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        }
    }
    let attack_frame = fighter.get_int(*FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
    if 0 <= attack_frame {
        fighter.dec_int(*FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
        if attack_frame - 1 < 0 {
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
            fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT);
            fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY);
        }
    }
    let invalid_attack_escape_frame = fighter.get_int(*FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME);
    if 0 < invalid_attack_escape_frame {
        fighter.dec_int(*FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME);
        if invalid_attack_escape_frame - 1 == 0 {
            let transitions = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT
            ];
            for val in transitions.iter() {
                fighter.enable_transition_term(*val);
            }
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Dash)]
unsafe fn status_end_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
	let mut initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	let run_speed_max = fighter.get_param_float("run_speed_max", "");
	if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_WALK].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        let applied_speed_clamped = initial_speed.clamp(-run_speed_max, run_speed_max);

		initial_speed = applied_speed_clamped;
	}
    if StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_STATUS_KIND_TURN {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_LATE_PIVOT) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT) {
                VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
                let dash_speed: f32 = fighter.get_param_float("dash_speed", "");
                let speed_mul = -1.0;
                let pivot_boost = dash_speed * speed_mul * fighter.lr() * -1.0;
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, pivot_boost);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            }
        }
    } else {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
    }
    VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);
    call_original!(fighter)
}

// FIGHTER_STATUS_KIND_TURN_DASH

#[skyline::hook(replace = L2CFighterCommon_status_pre_TurnDash)]
unsafe fn status_pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    fighter.set_status_kind_interrupt(*FIGHTER_STATUS_KIND_TURN);
    return 1.into()
}

// ============================================================
// RUN
// ============================================================

#[skyline::hook(replace = L2CFighterCommon_status_pre_Run)]
unsafe fn status_pre_run(fighter: &mut L2CFighterCommon) -> L2CValue {
	let mut initial_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);
	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}
	VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);
    call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_Run_Sub)]
unsafe fn status_run_sub(fighter: &mut L2CFighterCommon) {
    let value: f32 = if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_DASH || fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_TURN {
        fighter.get_float(*FIGHTER_STATUS_RUN_WORK_FLOAT_START_FRAME)
    } else {
        0.0
    };
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("run"), value, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
    if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA) {
        fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_GEKIKARA_RUN_BRAKE);
    }
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);
}

#[skyline::hook(replace = L2CFighterCommon_status_Run_Main)]
unsafe extern "C" fn status_run_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let run_accel_mul = fighter.get_param_float("run_accel_add", "");
    let run_accel_add = fighter.get_param_float("run_accel_mul", "");
    let run_speed_max = fighter.get_param_float("run_speed_max", "");
	let stick_x = fighter.global_table[STICK_X].get_f32();
	let prev_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	let mut speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	if prev_speed * fighter.lr() < 0.0 {
		let added_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
		let applied_speed = added_speed - speed_motion;
		let applied_speed_clamped = applied_speed.clamp(-run_speed_max, run_speed_max);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, added_speed);
	} else if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	} else {}
    let ret = call_original!(fighter);
    if ret.get_i32() != 0 {
        if (
            WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
            && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
        ) || (
            WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW)
            && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0
        ) || (
            WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (
                fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0
                || fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0
            )
        ) && {
            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be));
            app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            interrupt!(fighter, *FIGHTER_STATUS_KIND_APPEAL, false);
        }
    }
    ret
}

#[skyline::hook(replace = L2CFighterCommon_status_RunBrake_Main)]
unsafe fn status_runbrake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
    ) || (
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0
    ) || (
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (
            fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0
            || fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0
        )
    ) && {
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be));
        app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } {
        interrupt!(fighter, *FIGHTER_STATUS_KIND_APPEAL, false);
    }
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    if fighter.sub_transition_group_check_ground_guard().get_bool() {
        return true.into();
    }
	call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_TurnRunBrake_Main)]
unsafe fn status_turnrunbrake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    if fighter.sub_transition_group_check_ground_guard().get_bool() {
        return true.into();
    }
    call_original!(fighter)
}

// ============================================================
// TURN
// ============================================================

#[skyline::hook(replace = L2CFighterCommon_status_pre_TurnCommon)]
unsafe extern "C" fn status_pre_turncommon(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    fighter.on_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_TURN_ATTACK_S4_REV_PAD);
    let frame = if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_LATE_PIVOT) {
        5.0
    } else {
        0.0
    };
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("turn"), frame, 1.0, false, 0.0, false, false);
}

#[skyline::hook(replace = L2CFighterCommon_status_Turn_Main)]
unsafe extern "C" fn status_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let should_end = if fighter.global_table[0x35].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[0x35].get_ptr() as _;
        if !custom_routine.is_null() {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine); callable(fighter).get_bool()} else {false}
    } else { false };
    if !should_end {
        let dash_stick_x: f32 = fighter.get_param_float("common", "dash_stick_x");
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let turn_work_lr: f32 = fighter.get_float(*FIGHTER_STATUS_TURN_WORK_FLOAT_LR);
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && stick_x * -1.0 * turn_work_lr < dash_stick_x
        && VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN)
        && StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_DASH
        && fighter.motion_frame() == 1.0
        && VarModule::is_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT) {
            VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
            let dash_speed: f32 = fighter.get_param_float("dash_speed", "");
            let speed_mul = -1.0;
            let pivot_boost = dash_speed * speed_mul * fighter.lr();
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, pivot_boost);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }
        if !fighter.status_TurnCommon().get_bool() {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.motion_frame() >= 1.0 {
                if (
                    VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN)
                    && fighter.motion_frame() == 1.0 
                    && stick_x * -1.0 * turn_work_lr >= dash_stick_x
                ) || fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
                    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
                    interrupt!(fighter, FIGHTER_STATUS_KIND_DASH, true);
                }
                let lr = fighter.get_float(*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
                    let next_status = if [*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_DEMON].contains(&fighter.kind()) && lr != 0.0 {FIGHTER_RYU_STATUS_KIND_DASH_BACK} else {FIGHTER_STATUS_KIND_TURN_DASH};
                    interrupt!(fighter, next_status, true);
                }
            }
            return L2CValue::I32(0);
        }
    }
    L2CValue::I32(1)
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Turn)]
unsafe fn status_end_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_LATE_PIVOT);
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_DASH {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
    }
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_TURN_DASH {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    }
    fighter.sub_exit_Turn();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_walk_common,
            status_walk_main_common,
            status_pre_walk,
            status_DashCommon,
            sub_dash_uniq_process_main_internal,
            status_end_dash,
            status_dash_main_common,
            status_pre_turndash,
            status_run_sub,
            status_run_main,
            status_pre_run,
            status_runbrake_main,
            status_turnrunbrake_main,
            status_pre_turncommon,
            status_turn_main,
            status_end_turn
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
