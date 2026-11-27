#![allow(improper_ctypes_definitions)]

// claude code: tether wall latch. there are no ledges, so a tether grabs walls instead, the way Byleth's
// Sword of the Creator does: it latches on, reels the fighter in and wall jumps. the pieces:
// - who and how far: HDR's list of tether zairs (fighters/common/src/function_hooks/lua_bind_hook/ground.rs,
//   tether_zair) and the reach of their ledge boxes, from Dionysus' tether table (dionysus_core/src/tether.rs)
// - the tether itself: the game's own AIR_LASSO statuses, each fighter's own called through the original,
//   named and ordered like Dionysus' fighter template (custom_fighters/template_v2/src/tether.rs). they look
//   for a ledge, so while latched the ledge checks answer with the wall
// - the limits, the angle and lining up with the wall: Byleth's (param_special_hi, SPECIAL_HI_WALL_JUMP)
// zairs, the tether specials and the Belmonts' whip aerials
use super::*;
use smash_rs::cpp::simd::Vector4;

const WALL_HIT_MAX: i32 = 1; // wall_hit_max, latches until landing
// degrees off upright (Byleth's wall_angle_max is 69; Grant set 45, 2026-11-27). wall jumps share it
// (general_statuses::slanted_wall_jump)
pub const WALL_ANGLE_MAX: f32 = 45.0;

// how far a tether looks for a wall, from its ledge box (cliff_hang_data in the fighter's vl.prc): forward,
// up and down from where it aims. forward < 0 looks behind (Belmont back airs)
struct Reach {
    forward: f32,
    up: f32,
    down: f32,
    from_shoulder: bool // zairs aim from the shoulder; the rest from the fighter's position, like their ledge box
}

const fn reach_box(forward: f32, up: f32, down: f32) -> Option<Reach> {
    Some(Reach { forward, up, down, from_shoulder: false })
}

const fn zair_box(forward: f32, up: f32, down: f32) -> Option<Reach> {
    Some(Reach { forward, up, down, from_shoulder: true })
}

// the tethers in their ledge search window: zairs (AIR_LASSO), the tether specials and the Belmonts' whip
// aerials, the moves HDR lists in can_entry_cliff (tether_zair, tether_special, tether_aerial). Byleth's up
// special grabs walls already
unsafe fn reach(boma: &mut BattleObjectModuleAccessor) -> Option<Reach> {
    let kind = boma.kind();
    let status = boma.status();
    if [*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER].contains(&kind) {
        if status != *FIGHTER_STATUS_KIND_ATTACK_AIR
        || !boma.is_flag(*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK) {
            return None;
        }
        // the box their AttackAir main loop picks for each whip motion (FIGHTER_SIMON_CLIFF_HANG_DATA_AIR_LASSO_*)
        return match MotionModule::motion_kind(boma) {
            0xc3495ada5 => reach_box(70.0, 40.0, -20.0), // F
            0xfdbf74a7b => reach_box(70.0, 30.0, -40.0), // F_LW
            0xfcaf6254b => reach_box(-70.0, 58.0, -15.0), // B_HI
            0xc33f869bc => reach_box(-70.0, 40.0, -20.0), // B
            0xf5495dd2c => reach_box(-70.0, 30.0, -40.0), // B_LW
            0xdde67d935 => reach_box(35.0, 70.0, 18.0), // HI
            _ => reach_box(70.0, 58.0, -15.0) // F_HI
        };
    }
    if !boma.is_flag(*FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK) {
        return None;
    }
    if status == *FIGHTER_STATUS_KIND_AIR_LASSO {
        return if [*FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD].contains(&kind) {
            zair_box(57.0, 44.0, -34.0)
        } else if kind == *FIGHTER_KIND_SZEROSUIT {
            zair_box(60.0, 40.0, -10.0)
        } else if [*FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_LUCAS].contains(&kind) {
            zair_box(60.0, 48.0, -34.0)
        } else {
            None
        };
    }
    // the specials' AIR_LASSO boxes (cliff_hang_data index 1)
    if kind == *FIGHTER_KIND_SZEROSUIT && status == *FIGHTER_STATUS_KIND_SPECIAL_S {
        reach_box(60.0, 40.0, -10.0)
    } else if kind == *FIGHTER_KIND_SHIZUE && status == *FIGHTER_STATUS_KIND_SPECIAL_S {
        reach_box(60.0, 55.0, -25.0)
    } else if kind == *FIGHTER_KIND_JACK && status == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        reach_box(62.0, 70.0, -25.0)
    } else if kind == *FIGHTER_KIND_PFUSHIGISOU && status == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        reach_box(72.0, 65.0, -52.0)
    } else if kind == *FIGHTER_KIND_TANTAN && status == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR {
        reach_box(67.0, 70.0, -25.0)
    } else {
        None
    }
}

pub unsafe fn is_latched(boma: &mut BattleObjectModuleAccessor) -> bool {
    boma.is_fighter() && VarModule::is_flag(boma.object(), vars::common::instance::TETHER_WALL)
}

