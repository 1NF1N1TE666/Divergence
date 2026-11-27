use super::*;

// claude code: no getup mixups. a fighter about to be knocked down stands in plain WAIT instead, so
// landing out of a launch is a neutral reset. the one exception is a FORCE_KNOCKDOWN launch
// (KNOCKED_DOWN, damage.rs): that fighter lies down for real, with every getup option (stand, roll,
// getup attack) locked, and stands in plain WAIT after common `down_frame` frames of lying

// claude code: the lying statuses a KNOCKED_DOWN fighter can be in
unsafe fn down_statuses() -> [i32; 6] {
    [
        *FIGHTER_STATUS_KIND_DOWN,
        *FIGHTER_STATUS_KIND_DOWN_WAIT,
        *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
        *FIGHTER_STATUS_KIND_DOWN_CONTINUE,
        *FIGHTER_STATUS_KIND_DOWN_SPOT,
        *FIGHTER_STATUS_KIND_DOWN_REFLECT_LR
    ]
}

// claude code: the launch statuses a KNOCKED_DOWN fighter is in on the way down
unsafe fn launch_statuses() -> [i32; 8] {
    [
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY
    ]
}

unsafe fn is_knocked_down(boma: &mut BattleObjectModuleAccessor) -> bool {
    VarModule::is_flag(boma.object(), vars::common::instance::KNOCKED_DOWN)
}

// claude code: getup inputs are off while KNOCKED_DOWN (transition.rs)
pub unsafe fn is_getup_locked(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    boma.is_fighter()
    && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_STAND
    && is_knocked_down(boma)
}

// claude code: every frame (sys_line, opff.rs): counts a KNOCKED_DOWN fighter's lying frames and stands
// them in WAIT at `down_frame`. KNOCKED_DOWN ends on anything but the launch and the lying
pub unsafe fn forced_down(fighter: &mut L2CFighterCommon) {
    if !is_knocked_down(fighter.boma()) {
        return;
    }
    let status = fighter.status();
    if launch_statuses().contains(&status) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::DOWN_FRAME, 0);
        return;
    }
    if !down_statuses().contains(&status) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::KNOCKED_DOWN);
        return;
    }
    VarModule::inc_int(fighter.battle_object, vars::common::instance::DOWN_FRAME);
    let frame = VarModule::get_int(fighter.battle_object, vars::common::instance::DOWN_FRAME);
    if frame as f32 >= fighter.get_param_float("common", "down_frame") {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::KNOCKED_DOWN);
        fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
    }
}

// claude code: a getup that starts anyway (vanilla's own auto getup, or anything the locked input
// misses) while KNOCKED_DOWN becomes plain WAIT too
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DownStand)]
unsafe fn status_pre_DownStand(fighter: &mut L2CFighterCommon) -> L2CValue {
    if is_knocked_down(fighter.boma()) {
        return getup_to_wait(fighter);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DownStandFb)]
unsafe fn status_pre_DownStandFb(fighter: &mut L2CFighterCommon) -> L2CValue {
    if is_knocked_down(fighter.boma()) {
        return getup_to_wait(fighter);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DownStandAttack)]
unsafe fn status_pre_DownStandAttack(fighter: &mut L2CFighterCommon) -> L2CValue {
    if is_knocked_down(fighter.boma()) {
        return getup_to_wait(fighter);
    }
    call_original!(fighter)
}

unsafe fn getup_to_wait(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::KNOCKED_DOWN);
    fighter.boma().set_status_kind_interrupt(*FIGHTER_STATUS_KIND_WAIT);
    1.into()
}

// hdr: this and the four hooks after it are HDR's (fighters/common/src/general_statuses/down.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Down)]
unsafe fn status_pre_Down(fighter: &mut L2CFighterCommon) -> L2CValue {
    // claude code: no knockdown unless KNOCKED_DOWN: plain WAIT instead
    if !is_knocked_down(fighter.boma()) {
        fighter.boma().set_status_kind_interrupt(*FIGHTER_STATUS_KIND_WAIT);
        return 1.into();
    }
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
        true,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_SLOPE_TOP_UNLIMIT as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_DamageFlyChkUniq)]
unsafe fn sub_DamageFlyChkUniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = call_original!(fighter);
    if ret.get_bool() {
        if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP) {
            fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
            fighter.set_float(0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
        }
    }
    ret
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_DamageFlyChkUniq,
            status_pre_Down,
            status_pre_DownStand,
            status_pre_DownStandFb,
            status_pre_DownStandAttack
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
