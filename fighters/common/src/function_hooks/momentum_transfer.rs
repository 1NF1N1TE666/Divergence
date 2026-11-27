use super::*;

// hdr: based on HDR's momentum transfer (fighters/common/src/function_hooks/momentum_transfer.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_AttackAir)]
pub unsafe fn status_pre_AttackAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    original!()(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_AttackAir
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
