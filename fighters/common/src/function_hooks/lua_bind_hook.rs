use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

unsafe fn is_edge_slip_status(boma: &mut BattleObjectModuleAccessor) -> bool {
    boma.is_fighter() && boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_F,
        *FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
        *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
        *FIGHTER_STATUS_KIND_APPEAL
    ])
}

// claude code: slippery ground moves. the "stop at the edge" ground corrections become plain ground
// correction, so every grounded move can slide off edges
unsafe fn slide_off_correct_kind(kind: i32) -> i32 {
    if is_edge_stop_kind(kind) {
        *GROUND_CORRECT_KIND_GROUND
    } else {
        kind
    }
}

unsafe fn is_edge_stop_kind(kind: i32) -> bool {
    kind == *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP || kind == *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK
}

// claude code: remembers, per status, that it asked to stop at edges (so it can slide off), and forgets
// it when the status switches to air correction itself (a move that really takes off). see
// opff.rs slide_off_own_status, which gives such a status air physics once it's off the edge
unsafe fn note_edge_stop(boma: &mut BattleObjectModuleAccessor, kind: i32) {
    if !boma.is_fighter() || !VarModule::has_var_module(boma.object()) {
        return;
    }
    if is_edge_stop_kind(kind) {
        VarModule::on_flag(boma.object(), vars::common::status::EDGE_STOP_SLIPPED);
    } else if kind == *GROUND_CORRECT_KIND_AIR
    && !VarModule::is_flag(boma.object(), vars::common::status::SLID_OFF_OWN_STATUS) {
        VarModule::off_flag(boma.object(), vars::common::status::EDGE_STOP_SLIPPED);
    }
}

// claude code: the common ground attacks and throws that keep going after sliding off an edge (grabs
// themselves slide off but just fall)
pub fn is_slide_off_kind(status_kind: i32) -> bool {
    [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_100,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_THROW
    ].contains(&status_kind)
}

// claude code: true while a slid-off move runs in the air. its scripts are told they're still on the
// ground (StatusModule::situation_kind, function_hooks/mod.rs), so a throw keeps hold of the opponent and
// plays out, while the fighter really falls
pub unsafe fn is_slide_off_air(boma: &mut BattleObjectModuleAccessor) -> bool {
    if !boma.is_fighter() {
        return false;
    }
    let status_kind = boma.status();
    (VarModule::is_flag(boma.object(), vars::common::instance::SLIDE_OFF_AIR) && is_slide_off_kind(status_kind))
    || (VarModule::is_flag(boma.object(), vars::common::instance::SLIDE_OFF_THROWN) && status_kind == *FIGHTER_STATUS_KIND_THROWN)
}

// claude code: keeps a throw's opponent attached off an edge. the throw takes the air-throw flag HDR's
// Kirby throws use, and the opponent is told they're still grounded too, like the thrower
unsafe fn hold_thrown_opponent(boma: &mut BattleObjectModuleAccessor) {
    if !boma.is_status(*FIGHTER_STATUS_KIND_THROW) {
        return;
    }
    boma.on_flag(*FIGHTER_STATUS_THROW_FLAG_START_AIR);
    let opponent_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
    if opponent_id == *BATTLE_OBJECT_ID_INVALID as u32 {
        return;
    }
    let opponent = &mut *sv_battle_object::module_accessor(opponent_id);
    if opponent.is_fighter() {
        VarModule::on_flag(opponent.object(), vars::common::instance::SLIDE_OFF_THROWN);
    }
}

unsafe fn slide_off_air_physics(boma: &mut BattleObjectModuleAccessor, speed_y: f32) {
    let air_speed_y_stable = boma.get_param_float("air_speed_y_stable", "");
    air_physics(boma, speed_y, air_speed_y_stable);
}

// claude code: a move that just slid off starts falling from a standstill
unsafe fn start_slide_off_air(boma: &mut BattleObjectModuleAccessor) {
    VarModule::on_flag(boma.object(), vars::common::instance::SLIDE_OFF_AIR);
    VarModule::set_float(boma.object(), vars::common::instance::SLIDE_OFF_SPEED_Y, 0.0);
    slide_off_air_physics(boma, 0.0);
    hold_thrown_opponent(boma);
}

