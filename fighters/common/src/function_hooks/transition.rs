use super::*;

#[skyline::hook(replace = WorkModule::is_enable_transition_term)]
unsafe fn is_enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if boma.is_fighter()
    && [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ATTACK,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_SPEICAL,
        // claude code: dodges are removed, so their inputs are switched off here, before any status
        // acts on them, for everyone (CPUs included). refusing the request alone (removed_dodge_request,
        // lua_bind_hook.rs) wasn't enough: the status that asked has already been told it changed and
        // skips the rest of its frame. a CPU spamming airdodge after a jump skipped every frame of the
        // jump's own code and hung in midair, frozen but drifting (2026-11-19, Kazuya; likely the DJC
        // floats too)
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR,
        // claude code: teching is removed: every tech (floor, roll, wall, walltech jump, ceiling) is
        // switched off here, so a launch into a surface always bounces or lands. the defender's
        // answer is DI, drift DI and SDI on the bounce instead (general_statuses/damage.rs)
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
    ].contains(&flag) {
        return false;
    }
    // claude code: a broken shield is gone until respawn (general_statuses/shield.rs)
    if boma.is_fighter()
    && [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD].contains(&flag)
    && general_statuses::shield::is_shield_broken(boma) {
        return false;
    }
    // claude code: no getups from a forced knockdown (general_statuses/down.rs)
    if general_statuses::down::is_getup_locked(boma, flag) {
        return false;
    }
    // claude code: specials locked this airtime (universal no special fall)
    if boma.is_fighter() && lua_bind_hook::is_special_term_locked(boma, flag) {
        return false;
    }
    // claude code: a special a wall jump gave back (general_statuses/walljump.rs)
    if general_statuses::walljump::is_term_forced(boma, flag) {
        return true;
    }
    original!()(boma, flag)
}

// hdr: HDR's version (fighters/common/src/function_hooks/transition.rs)
#[skyline::hook(replace=WorkModule::enable_transition_term)]
unsafe fn enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN {
        VarModule::on_flag(boma.object(), vars::common::status::IS_DASH_TO_RUN_FRAME);
    }
    original!()(boma, flag)
}

pub fn install() {
    skyline::install_hooks!(
        is_enable_transition_term_hook,
        enable_transition_term_hook
    );
}