// the wall spot, kept where the ledge checks can point to it: one per player
unsafe fn wall_pos(boma: &mut BattleObjectModuleAccessor) -> *const [f32; 4] {
    static mut WALL_POS: [[f32; 4]; 8] = [[0.0; 4]; 8];
    let entry = boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID).clamp(0, 7) as usize;
    WALL_POS[entry] = [
        VarModule::get_float(boma.object(), vars::common::instance::TETHER_WALL_X),
        VarModule::get_float(boma.object(), vars::common::instance::TETHER_WALL_Y),
        PostureModule::pos_z(boma),
        0.0
    ];
    &WALL_POS[entry]
}

// looks for a wall within reach: level first (or the box's height nearest level), then up and down to the
// corners of the ledge box. gives the spot and the side it's on (1 = right)
unsafe fn find_wall(boma: &mut BattleObjectModuleAccessor, reach: &Reach) -> Option<(Vector2f, f32)> {
    let side = boma.lr() * reach.forward.signum();
    let scale = PostureModule::scale(boma);
    let start = if reach.from_shoulder {
        let shoulder = &mut Vector3f::zero();
        ModelModule::joint_global_position(boma, Hash40::new("shoulderr"), shoulder, true);
        Vector2f::new(shoulder.x, shoulder.y)
    } else {
        Vector2f::new(PostureModule::pos_x(boma), PostureModule::pos_y(boma))
    };
    for y in [0.0f32.clamp(reach.down, reach.up), reach.up, reach.down] {
        let ray = Vector2f::new(reach.forward.abs() * side * scale, y * scale);
        let pos = &mut Vector2f::zero();
        let normal = &mut Vector2f::zero();
        if GroundModule::ray_check_hit_pos_normal(boma, &start, &ray, pos, normal, true) & 1 != 0
        && normal.x * side < 0.0
        && normal.y.atan2(normal.x.abs()).to_degrees().abs() < WALL_ANGLE_MAX {
            return Some((Vector2f::new(pos.x, pos.y), side));
        }
    }
    None
}

// a tether's own ledge check, asked by its status in the search window (FLAG_CHECK): a wall in reach latches
// on and counts as the ledge, the same frame, so even a one-frame window (Zero Suit Samus' side special,
// Ivysaur's up special) catches it. the status then makes its own rope and goes to the reach, as it would for
// a ledge
unsafe fn try_latch(boma: &mut BattleObjectModuleAccessor) -> bool {
    if is_latched(boma) {
        return true;
    }
    if !boma.is_fighter()
    || !boma.is_situation(*SITUATION_KIND_AIR)
    || VarModule::get_int(boma.object(), vars::common::instance::TETHER_WALL_COUNT) >= WALL_HIT_MAX {
        return false;
    }
    let Some(reach) = reach(boma) else { return false };
    let Some((pos, side)) = find_wall(boma, &reach) else { return false };
    VarModule::on_flag(boma.object(), vars::common::instance::TETHER_WALL);
    VarModule::inc_int(boma.object(), vars::common::instance::TETHER_WALL_COUNT);
    VarModule::set_float(boma.object(), vars::common::instance::TETHER_WALL_X, pos.x);
    VarModule::set_float(boma.object(), vars::common::instance::TETHER_WALL_Y, pos.y);
    VarModule::set_float(boma.object(), vars::common::instance::TETHER_WALL_LR, side);
    true
}

// every status request (lua_bind_hook.rs filter): latches reset on the ground, and a latch lets go once the
// fighter heads anywhere but the tether's own statuses
unsafe fn latch_request(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if boma.is_situation(*SITUATION_KIND_GROUND) {
        VarModule::set_int(boma.object(), vars::common::instance::TETHER_WALL_COUNT, 0);
    }
    if is_latched(boma) && ![
        *FIGHTER_STATUS_KIND_AIR_LASSO_REACH,
        *FIGHTER_STATUS_KIND_AIR_LASSO_HANG,
        *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND,
        *FIGHTER_STATUS_KIND_WALL_JUMP
    ].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_WALL);
    }
}

// a tether status corrects its position to the ledge it's on. on a wall it's plain air correction, like
// Byleth's
unsafe fn air_correct(fighter: &mut L2CFighterCommon) {
    if is_latched(fighter.boma()) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
}

unsafe extern "C" fn reach_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH)(fighter);
    air_correct(fighter);
    ret
}

// the hang is skipped on a wall: it reels in right away
unsafe extern "C" fn hang_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG)(fighter);
    air_correct(fighter);
    if is_latched(fighter.boma()) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_AIR_LASSO_REWIND, false);
    }
    ret
}

unsafe extern "C" fn rewind_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND)(fighter);
    air_correct(fighter);
    ret
}