// claude code: keeps a slid-off move falling when it moves on to its next status (like a jab
// restarting), which resets ground correction and gravity, caps the fall at the fighter's fall
// speed, and remembers it for the next status
pub unsafe fn keep_slide_off_air(boma: &mut BattleObjectModuleAccessor) {
    hold_thrown_opponent(boma);
    if GroundModule::get_correct(boma) != *GROUND_CORRECT_KIND_AIR
    || !KineticModule::is_enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
        let speed_y = VarModule::get_float(boma.object(), vars::common::instance::SLIDE_OFF_SPEED_Y);
        slide_off_air_physics(boma, speed_y);
    }
    let gravity = boma.get_gravity_energy() as *mut smash::app::FighterKineticEnergyGravity;
    let air_speed_y_stable = boma.get_param_float("air_speed_y_stable", "");
    let speed_y = lua_bind::KineticEnergy::get_speed_y(gravity as *mut smash::app::KineticEnergy);
    if speed_y < -air_speed_y_stable {
        lua_bind::FighterKineticEnergyGravity::set_speed(gravity, -air_speed_y_stable);
    }
    VarModule::set_float(boma.object(), vars::common::instance::SLIDE_OFF_SPEED_Y, speed_y.max(-air_speed_y_stable));
}

// claude code: status requests around sliding off. a move that just slid off asks for FALL: ground
// moves keep going (start_slide_off_air), specials restart as their aerial version, anything else falls.
// while one runs, its own next statuses go through, standing/moving/landing ones become FALL, and
// anything else (getting hit, a special) just ends it. Err = no say
unsafe fn slide_off_request(boma: &mut BattleObjectModuleAccessor, status_kind: i32) -> Result<Option<i32>, ()> {
    if VarModule::is_flag(boma.object(), vars::common::instance::SLIDE_OFF_AIR) {
        if is_slide_off_kind(status_kind) {
            return Err(());
        }
        VarModule::off_flag(boma.object(), vars::common::instance::SLIDE_OFF_AIR);
        if [
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_WALK,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_RUN,
            *FIGHTER_STATUS_KIND_RUN_BRAKE,
            *FIGHTER_STATUS_KIND_TURN,
            *FIGHTER_STATUS_KIND_TURN_DASH,
            *FIGHTER_STATUS_KIND_TURN_RUN,
            *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE,
            *FIGHTER_STATUS_KIND_SQUAT,
            *FIGHTER_STATUS_KIND_SQUAT_WAIT,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_SQUAT_F,
            *FIGHTER_STATUS_KIND_SQUAT_B,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_JUMP_SQUAT,
            *FIGHTER_STATUS_KIND_GUARD_ON,
            *FIGHTER_STATUS_KIND_GUARD,
            *FIGHTER_STATUS_KIND_CATCH_CUT
        ].contains(&status_kind) {
            return Ok(Some(*FIGHTER_STATUS_KIND_FALL));
        }
        return Err(());
    }
    if status_kind != *FIGHTER_STATUS_KIND_FALL {
        return Err(());
    }
    if !boma.is_situation(*SITUATION_KIND_AIR) {
        return Err(());
    }
    if is_slide_off_kind(boma.status()) && !MotionModule::is_end(boma) {
        start_slide_off_air(boma);
        return Ok(None);
    }
    if !StatusModule::is_situation_changed(boma) {
        return Err(());
    }
    if VarModule::is_flag(boma.object(), vars::common::instance::IN_SPECIAL)
    && boma.status() >= *FIGHTER_STATUS_KIND_SPECIAL_N {
        let last = VarModule::get_int(boma.object(), vars::common::instance::LAST_SPECIAL_KIND);
        if special_used_flag(last).is_some() {
            return Ok(filter_status_request(boma, last));
        }
    }
    Err(())
}

