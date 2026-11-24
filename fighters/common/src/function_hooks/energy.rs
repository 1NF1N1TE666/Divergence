#![allow(improper_ctypes_definitions)]

use super::*;
use std::mem::transmute;
use smash_rs::cpp::simd::{Vector2, Vector3, Vector4};

// hdr: based on HDR's motion energy rewrite (fighters/common/src/function_hooks/energy/motion.rs and
// energy/mod.rs): the energy layouts, the game functions below and motion_update's flow. condensed here
#[derive(Copy, Clone)]
#[repr(C)]
struct PaddedVec2 { x: f32, y: f32, padding: u64 }

impl PaddedVec2 {
    fn new(x: f32, y: f32) -> Self { Self { x, y, padding: 0 } }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
enum ResetType {
    GroundTransLoop = 0x0, GroundTransLoopGekikara, GroundTrans, GroundTransIgnoreNorm,
    AirTrans, AirTransAngle, AirTransY, AirTransAngleSuperJumpPunch, AirTrans2nd,
    CliffTransIntp, CliffTrans, CliffTransGround, LadderMove, LadderTrans,
}

#[repr(C)]
struct MotionEnergy {
    _vtable: u64,
    _x8: u64,
    speed: PaddedVec2,
    _rot_speed: PaddedVec2,
    _enable: bool,
    _unk: [u8; 0xF],
    accel: PaddedVec2,
    speed_max: PaddedVec2,
    speed_brake: PaddedVec2,
    speed_limit: PaddedVec2,
    _x80: u8,
    _consider_ground_friction: bool,
    active_flag: bool,
    _x83: u8,
    reset_type: ResetType,
    lr: f32,
    angle: f32,
    angle_whole: f32,
    angle_intp_end: f32,
    angle_intp_frames_remaining: i32,
    speed_mul: f32,
    prev_speed: PaddedVec2,
    speed_mul_2nd: PaddedVec2,
    update_flag: bool,
}

#[skyline::from_offset(0x47bf90)]
extern "C" fn process_energy(energy: &mut MotionEnergy, boma: &mut BattleObjectModuleAccessor);
#[skyline::from_offset(0x47b4f0)]
extern "C" fn adjust_speed_for_ground_normal(speed: Vector2, boma: &mut BattleObjectModuleAccessor) -> Vector2;
#[skyline::from_offset(0x6941e0)]
extern "C" fn handle_cliff(boma: &mut BattleObjectModuleAccessor, vec: &Vector4f) -> Vector4;

impl MotionEnergy {
    unsafe fn set_and_process(&mut self, accel: PaddedVec2, speed_max: PaddedVec2, speed: PaddedVec2, boma: &mut BattleObjectModuleAccessor) {
        self.accel = accel;
        self.speed_max = speed_max;
        process_energy(self, boma);
        self.active_flag = true;
        self.prev_speed = speed;
    }
}

#[skyline::hook(offset = 0x6d5cb0)]
unsafe fn motion_update(energy: &mut MotionEnergy, boma: &mut BattleObjectModuleAccessor) {
    use ResetType::*;
    let reset_type = energy.reset_type;
    let is_ground = matches!(reset_type, GroundTransLoop | GroundTransLoopGekikara | GroundTrans | GroundTransIgnoreNorm);
    let is_2nd = reset_type == AirTrans2nd;
    let zero = PaddedVec2::new(0.0, 0.0);
    let mm = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 8);
    let vt = *(mm as *const *const u64);

    energy.active_flag = true;
    let is_updating_energy: extern "C" fn(u64) -> bool = transmute(*vt.add(if is_2nd { 0x1f0 / 8 } else { 0x1e8 / 8 }));
    if !is_updating_energy(mm) {
        if reset_type == LadderMove {
            energy.set_and_process(PaddedVec2::new(-energy.speed.x, -energy.speed.y), zero, zero, boma);
            return;
        }
        let backup_brake = energy.speed_brake;
        if is_ground {
            energy.speed_limit = PaddedVec2::new(boma.get_param_float("common", "ground_speed_limit"), 0.0);
        }
        energy.set_and_process(zero, zero, zero, boma);
        energy.speed_brake = backup_brake;
        return;
    }

