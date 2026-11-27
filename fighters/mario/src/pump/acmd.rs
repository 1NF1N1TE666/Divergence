use super::*;
use config::Special;

unsafe extern "C" fn effect_light_scalding(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mario_pump_shoot"), Hash40::new("mouth"), 0, 0, 0, 0, 90, 0, 0.9, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_steam3"), Hash40::new("body"), 0, 0, 0, 0, 0, -90, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_pump_shoot"), false, true);
    }
}

unsafe extern "C" fn effect_light_pressure(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mario_pump_shoot"), Hash40::new("mouth"), 0, 0, 0, 0, 90, 0, 1.125, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_pump_shoot"), false, true);
    }
}

unsafe extern "C" fn effect_heavy_scalding(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mario_pump_shoot"), Hash40::new("mouth"), 0, 0, 0, 0, 90, 0, 1.3, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_steam3"), Hash40::new("body"), 0, 0, 0, 0, 0, -90, 1.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_pump_shoot"), false, true);
    }
}

unsafe extern "C" fn effect_heavy_pressure(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mario_pump_shoot"), Hash40::new("mouth"), 0, 0, 0, 0, 90, 0, 1.56, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_pump_shoot"), false, true);
    }
}

unsafe extern "C" fn effect_hold_pressure(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mario_pump_drop"), Hash40::new("mouth"), 0, 0, 0, 0, 90, 0, 1.3, true);
        EFFECT_FOLLOW(agent, Hash40::new("mario_pump_tank"), Hash40::new("body"), -1.9, -1.6, 0, 90, 90, 0, 1.3, true);
    }
}

pub fn install(agent: &mut Agent) {
    config::custom_acmd!(agent, Special::Down, "effect_hold", 3 => effect_hold_pressure);
    config::custom_acmd!(agent, Special::Down, "effect_light", 2 => effect_light_scalding, 3 => effect_light_pressure);
    config::custom_acmd!(agent, Special::Down, "effect_heavy", 2 => effect_heavy_scalding, 3 => effect_heavy_pressure);
}