#[skyline::hook(replace=GroundModule::correct)]
unsafe fn correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
    if modes::faux_air::is_airborne(boma) {
        return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if is_edge_slip_status(boma) {
        return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if boma.is_fighter() {
        note_edge_stop(boma, kind.0);
        return original!()(boma, GroundCorrectKind(slide_off_correct_kind(kind.0)));
    }
    original!()(boma, kind)
}

// claude code: statuses that set their ground correction mid-status (grabs among them) get the same
// slippery correction as GroundModule::correct
#[skyline::hook(replace=GroundModule::set_correct)]
unsafe fn set_correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) {
    if modes::faux_air::is_airborne(boma) {
        return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if boma.is_fighter() {
        note_edge_stop(boma, kind.0);
        return original!()(boma, GroundCorrectKind(slide_off_correct_kind(kind.0)));
    }
    original!()(boma, kind)
}

#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, mut situation: smash::app::SituationKind, mut kinetic_type: i32, mut ground_correct_kind: u32, ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool, keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let asked_correct = ground_correct_kind as i32;
    if is_edge_slip_status(boma)
    || (boma.is_fighter() && boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_status(*FIGHTER_STATUS_KIND_TURN)) {
        ground_correct_kind = *GROUND_CORRECT_KIND_GROUND as u32;
    }

    // claude code: a slid-off move's next status keeps its air situation and correction
    if is_slide_off_air(boma) {
        situation = smash::app::SituationKind(*SITUATION_KIND_NONE);
        ground_correct_kind = *GROUND_CORRECT_KIND_AIR as u32;
    }

    // claude code: faux air's ground statuses stay in the air (modes/faux_air.rs)
    modes::faux_air::init_settings(boma, &mut situation, &mut ground_correct_kind);

    if boma.is_fighter() {
        if boma.is_prev_situation(*SITUATION_KIND_AIR)
        && (situation.0 == *SITUATION_KIND_GROUND || (boma.is_situation(*SITUATION_KIND_GROUND) && situation.0 == *SITUATION_KIND_NONE))
        && kinetic_type == *FIGHTER_KINETIC_TYPE_MOTION {
            kinetic_type = *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL;
        }
        JostleModule::set_team(boma, 0);
        ground_correct_kind = slide_off_correct_kind(ground_correct_kind as i32) as u32;
    }

    if VarModule::has_var_module(boma.object()) {
        let mut mask = 0;
        if keep_flag != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG {
            mask += VarModule::RESET_STATUS_FLAG;
        }
        if keep_int != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT {
            mask += VarModule::RESET_STATUS_INT + VarModule::RESET_STATUS_INT64;
        }
        if keep_float != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT {
            mask += VarModule::RESET_STATUS_FLOAT;
        }
        VarModule::reset(boma.object(), mask);
    }
    note_edge_stop(boma, asked_correct);

    let ret = original!()(boma, situation, kinetic_type, ground_correct_kind, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);
    modes::faux_air::keep_falling(boma);
    ret
}

#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, clear_buffer: bool) -> u64 {
    let mut status_kind = status_kind;
    if boma.is_fighter() {
        // claude code: pummels are removed, but a CPU's grab plan waits on them, so CPUs keep theirs
        // (is_cpu, mod.rs) rather than holding the grab forever
        if status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK && !is_cpu(boma) {
            return 0;
        }
        if let Some(kind) = removed_dodge_request(boma, status_kind) {
            match kind {
                Some(kind) => status_kind = kind,
                None => return 0
            }
        }
    }
    let clear_buffer = clear_buffer && !modes::faux_air::keeps_buffer(boma, status_kind);
    // claude code: universal no special fall (FALL_SPECIAL -> FALL, locked specials blocked)
    let status_kind = match filter_status_request(boma, status_kind) {
        Some(kind) => kind,
        None => return 0
    };
    original!()(boma, status_kind, clear_buffer)
}

// claude code: airdodges, spotdodges and rolls are removed. from a free state (standing, moving,
// shielding, landing, falling, jumping, tumbling...) a dodge request is the player's input, and
// refusing it leaves them where they were. any other status asking for a dodge is a move ending itself
// into one it stored (shield-cancelling a charge, for one), and refusing that left them stuck, so it
// becomes FALL in the air or WAIT on the ground. None = refuse, Some(kind) = change into kind, outer
// None = not a dodge. the dodge inputs themselves are switched off (is_enable_transition_term_hook,
// mod.rs), so the free-state refusal is only a backstop now
unsafe fn removed_dodge_request(boma: &mut BattleObjectModuleAccessor, status_kind: i32) -> Option<Option<i32>> {
    if ![
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE_B
    ].contains(&status_kind) {
        return None;
    }
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_WALK_BRAKE,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_TURN,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_F,
        *FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_OFF,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_PASS,
        *FIGHTER_STATUS_KIND_WALL_JUMP,
        *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ]) {
        return Some(None);
    }
    if boma.is_situation(*SITUATION_KIND_GROUND) {
        Some(Some(*FIGHTER_STATUS_KIND_WAIT))
    } else {
        Some(Some(*FIGHTER_STATUS_KIND_FALL))
    }
}