    // claude code: a held jab 1 restarts the jab over and over, and each restart would carry the last
    // jab's own forward motion into the stop speed, stacking it every loop until it flies off stage.
    // a jab started straight out of a jab carries no momentum
    let is_jab_repeat = boma.is_status(*FIGHTER_STATUS_KIND_ATTACK)
        && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK;
    let mut is_stop_added = false;
    if !energy.update_flag && !is_jab_repeat && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3]) {
        let stop_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        let main_speed = KineticModule::get_sum_speed3f(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_GROUND, &Vector2f { x: main_speed.x, y: main_speed.y }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, boma);
        lua_bind::KineticEnergy::enable(stop_energy);
        is_stop_added = true;
    }

    if is_2nd {
        let update_2nd: extern "C" fn(u64) = transmute(*vt.add(0x220 / 8));
        update_2nd(mm);
    } else {
        MotionModule::update_trans_move_speed(boma);
    }
    let trans_move_speed: extern "C" fn(&mut BattleObjectModuleAccessor) -> Vector3 = transmute(
        if is_2nd { MotionModule::trans_move_speed_2nd as *const () } else { MotionModule::trans_move_speed as *const () }
    );
    let trans = trans_move_speed(boma);
    let mut move_speed = PaddedVec2::new(trans.z(), trans.y());

    if energy.angle_intp_frames_remaining >= 1 {
        energy.angle_whole += (energy.angle_intp_end - energy.angle_whole) / energy.angle_intp_frames_remaining as f32;
        energy.angle_intp_frames_remaining -= 1;
    }
    if energy.angle_whole != 0.0 {
        move_speed.x = move_speed.x * energy.angle_whole.cos() - move_speed.y * energy.angle_whole.sin();
        move_speed.y = move_speed.y * energy.angle_whole.cos() + move_speed.x * energy.angle_whole.sin();
    }

    let lr = if matches!(reset_type, GroundTransLoop | GroundTransLoopGekikara | AirTransAngle | AirTransAngleSuperJumpPunch) {
        boma.lr()
    } else {
        energy.lr
    };
    move_speed.x *= lr * energy.speed_mul * energy.speed_mul_2nd.x;
    move_speed.y *= energy.speed_mul * energy.speed_mul_2nd.y;
    // claude code: aura mode, a move's animation movement times Aura (modes/aura.rs)
    let aura = modes::aura::mobility_mul(boma);
    move_speed.x *= aura;
    move_speed.y *= aura;

    if boma.status_frame() == 0 {
        move_speed.x = if is_stop_added { 0.0 } else { energy.prev_speed.x };
    }

    energy.active_flag = matches!(reset_type, GroundTransLoop | GroundTransLoopGekikara | CliffTransGround | LadderMove | LadderTrans);
    if is_ground {
        energy.speed_limit = PaddedVec2::new(-1.0, 0.0);
    }

    let speed = match reset_type {
        GroundTransLoop | GroundTransIgnoreNorm | AirTrans | AirTrans2nd => move_speed,
        GroundTrans => {
            let v = adjust_speed_for_ground_normal(Vector2 { vec: [move_speed.x, move_speed.y] }, boma);
            PaddedVec2::new(v.x(), v.y())
        }
        GroundTransLoopGekikara => {
            let some_rate = boma.get_float(0x1000009);
            let motion_rate = MotionModule::rate(boma);
            if some_rate != 0.0 && motion_rate / some_rate != 0.0 {
                PaddedVec2::new(move_speed.x * some_rate / motion_rate, move_speed.y * some_rate / motion_rate)
            } else {
                zero
            }
        }
        AirTransAngle => PaddedVec2::new(move_speed.x * energy.angle.cos() - move_speed.y * energy.angle.sin(), move_speed.y * energy.angle.cos() + move_speed.x * energy.angle.sin()),
        AirTransY => PaddedVec2::new(0.0, move_speed.y),
        AirTransAngleSuperJumpPunch => {
            let stick_x = ControlModule::get_stick_x(boma);
            let dir = boma.get_float(*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
            if stick_x.abs() > dir {
                let interp = (stick_x.abs() - dir) / (1.0 - dir) * boma.get_float(*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
                let new_angle = if stick_x <= 0.0 { interp.to_radians() } else { -interp.to_radians() };
                if energy.angle.abs() < new_angle.abs() {
                    energy.angle = new_angle;
                }
            }
            let angle = energy.angle;
            PaddedVec2::new(move_speed.x * angle.cos() - move_speed.y * energy.angle.sin(), move_speed.y * angle.cos() + move_speed.x * energy.angle.sin())
        }
        CliffTransIntp | CliffTrans | CliffTransGround => {
            let cliff_trans: extern "C" fn(u64) -> Vector4 = transmute(*vt.add(0x230 / 8));
            let v = cliff_trans(mm);
            let v = handle_cliff(boma, &Vector4f { x: v.x(), y: v.y(), z: v.z(), w: v.w() });
            let mul = if reset_type == CliffTransIntp { 1.0 / (boma.get_int(0x11000005) + 1) as f32 } else { 1.0 };
            PaddedVec2::new(v.x() * mul, v.y() * mul)
        }
        LadderMove => {
            let stick_y = ControlModule::get_stick_y(boma);
            let speed_y = if 0.5 > stick_y.abs() || stick_y.is_nan() {
                0.0
            } else if stick_y <= 0.0 {
                -boma.get_param_float("common", "ladder_speed_d_max") * MotionModule::rate(boma)
            } else {
                boma.get_param_float("common", "ladder_speed_u_max") * MotionModule::rate(boma)
            };
            PaddedVec2::new(0.0, speed_y)
        }
        LadderTrans => {
            let mut tra = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            MotionModule::trans_tra(boma, &mut tra, true, true);
            let speed_y = boma.get_float(*FIGHTER_STATUS_LADDER_WORK_FLOAT_LADDER_END_Y) + tra.y - boma.get_float(*FIGHTER_STATUS_LADDER_WORK_FLOAT_LADDER_END_START_Y);
            WorkModule::add_float(boma, speed_y, *FIGHTER_STATUS_LADDER_WORK_FLOAT_LADDER_END_START_Y);
            PaddedVec2::new(0.0, speed_y)
        }
    };

    if is_ground && energy.update_flag && speed.x == 0.0 && energy.prev_speed.x == 0.0 {
        energy.set_and_process(zero, zero, speed, boma);
        return;
    }
    let from = if energy.update_flag {
        energy.prev_speed
    } else {
        energy.update_flag = true;
        energy.speed
    };
    energy.set_and_process(PaddedVec2::new(speed.x - from.x, speed.y - from.y), PaddedVec2::new(-1.0, -1.0), speed, boma);
}

// claude code: aura mode (modes/aura.rs). the speeds a move's own code gives the stop energy (the
// speed it sets, plus the accel, brake and caps that shape it) are times Aura, so the path stretches
// and the timing stays the same. the speeds it gives the gravity energy (launches and hops, 2026-11-23,
// Bandana Dee's up-B missed it) are times the square root of Aura: gravity itself is left alone, so a
// launch √Aura faster rises exactly Aura times as high. the code reading either speed back gets it
// divided back down, so code that reads a speed and sets it again (slowing it each frame) doesn't stack
// the Aura frame on frame. each setter takes the energy id then its values (`count` of them) off the
// lua stack; the ones in `scaled` get the multiplier. a call that doesn't look like that (another
// energy, or a value that isn't a number) is left as it is
struct EnergyArgs<'a> {
    count: usize,
    scaled: &'a [usize],
}

