use super::*;

#[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    let is_autolink = hitbox_params[4].get_i32() >= 362;
    for i in 0..36 {
        if i == 3 {
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(hitbox_params[i].get_f32() * 1.2));
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
    for i in 0..16 {
        if i == 2 && is_throw {
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(params[i].get_f32() * 1.2));
        } else {
            l2c_agent.push_lua_stack(&mut params[i].clone());
        }
    }
    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        attack_replace,
        attack_abs_replace
    );
}