// the wall jump out of the reel (walljump.rs): their middle (rot) on the spot the rope held, then out from it so
// they aren't inside the wall, facing away from it: the wall jump's push is along the way a fighter faces
// (kinetic WALL_JUMP: passive_wall_jump_x_speed * lr)
pub unsafe fn leave_wall(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if !is_latched(boma) {
        return;
    }
    VarModule::off_flag(fighter.battle_object, vars::common::instance::TETHER_WALL);
    let lr = VarModule::get_float(fighter.battle_object, vars::common::instance::TETHER_WALL_LR);
    let wall = *wall_pos(boma);
    let rot = &mut Vector3f::zero();
    ModelModule::joint_global_position(boma, Hash40::new("rot"), rot, true);
    let pos = *PostureModule::pos(boma);
    let out = 4.0 * PostureModule::scale(boma);
    PostureModule::set_pos(boma, &Vector3f::new(pos.x + wall[0] - rot.x - lr * out, pos.y + wall[1] - rot.y, pos.z));
    PostureModule::set_lr(boma, -lr);
    PostureModule::update_rot_y_lr(boma);
}

// a latched tether's statuses check for a ledge the way they always do, and find the wall. a ledge grab the
// reach or hang asks for becomes the reel, and one the reel asks for becomes the wall jump. one asked for
// before the rope is out (by the move that latched) is refused: there's no ledge to grab
pub unsafe fn ledge_request(boma: &mut BattleObjectModuleAccessor, status_kind: i32) -> Option<i32> {
    let status_kind = if !is_latched(boma)
    || ![
        *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT
    ].contains(&status_kind) {
        status_kind
    } else if boma.is_status(*FIGHTER_STATUS_KIND_AIR_LASSO_REWIND) {
        *FIGHTER_STATUS_KIND_WALL_JUMP
    } else if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_AIR_LASSO_REACH, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG]) {
        *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND
    } else {
        return None;
    };
    latch_request(boma, status_kind);
    Some(status_kind)
}

// the ledge checks of the tether statuses: in a tether's search window they look for a wall (try_latch), and
// while latched they find it. Joker's and Min Min's up specials (and the Belmonts' up air) ask this one, then
// can_entry_cliff (lua_bind_hook.rs, false for everything else: no ledges)
#[skyline::hook(replace = GroundModule::is_status_cliff)]
unsafe fn divergence_is_status_cliff(boma: &mut BattleObjectModuleAccessor) -> bool {
    try_latch(boma) || original!()(boma)
}

// the common ledge check the rest ask: the zairs (common AirLasso), Zero Suit Samus' and Isabelle's side
// specials, Ivysaur's up special and the Belmonts' whip aerials
#[skyline::hook(replace = L2CFighterCommon_sub_fighter_general_term_is_cliff_check_pos)]
unsafe fn sub_fighter_general_term_is_cliff_check_pos(fighter: &mut L2CFighterCommon) -> L2CValue {
    if try_latch(fighter.boma()) {
        return L2CValue::Bool(true);
    }
    original!()(fighter)
}

#[skyline::hook(replace = GroundModule::hang_cliff_dir)]
unsafe fn divergence_hang_cliff_dir(boma: &mut BattleObjectModuleAccessor) -> f32 {
    if is_latched(boma) {
        return -VarModule::get_float(boma.object(), vars::common::instance::TETHER_WALL_LR);
    }
    original!()(boma)
}

#[skyline::hook(replace = GroundModule::hang_cliff_pos)]
unsafe fn divergence_hang_cliff_pos(boma: &mut BattleObjectModuleAccessor) -> u64 {
    if is_latched(boma) {
        return wall_pos(boma) as u64;
    }
    original!()(boma)
}

#[skyline::hook(replace = GroundModule::hang_cliff_pos_3f)]
unsafe fn divergence_hang_cliff_pos_3f(boma: &mut BattleObjectModuleAccessor) -> Vector4 {
    if is_latched(boma) {
        return Vector4 { vec: *wall_pos(boma) };
    }
    original!()(boma)
}

// Fighter's link event handler (13.0.4 and 13.0.5; each fighter's own ON_LINK_EVENT runs inside it). the rope asks its
// fighter for the spot it hangs from, and tells it when the reel is done: that's the wall jump. the event's
// kind is at 0x10, its answer at 0x28 (answered) and 0x30 (the spot)
#[skyline::hook(offset = 0x635390)]
unsafe fn divergence_on_link_event(fighter: *mut BattleObject, event: *mut u8) -> u64 {
    let boma = &mut *(*fighter).module_accessor;
    if !is_latched(boma) {
        return call_original!(fighter, event);
    }
    let kind = *(event.add(0x10) as *const u64) & 0xFF_FFFF_FFFF;
    if kind == hash40("weapon_lasso_hang_rewind_end") {
        StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, false);
        return 0;
    }
    let ret = call_original!(fighter, event);
    if kind == hash40("weapon_lasso_cliff_pos") {
        *event.add(0x28) = 1;
        *(event.add(0x30) as *mut [f32; 4]) = *wall_pos(boma);
    }
    ret
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(sub_fighter_general_term_is_cliff_check_pos);
    }
}

pub fn install() {
    skyline::install_hooks!(
        divergence_is_status_cliff,
        divergence_hang_cliff_dir,
        divergence_hang_cliff_pos,
        divergence_hang_cliff_pos_3f,
        divergence_on_link_event
    );
    skyline::nro::add_hook(nro_hook);
    smashline::Agent::new("fighter")
        .status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, reach_main)
        .status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, hang_main)
        .status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND, rewind_main)
        .install();
}
