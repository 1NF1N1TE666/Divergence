use super::*;
use config::Special;

unsafe extern "C" fn effect_specialn_mighty(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("captain_fp_body_r"), Hash40::new("captain_fp_body_l"), Hash40::new("top"), 0, 8, 7, 0, 0, 0, 1.5, true, *EF_FLIP_XY);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("captain_fp_wing_r"), Hash40::new("captain_fp_wing_r"), Hash40::new("wingr2"), -6, 0, -2, -10, -10, 0, 1.05, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("captain_fp_wing_l"), Hash40::new("captain_fp_wing_l"), Hash40::new("wingl2"), 6, 0, -2, -10, 10, 0, 1.05, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_body_r"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_body_l"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_wing_r"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_wing_l"), -1);
    }
}

unsafe extern "C" fn effect_specialnhit_mighty(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("captain_fp_body_r"), Hash40::new("captain_fp_body_l"), Hash40::new("top"), 0, 8, 7, 0, 0, 0, 1.5, true, *EF_FLIP_XY);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("captain_fp_wing_r"), Hash40::new("captain_fp_wing_r"), Hash40::new("wingr2"), -6, 0, -2, -10, -10, 0, 1.05, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("captain_fp_wing_l"), Hash40::new("captain_fp_wing_l"), Hash40::new("wingl2"), 6, 0, -2, -10, 10, 0, 1.05, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("captain_fp_hit_r"), Hash40::new("captain_fp_hit_r"), Hash40::new("wingr2"), -6, 2, -2, -10, -10, 0, 1.05, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("captain_fp_hit_l"), Hash40::new("captain_fp_hit_l"), Hash40::new("wingl2"), 6, 2, -2, -10, 10, 0, 1.05, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_body_r"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_body_l"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_wing_r"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_wing_l"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_hit_r"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("captain_fp_hit_l"), -1);
    }
}

pub fn install(agent: &mut Agent) {
    config::custom_acmd!(agent, Special::Neutral, "effect_specialn", 3 => effect_specialn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialnhit", 3 => effect_specialnhit_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialnturn", 3 => effect_specialn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialnturnhit", 3 => effect_specialnhit_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairn", 3 => effect_specialn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairnhit", 3 => effect_specialnhit_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairnturn", 3 => effect_specialn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairnturnhit", 3 => effect_specialnhit_mighty);
}