// the multiplier for an energy's speeds (stop: Aura, gravity: √Aura), 1 for the rest
unsafe fn energy_mul(lua_state: u64, energy_id: i32) -> f32 {
    let boma = &mut *sv_system::battle_object_module_accessor(lua_state);
    let aura = modes::aura::mobility_mul(boma);
    if energy_id == *FIGHTER_KINETIC_ENERGY_ID_STOP {
        aura
    } else if energy_id == *FIGHTER_KINETIC_ENERGY_ID_GRAVITY {
        aura.sqrt()
    } else {
        1.0
    }
}

unsafe fn scale_energy(lua_state: u64, stop: Option<EnergyArgs>, gravity: Option<EnergyArgs>) {
    let mut agent = L2CAgent::new(lua_state);
    let energy_id = agent.pop_lua_stack(1).get_i32();
    let args = if energy_id == *FIGHTER_KINETIC_ENERGY_ID_STOP {
        stop
    } else if energy_id == *FIGHTER_KINETIC_ENERGY_ID_GRAVITY {
        gravity
    } else {
        None
    };
    let Some(args) = args else { return };
    let mul = energy_mul(lua_state, energy_id);
    if mul == 1.0 {
        return;
    }
    scale_lua_args(lua_state, args.count + 1, args.scaled, mul);
}

