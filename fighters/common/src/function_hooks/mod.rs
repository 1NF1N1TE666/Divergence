use super::*;

pub mod attack;
pub mod damage;
pub mod collision;
pub mod energy;
pub mod jumps;
pub mod params;

pub mod lua_bind_hook;

// claude code: air correction, with gravity and a fall speed applied (like HDR's air-continuing dash
// attacks). used by slid-off moves (lua_bind_hook.rs)
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

// claude code: true for a CPU-controlled fighter (amiibo included). CPUs still play by vanilla's
// rules: their AI asks for things Divergence took out (rolls, spotdodges, pummels), and where Divergence just
// refuses those, a human lets go of the button but a CPU keeps asking, every frame, and freezes in
// place. the places that refuse them check this and give CPUs a working stand-in instead
pub unsafe fn is_cpu(boma: &mut BattleObjectModuleAccessor) -> bool {
    static mut MANAGER: usize = 0;
    if !boma.is_fighter() {
        return false;
    }
    if MANAGER == 0 {
        skyline::nn::ro::LookupSymbol(&mut MANAGER as *mut usize, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\0".as_ptr() as _);
        if MANAGER == 0 {
            return false;
        }
    }
    let manager = *(MANAGER as *const *mut smash::app::FighterManager);
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

#[skyline::hook(replace = GroundModule::can_entry_cliff)]
unsafe fn can_entry_cliff_hook(boma: &mut BattleObjectModuleAccessor) -> bool {
    false
}

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
    ].contains(&flag) {
        return false;
    }
    // claude code: specials locked this airtime (universal no special fall)
    if boma.is_fighter() && lua_bind_hook::is_special_term_locked(boma, flag) {
        return false;
    }
    original!()(boma, flag)
}

#[skyline::hook(replace=KineticModule::change_kinetic)]
unsafe fn change_kinetic_hook(boma: &mut BattleObjectModuleAccessor, kinetic_type: i32) -> i32 {
    original!()(boma, kinetic_type)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_AttackAir)]
pub unsafe fn status_pre_AttackAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    original!()(fighter)
}

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
    // claude code: a slid-off move's scripts still see the ground (lua_bind_hook::is_slide_off_air)
    if lua_bind_hook::is_slide_off_air(boma) {
        return *SITUATION_KIND_GROUND;
    }
    original!()(boma)
}

#[skyline::hook(replace=WorkModule::enable_transition_term)]
unsafe fn enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN {
        VarModule::on_flag(boma.object(), vars::common::status::IS_DASH_TO_RUN_FRAME);
    }
    original!()(boma, flag)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_AttackAir
        );
    }
}

pub fn install() {
    attack::install();
    damage::install();
    collision::install();
    energy::install();
    jumps::install();
    params::install();
    lua_bind_hook::install();

    skyline::install_hooks!(
        can_entry_cliff_hook,
        is_enable_transition_term_hook,
        change_kinetic_hook,
        situation_kind_replace,
        enable_transition_term_hook
    );
    skyline::nro::add_hook(nro_hook).unwrap();

    unsafe {
        skyline::patching::Patch::in_text(0x6d21a8).data(0x52800015u32);
        skyline::patching::Patch::in_text(0x4ceae8).data(0x140000D1u32);
        skyline::patching::Patch::in_text(0x3fd360).nop();
        skyline::patching::Patch::in_text(0x614fb4).data(0x54000060);
    }
}
