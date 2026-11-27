use super::*;

// claude code: no jab locks. a hit on a fighter lying on the ground never becomes the lying hit
// (DOWN_DAMAGE, which keeps them down for follow-up locks and a forced getup); it becomes a normal
// standing hit reaction instead, so the hit's own knockback decides what happens
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DownDamage)]
unsafe fn status_pre_DownDamage(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.boma().set_status_kind_interrupt(*FIGHTER_STATUS_KIND_DAMAGE);
    1.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_DownDamage
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
