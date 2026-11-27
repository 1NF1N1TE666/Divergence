use super::*;

#[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    // claude code: divergent variants rewrite their own hitboxes (divergent_variants.rs)
    crate::fighters::common::divergent_variants::rewrite_attack(sv_system::battle_object_module_accessor(lua_state), &mut hitbox_params);
    let is_autolink = hitbox_params[4].get_i32() >= 362;
    // claude code: aura mode, damage times Aura, and a projectile's size takes it (modes.rs)
    let boma = &mut *sv_system::battle_object_module_accessor(lua_state);
    let aura = modes::power_mul(boma);
    modes::scale_projectile(boma);
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
    // claude code: aura mode, throws and other set hits times Aura too (modes.rs)
    let aura = modes::power_mul(&mut *sv_system::battle_object_module_accessor(lua_state));
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

// hdr: this and attack_module_set_power_hook_pattern are HDR's (fighters/common/src/function_hooks/attack.rs)
#[skyline::hook(offset = 0x46b648, inline)]
unsafe fn get_hitstop_mul(ctx: &mut skyline::hooks::InlineCtx) {
    if ctx.registers[1].w() == 0x2 {
        let hitstop_mul: f32 = 1.0;
        ctx.registers_f[0].set_s(hitstop_mul)
    }
}

#[skyline::hook(offset = 0x3dd688, inline)]
unsafe extern "C" fn attack_module_set_power_hook_pattern(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers_f[2].set_s(1.0);
}

// claude code: every hit on a fighter is logged here (FighterManager::notify_log_event_collision_hit,
// whose lua_bind wrapper jumps to this function). an attacking fighter whose status has
// FORCE_OPPONENT_KNOCKDOWN on gives the fighter it hit FORCE_KNOCKDOWN (no floor bounce, no tech)
#[skyline::hook(offset = 0x67a7b0)]
unsafe fn notify_log_event_collision_hit(fighter_manager: u64, attacker_id: u32, defender_id: u32, power: f32, collision_kind: i32, flag: bool) -> u64 {
    if sv_battle_object::is_active(attacker_id)
    && sv_battle_object::is_active(defender_id) {
        let attacker = sv_battle_object::module_accessor(attacker_id);
        let defender = sv_battle_object::module_accessor(defender_id);
        if (*attacker).is_fighter()
        && (*defender).is_fighter()
        && VarModule::is_flag((*attacker).object(), vars::common::status::FORCE_OPPONENT_KNOCKDOWN) {
            VarModule::on_flag((*defender).object(), vars::common::instance::FORCE_KNOCKDOWN);
        }
    }
    call_original!(fighter_manager, attacker_id, defender_id, power, collision_kind, flag)
}

pub fn install() {
    skyline::install_hooks!(
        attack_replace,
        attack_abs_replace,
        get_hitstop_mul,
        attack_module_set_power_hook_pattern,
        notify_log_event_collision_hit
    );
}
