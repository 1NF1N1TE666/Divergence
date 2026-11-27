use super::*;
use globals::*;

// wizard's dropkick off the ground comes down before the kick is over. the kick's animation is 2.51 a
// frame in Ultimate, 1.75 and slowing in Sm4sh, so from the landing on it's brought down to the 1.2 that
// Sm4sh ends the dropkick at
unsafe extern "C" fn dropkick_landing(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if !boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW)
    || !boma.is_situation(*SITUATION_KIND_GROUND)
    || !boma.is_motion(Hash40::new("special_lw"))
    || boma.motion_frame() <= 13.0
    || config::custom_number(boma, Special::Down) != 2 {
        return;
    }
    let energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut app::FighterKineticEnergyMotion;
    lua_bind::FighterKineticEnergyMotion::set_speed_mul(energy, 1.2 / 2.51);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, dropkick_landing);
}