// takes `count` arguments off the lua stack and puts them back, the ones in `scaled` times `mul`.
// left alone if any of those isn't a number
unsafe fn scale_lua_args(lua_state: u64, count: usize, scaled: &[usize], mul: f32) {
    let mut agent = L2CAgent::new(lua_state);
    let mut params: Vec<L2CValue> = (0..count).map(|i| agent.pop_lua_stack(i as i32 + 1)).collect();
    if scaled.iter().any(|&i| !matches!(params[i].val_type, L2CValueType::Num | L2CValueType::Int)) {
        return;
    }
    agent.clear_lua_stack();
    for (i, param) in params.iter_mut().enumerate() {
        if scaled.contains(&i) {
            agent.push_lua_stack(&mut L2CValue::new_num(param.get_f32() * mul));
        } else {
            agent.push_lua_stack(param);
        }
    }
}

unsafe fn energy_read_mul(lua_state: u64) -> f32 {
    let energy_id = L2CAgent::new(lua_state).pop_lua_stack(1).get_i32();
    1.0 / energy_mul(lua_state, energy_id)
}

const XY: EnergyArgs = EnergyArgs { count: 2, scaled: &[1, 2] };

// stop (id, x, y), gravity (id, y)
#[skyline::hook(replace = smash::app::sv_kinetic_energy::set_speed)]
unsafe fn sv_set_speed(lua_state: u64) {
    scale_energy(lua_state, Some(XY), Some(EnergyArgs { count: 1, scaled: &[1] }));
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_kinetic_energy::add_speed)]
unsafe fn sv_add_speed(lua_state: u64) {
    scale_energy(lua_state, Some(XY), None);
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_kinetic_energy::set_accel)]
unsafe fn sv_set_accel(lua_state: u64) {
    scale_energy(lua_state, Some(XY), None);
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_kinetic_energy::set_brake)]
unsafe fn sv_set_brake(lua_state: u64) {
    scale_energy(lua_state, Some(XY), None);
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_kinetic_energy::set_limit_speed)]
unsafe fn sv_set_limit_speed(lua_state: u64) {
    scale_energy(lua_state, Some(XY), None);
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_kinetic_energy::set_stable_speed)]
unsafe fn sv_set_stable_speed(lua_state: u64) {
    scale_energy(lua_state, Some(XY), None);
    original!()(lua_state);
}

// (id, reset type, x, y, z, ...): stop x and y, gravity y
#[skyline::hook(replace = smash::app::sv_kinetic_energy::reset_energy)]
unsafe fn sv_reset_energy(lua_state: u64) {
    scale_energy(lua_state, Some(EnergyArgs { count: 6, scaled: &[2, 3] }), Some(EnergyArgs { count: 6, scaled: &[3] }));
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_kinetic_energy::get_speed_x)]
unsafe fn sv_get_speed_x(lua_state: u64) -> f32 {
    let mul = energy_read_mul(lua_state);
    original!()(lua_state) * mul
}

#[skyline::hook(replace = smash::app::sv_kinetic_energy::get_speed_y)]
unsafe fn sv_get_speed_y(lua_state: u64) -> f32 {
    let mul = energy_read_mul(lua_state);
    original!()(lua_state) * mul
}

// claude code: aura mode, an ACMD script setting a move's speed outright (x, y, attribute) gets it
// times Aura, like the stop energy above
#[skyline::hook(replace = smash::app::sv_animcmd::SET_SPEED_EX)]
unsafe fn set_speed_ex(lua_state: u64) {
    let boma = &mut *sv_system::battle_object_module_accessor(lua_state);
    let aura = modes::aura::mobility_mul(boma);
    if aura != 1.0 {
        scale_lua_args(lua_state, 3, &[0, 1], aura);
    }
    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        motion_update,
        sv_set_speed,
        sv_add_speed,
        sv_set_accel,
        sv_set_brake,
        sv_set_limit_speed,
        sv_set_stable_speed,
        sv_reset_energy,
        sv_get_speed_x,
        sv_get_speed_y,
        set_speed_ex
    );
}
