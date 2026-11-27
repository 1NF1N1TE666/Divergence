use super::*;
use skyline::hooks::{getRegionAddress, Region};

static mut INT_OFFSET: usize = 0x4e53a0;
static mut FLOAT_OFFSET: usize = 0x4e53e0;

// claude code: universal movement rules for fighters' own attributes: traction is halved, and fast fall
// speed ignores its own value: it's always fall speed x1.6. `get` reads another attribute the way the
// hook sees it (slotted values included). None leaves the param as it is
unsafe fn universal_param(param_type: u64, ret: f32, get: impl Fn(u64) -> f32) -> Option<f32> {
    if param_type == hash40("dive_speed_y") {
        return Some(get(hash40("air_speed_y_stable")) * 1.6);
    }
    if param_type == hash40("ground_brake") {
        return Some(ret * 0.5);
    }
    // claude code: a fighter whose initial dash is faster than their run trades the two, so every run
    // is the faster of the pair and every dash the slower. foxtrotting (re-dashing, `re_dash_frame`) is
    // control and pivots, running is speed, and the player picks one
    if param_type == hash40("dash_speed") {
        return Some(ret.min(get(hash40("run_speed_max"))));
    }
    if param_type == hash40("run_speed_max") {
        return Some(ret.max(get(hash40("dash_speed"))));
    }
    None
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

// claude code: skyline exports every hook by its function name, and lib_paramconfig exports its own
// get_param_int_hook / get_param_float_hook. with the same names, the loader can hand Divergence lib_paramconfig's
// functions when Divergence installs its hooks, and the match load hangs. so Divergence's names carry a _divergence suffix
#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_hook_divergence(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let ret = original!()(module_accessor, param_type, param_hash);
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    // claude code: the fighter's own params, on Divergence's slots (src/config.rs)
    if let Some(config::Param::Int(value)) = config::slot_param(boma, param_type, param_hash) {
        return value;
    }
    if param_hash == 0
    && param_type == hash40("wall_jump_type")
    && boma.is_fighter() {
        return 1;
    }
    ret
}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_hook_divergence(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let ret = original!()(module_accessor, param_type, param_hash);
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    // claude code: the fighter's own params, on Divergence's slots (src/config.rs). a vl param is final; a
    // fighter attribute (param_hash 0) still goes through everything below, universal rules included
    let ret = match config::slot_param(boma, param_type, param_hash) {
        Some(config::Param::Float(value)) if param_hash != 0 => return value,
        Some(config::Param::Float(value)) => value,
        _ => ret
    };
    if param_hash == 0
    && [
        hash40("landing_attack_air_frame_n"),
        hash40("landing_attack_air_frame_f"),
        hash40("landing_attack_air_frame_b"),
        hash40("landing_attack_air_frame_hi"),
        hash40("landing_attack_air_frame_lw"),
        hash40("landing_frame"),
        hash40("landing_frame_light")
    ].contains(&param_type) {
        return 3.0;
    }
    // claude code: the footstool box (its size and the extra margin around it) is 1.2x as big
    if param_hash == 0
    && boma.is_fighter()
    && [
        hash40("tread_range_w"),
        hash40("tread_range_h"),
        hash40("tread_range_add_w"),
        hash40("tread_range_add_h")
    ].contains(&param_type) {
        return ret * 1.2;
    }
    // claude code: the game turns each jump height into a launch speed once, at load (0x6f21d0:
    // sqrt(2 x gravity x height) + gravity / 2), and jumps read that saved speed. slotted heights and
    // gravity differ by slot, so for every fighter with slotted params the speeds are worked out live
    // from the current values instead
    if param_hash == 0
    && boma.is_fighter()
    && config::has_params(boma.kind()) {
        let height = [
            (hash40("jump_speed_y"), hash40("jump_y")),
            (hash40("mini_jump_speed_y"), hash40("mini_jump_y")),
            (hash40("jump_aerial_speed_y"), hash40("jump_aerial_y")),
        ].iter().find(|(speed, _)| *speed == param_type).map(|(_, height)| *height);
        if let Some(height) = height {
            let gravity = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
            let height = WorkModule::get_param_float(boma, height, 0);
            return (2.0 * gravity * height).sqrt() + gravity * 0.5;
        }
    }
    // claude code: universal movement rules (fighters only)
    if param_hash == 0
    && boma.is_fighter() {
        let original = original!();
        let fighter = boma as *mut BattleObjectModuleAccessor;
        let get = |hash| match config::slot_param(fighter, hash, 0) {
            Some(config::Param::Float(value)) => value,
            _ => original(module_accessor, hash, 0)
        };
        return universal_param(param_type, ret, get).unwrap_or(ret);
    }
    ret
}

// claude code: the hooks go in once the game loads its common nro, after every plugin has started.
// lib_paramconfig hooks these same functions when it starts, so Divergence's hooks always end up outside
// its: they see each moveset's edited stats, and Divergence's universal values (landing lag, wall jumps,
// traction, fast fall) hold on every slot
fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name != "common" {
        return;
    }
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
    }
    skyline::install_hooks!(
        get_param_int_hook_divergence,
        get_param_float_hook_divergence
    );
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
