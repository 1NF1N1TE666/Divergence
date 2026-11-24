use super::*;

#[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    // claude code: divergent variants rewrite their own hitboxes (common/src/variants.rs)
    crate::fighters::common::variants::on_attack(sv_system::battle_object_module_accessor(lua_state), &mut hitbox_params);
    let is_autolink = hitbox_params[4].get_i32() >= 362;
    // claude code: aura mode, damage times Aura, and a projectile's size takes it (modes/aura.rs)
    let boma = &mut *sv_system::battle_object_module_accessor(lua_state);
    let aura = modes::aura::power_mul(boma);
    modes::aura::scale_projectile(boma);
    for i in 0..36 {
        if i == 3 {
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(hitbox_params[i].get_f32() * 1.2 * aura));
        } else if i == 18 && !is_autolink {
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*ATTACK_LR_CHECK_POS as u64));
        } else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
        }
    }
    original!()(lua_state);
}

#[skyline::hook(replace = smash::app::sv_animcmd::ATTACK_ABS)]
unsafe fn attack_abs_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let params: Vec<L2CValue> = (0..16).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    let is_throw = params[0].get_i32() == *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW;
    // claude code: aura mode, throws and other set hits times Aura too (modes/aura.rs)
    let aura = modes::aura::power_mul(&mut *sv_system::battle_object_module_accessor(lua_state));
    for i in 0..16 {
        if i == 2 {
            let mul = if is_throw { 1.2 } else { 1.0 };
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(params[i].get_f32() * mul * aura));
        } else {
            l2c_agent.push_lua_stack(&mut params[i].clone());
        }
    }
    original!()(lua_state);
}

// claude code: aura mode, power a status sets on a hitbox after it's out (counters, charged
// projectiles) gets the Aura too (modes/aura.rs)
#[skyline::hook(replace = AttackModule::set_power)]
unsafe fn set_power_hook(boma: &mut BattleObjectModuleAccessor, id: i32, power: f32, arg4: bool) {
    let mul = modes::aura::power_mul(boma);
    original!()(boma, id, power * mul, arg4)
}

pub fn install() {
    skyline::install_hooks!(
        attack_replace,
        attack_abs_replace,
        set_power_hook
    );
}