// claude code: universal no special fall (made from HDR's per-move edits, which swap a move's
// FALL_SPECIAL for FALL and lock it until landing or getting hit). the "used this airtime" flag for a
// generic special status, if it is one
pub unsafe fn special_used_flag(status_kind: i32) -> Option<i32> {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { Some(vars::common::instance::SPECIAL_N_USED) }
    else if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S { Some(vars::common::instance::SPECIAL_S_USED) }
    else if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI { Some(vars::common::instance::SPECIAL_HI_USED) }
    else if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW { Some(vars::common::instance::SPECIAL_LW_USED) }
    else { None }
}

// claude code: all four "used this airtime" flags
pub unsafe fn special_used_flags() -> [i32; 4] {
    [
        vars::common::instance::SPECIAL_N_USED,
        vars::common::instance::SPECIAL_S_USED,
        vars::common::instance::SPECIAL_HI_USED,
        vars::common::instance::SPECIAL_LW_USED,
    ]
}

// claude code: locks the special this fall came from (LAST_SPECIAL_KIND), or all four if it's unknown
pub unsafe fn lock_last_special(boma: &mut BattleObjectModuleAccessor) {
    let last = VarModule::get_int(boma.object(), vars::common::instance::LAST_SPECIAL_KIND);
    match special_used_flag(last) {
        Some(flag) => VarModule::on_flag(boma.object(), flag),
        None => for flag in special_used_flags() { VarModule::on_flag(boma.object(), flag); }
    }
}

// claude code: filter for every status request. Some(kind) = change into kind, None = block it.
// slid-off moves are handled first; each generic special that starts is remembered in
// LAST_SPECIAL_KIND (and IN_SPECIAL), locked specials are refused, and FALL_SPECIAL becomes FALL
pub unsafe fn filter_status_request(boma: &mut BattleObjectModuleAccessor, status_kind: i32) -> Option<i32> {
    // claude code: faux air's jumps, platform drops and falls go last (modes/faux_air.rs)
    let status_kind = filter_common_request(boma, status_kind)?;
    modes::faux_air::request(boma, status_kind)
}

unsafe fn filter_common_request(boma: &mut BattleObjectModuleAccessor, status_kind: i32) -> Option<i32> {
    if !boma.is_fighter() {
        return Some(status_kind);
    }
    if too_many_requests(boma) {
        return None;
    }
    // claude code: slippery ground moves (see slide_off_request)
    if let Ok(result) = slide_off_request(boma, status_kind) {
        return result;
    }
    if status_kind < *FIGHTER_STATUS_KIND_SPECIAL_N {
        VarModule::off_flag(boma.object(), vars::common::instance::IN_SPECIAL);
    }
    if let Some(flag) = special_used_flag(status_kind) {
        if VarModule::is_flag(boma.object(), flag) {
            return None;
        }
        VarModule::set_int(boma.object(), vars::common::instance::LAST_SPECIAL_KIND, status_kind);
        VarModule::on_flag(boma.object(), vars::common::instance::IN_SPECIAL);
    }
    else if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        lock_last_special(boma);
        return Some(*FIGHTER_STATUS_KIND_FALL);
    }
    Some(status_kind)
}

// claude code: crash guard. Ultimate runs a status change within the same frame, so two statuses that
// each ask for the other on their first frame loop forever and overflow the stack (how the CPU dodge
// stand-ins crashed the game, 2026-11-19). no real chain asks for anywhere near 32 changes in one
// frame, so past that every request that frame is refused: a loop stalls a frame instead of crashing.
// the count is reset every frame from sys_line (opff.rs)
const MAX_REQUESTS_PER_FRAME: i32 = 32;

unsafe fn too_many_requests(boma: &mut BattleObjectModuleAccessor) -> bool {
    let count = VarModule::get_int(boma.object(), vars::common::instance::STATUS_REQUESTS_THIS_FRAME) + 1;
    VarModule::set_int(boma.object(), vars::common::instance::STATUS_REQUESTS_THIS_FRAME, count);
    count > MAX_REQUESTS_PER_FRAME
}

