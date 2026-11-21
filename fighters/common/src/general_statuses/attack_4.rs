use super::*;

// claude code: no smash attack charging (Smash 64): forward smash skips its hold status
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_AttackS4Hold)]
unsafe fn status_pre_AttackS4Hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.boma().set_status_kind_interrupt(*FIGHTER_STATUS_KIND_ATTACK_S4);
    1.into()
}

// claude code: the same for up smash
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_AttackHi4Hold)]
unsafe fn status_pre_AttackHi4Hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.boma().set_status_kind_interrupt(*FIGHTER_STATUS_KIND_ATTACK_HI4);
    1.into()
}

// claude code: the same for down smash
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_AttackLw4Hold)]
unsafe fn status_pre_AttackLw4Hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.boma().set_status_kind_interrupt(*FIGHTER_STATUS_KIND_ATTACK_LW4);
    1.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_AttackS4Hold,
            status_pre_AttackHi4Hold,
            status_pre_AttackLw4Hold
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
