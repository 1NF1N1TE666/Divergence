use super::*;

pub mod attack;
pub mod collision;
pub mod damage;
pub mod energy;
pub mod get_param;
pub mod jumps;
pub mod lua_bind_hook;
pub mod momentum_transfer;
pub mod transition;

// claude code: air correction, with gravity and a fall speed applied (like HDR's air-continuing dash
// attacks). used by slid-off moves (lua_bind_hook.rs) and the faux air mode (modes.rs)
pub unsafe fn air_physics(boma: &mut BattleObjectModuleAccessor, speed_y: f32, fall_speed: f32) {
    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    let air_accel_y = boma.get_param_float("air_accel_y", "");
    let gravity = boma.get_gravity_energy() as *mut smash::app::FighterKineticEnergyGravity;
    lua_bind::FighterKineticEnergyGravity::set_speed(gravity, speed_y);
    lua_bind::FighterKineticEnergyGravity::set_accel(gravity, -air_accel_y);
    lua_bind::FighterKineticEnergyGravity::set_stable_speed(gravity, fall_speed);
    lua_bind::FighterKineticEnergyGravity::set_limit_speed(gravity, fall_speed);
    KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

// claude code: the FighterManager singleton, null before it exists (is_cpu, modes.rs)
pub unsafe fn fighter_manager() -> *mut smash::app::FighterManager {
    static mut MANAGER: usize = 0;
    if MANAGER == 0 {
        skyline::nn::ro::LookupSymbol(&mut MANAGER as *mut usize, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\0".as_ptr() as _);
        if MANAGER == 0 {
            return std::ptr::null_mut();
        }
    }
    *(MANAGER as *const *mut smash::app::FighterManager)
}

// claude code: true for a CPU-controlled fighter (amiibo included). CPUs still play by vanilla's
// rules: their AI asks for things Divergence took out (rolls, spotdodges, pummels), and where Divergence just
// refuses those, a human lets go of the button but a CPU keeps asking, every frame, and freezes in
// place. the places that refuse them check this and give CPUs a working stand-in instead
pub unsafe fn is_cpu(boma: &mut BattleObjectModuleAccessor) -> bool {
    if !boma.is_fighter() {
        return false;
    }
    let manager = fighter_manager();
    if manager.is_null() {
        return false;
    }
    // fighters outside the 8 player entries (bosses, spirit stand-ins...) have nothing to look up
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if !(0..8).contains(&entry_id) {
        return false;
    }
    let info = lua_bind::FighterManager::get_fighter_information(manager, smash::app::FighterEntryID(entry_id));
    !info.is_null() && lua_bind::FighterInformation::is_operation_cpu(info)
}

pub fn install() {
    attack::install();
    collision::install();
    damage::install();
    energy::install();
    get_param::install();
    jumps::install();
    lua_bind_hook::install();
    momentum_transfer::install();
    transition::install();

    unsafe {
        // hdr: HDR's binary patches (fighters/common/src/function_hooks/mod.rs)
        skyline::patching::Patch::in_text(0x6d21a8).data(0x52800015u32);
        skyline::patching::Patch::in_text(0x4ceae8).data(0x140000D1u32);
        skyline::patching::Patch::in_text(0x3fd360).nop();
        skyline::patching::Patch::in_text(0x614fb4).data(0x54000060);
    }
}