// claude code: true when `term` would start a special that is locked this airtime. the opff
// `no_special_fall` gives specials back on landing, getting hit or dying
pub unsafe fn is_special_term_locked(boma: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let flag = if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND].contains(&term) {
        vars::common::instance::SPECIAL_N_USED
    } else if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND].contains(&term) {
        vars::common::instance::SPECIAL_S_USED
    } else if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND].contains(&term) {
        vars::common::instance::SPECIAL_HI_USED
    } else if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND].contains(&term) {
        vars::common::instance::SPECIAL_LW_USED
    } else {
        return false;
    };
    VarModule::is_flag(boma.object(), flag)
}

// claude code: non-script status requests get the same filter as change_status_request_from_script
#[skyline::hook(replace = StatusModule::change_status_request)]
unsafe fn change_status_request_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, clear_buffer: bool) -> u64 {
    let clear_buffer = clear_buffer && !modes::faux_air::keeps_buffer(boma, status_kind);
    match filter_status_request(boma, status_kind) {
        Some(kind) => original!()(boma, kind, clear_buffer),
        None => 0
    }
}

// claude code: walljump refresh for moves we have no list for (slotted movesets, mostly). a move that
// locks itself after one aerial use (and doesn't use special fall) turns a flag on or a counter up, and
// its own code clears it when the fighter lands. so: watch what a fighter clears on its landing frame
// (instance flags turned off, counters set to 0) and replay exactly that on a wall jump
// (opff.rs, wall_jump_restore_specials). learned per fighter kind + costume, since two movesets on one
// fighter (Armstrong, Springtrap...) give the same ID different meanings. only fighter-specific IDs
// count: the common ones (jump count and friends) stay out, so wall jumps don't give back jumps

// first fighter-specific instance IDs (FIGHTER_INSTANCE_WORK_ID_FLAG_TERM / _INT_TERM)
const FIGHTER_FLAG_FIRST: i32 = 0x200000E0;
const FIGHTER_FLAG_END: i32 = 0x20000400;
const FIGHTER_INT_FIRST: i32 = 0x100000BD;
const FIGHTER_INT_END: i32 = 0x10000400;

#[derive(Default)]
struct Learned {
    flags: BTreeSet<i32>,
    ints: BTreeSet<i32>,
}

static LEARNED: Mutex<BTreeMap<(i32, i32), Learned>> = Mutex::new(BTreeMap::new());

unsafe fn key(boma: &mut BattleObjectModuleAccessor) -> (i32, i32) {
    (boma.kind(), WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR))
}

// the frame a fighter really lands (not a slid-off move or faux air pretending to be grounded)
unsafe fn is_landing_frame(boma: &mut BattleObjectModuleAccessor) -> bool {
    boma.is_fighter()
    && StatusModule::is_situation_changed(boma)
    && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR
    && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
    && !is_slide_off_air(boma)
    && !modes::faux_air::is_faux_ground(boma)
}

#[skyline::hook(replace = WorkModule::off_flag)]
unsafe fn divergence_air_locks_off_flag(boma: &mut BattleObjectModuleAccessor, flag: i32) {
    if (FIGHTER_FLAG_FIRST..FIGHTER_FLAG_END).contains(&flag) && is_landing_frame(boma) {
        LEARNED.lock().unwrap().entry(key(boma)).or_default().flags.insert(flag);
    }
    original!()(boma, flag)
}

#[skyline::hook(replace = WorkModule::set_int)]
unsafe fn divergence_air_locks_set_int(boma: &mut BattleObjectModuleAccessor, value: i32, int: i32) {
    if value == 0 && (FIGHTER_INT_FIRST..FIGHTER_INT_END).contains(&int) && is_landing_frame(boma) {
        LEARNED.lock().unwrap().entry(key(boma)).or_default().ints.insert(int);
    }
    original!()(boma, value, int)
}

// claude code: called on a wall jump's first frame. clears what this fighter's landing clears
pub unsafe fn restore(boma: &mut BattleObjectModuleAccessor) {
    let (flags, ints): (Vec<i32>, Vec<i32>) = match LEARNED.lock().unwrap().get(&key(boma)) {
        Some(l) => (l.flags.iter().copied().collect(), l.ints.iter().copied().collect()),
        None => return,
    };
    for flag in flags {
        WorkModule::off_flag(boma, flag);
    }
    for int in ints {
        WorkModule::set_int(boma, 0, int);
    }
}

pub fn install() {
    skyline::install_hooks!(
        correct_hook,
        set_correct_hook,
        init_settings_hook,
        change_status_request_from_script_hook,
        change_status_request_hook,
        divergence_air_locks_off_flag,
        divergence_air_locks_set_int
    );
}
