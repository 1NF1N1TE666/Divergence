use super::*;
use config::Special;

const SPECIALN_2_RATES: [f32; 54] = [0.739, 0.642, 0.518, 0.394, 0.318, 0.291, 0.296, 0.345, 0.468, 0.625, 0.694, 0.622, 0.517, 0.531, 0.672, 0.797, 0.846, 0.852, 0.851, 0.853, 0.856, 0.861, 0.866, 0.869, 0.872, 0.872, 0.870, 0.867, 0.871, 0.887, 0.915, 0.945, 0.967, 0.978, 0.984, 0.987, 0.988, 0.986, 0.982, 0.975, 0.965, 0.947, 0.921, 0.897, 0.896, 0.910, 0.925, 0.931, 0.929, 0.923, 0.912, 0.892, 0.861, 0.824];
const SPECIALN_3_RATES: [f32; 54] = [1.388, 1.683, 2.276, 3.175, 4.070, 4.715, 5.116, 5.106, 4.313, 2.984, 2.132, 2.389, 3.228, 3.625, 3.271, 2.780, 2.568, 2.552, 2.559, 2.544, 2.509, 2.462, 2.416, 2.379, 2.355, 2.350, 2.374, 2.402, 2.366, 2.204, 1.930, 1.637, 1.424, 1.311, 1.256, 1.228, 1.218, 1.231, 1.272, 1.340, 1.421, 1.460, 1.382, 1.201, 1.048, 0.982, 0.971, 0.971, 0.971, 0.971, 0.971, 0.971, 0.971, 0.971];
const SPECIALS_2_RATES: [f32; 58] = [1.103, 1.149, 1.186, 1.203, 1.192, 1.151, 1.098, 1.063, 1.057, 1.069, 1.081, 1.089, 1.095, 1.110, 1.124, 1.152, 1.218, 1.307, 1.408, 1.541, 1.722, 1.920, 2.085, 2.203, 2.281, 2.314, 2.301, 2.263, 2.188, 2.036, 1.816, 1.611, 1.479, 1.419, 1.420, 1.467, 1.494, 1.448, 1.357, 1.301, 1.318, 1.390, 1.500, 1.662, 1.876, 2.082, 2.204, 2.216, 2.134, 2.008, 1.906, 1.866, 1.867, 1.863, 1.849, 1.886, 2.048, 2.325];
const SPECIALS_3_RATES: [f32; 58] = [1.170, 1.249, 1.311, 1.346, 1.361, 1.363, 1.335, 1.294, 1.271, 1.293, 1.308, 1.300, 1.284, 1.316, 1.351, 1.308, 1.240, 1.199, 1.209, 1.259, 1.336, 1.398, 1.399, 1.345, 1.300, 1.282, 1.276, 1.267, 1.251, 1.218, 1.170, 1.125, 1.096, 1.083, 1.086, 1.102, 1.117, 1.111, 1.089, 1.075, 1.080, 1.097, 1.125, 1.166, 1.220, 1.271, 1.302, 1.305, 1.284, 1.253, 1.227, 1.217, 1.217, 1.216, 1.213, 1.222, 1.263, 1.332];

const SPECIALHI_2_RATES: [f32; 37] = [0.667, 0.523, 0.413, 0.398, 0.500, 1.225, 1.721, 2.061, 2.437, 2.858, 3.132, 3.066, 2.719, 2.366, 2.214, 2.220, 2.217, 2.115, 1.964, 1.825, 1.711, 1.625, 1.566, 1.518, 1.471, 1.438, 1.426, 1.429, 1.434, 1.450, 1.500, 1.592, 1.707, 1.808, 1.872, 1.923, 1.990];
const SPECIALHI_3_RATES: [f32; 37] = [1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 0.804, 0.634, 0.497, 0.406, 0.377, 0.410, 0.465, 0.488, 0.474, 0.471, 0.490, 0.484, 0.423, 0.453, 0.624, 1.000, 2.257, 2.911, 3.041, 3.031, 3.042, 3.066, 3.143, 3.375, 3.806, 4.349, 4.824, 5.122, 5.359, 5.675];
const SPECIALAIRHI_3_RATES: [f32; 37] = [1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 0.804, 0.635, 0.498, 0.406, 0.377, 0.410, 0.465, 0.488, 0.474, 0.471, 0.490, 0.482, 0.423, 0.453, 0.624, 1.000, 2.217, 2.835, 2.938, 2.910, 2.918, 2.976, 3.128, 3.412, 3.828, 4.374, 4.951, 5.355, 5.537, 5.623];
unsafe extern "C" fn game_specialn_fast(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALN_2_RATES, 14.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    agent.boma().rate_curve(agent, &SPECIALN_2_RATES, 43.0);
    agent.boma().rate_curve(agent, &SPECIALN_2_RATES, 54.0);
}

unsafe extern "C" fn game_specialn_orb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 14.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 43.0);
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 54.0);
}

unsafe extern "C" fn effect_specialn_orb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1.5, true);
        }
    } else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1.5, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.353);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, false);
    }
}

unsafe extern "C" fn effect_specialairn_orb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 13.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1.5, true);
        }
    } else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1.5, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.353);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, false);
    }
}

unsafe extern "C" fn expression_specialn_orb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0x50000000);
    }
}

unsafe extern "C" fn game_specials_shocking(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, -1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    agent.boma().rate_curve(agent, &SPECIALS_2_RATES, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.2, 361, 105, 0, 30, 8.5, 0.0, 6.7, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.2, 361, 105, 0, 30, 5.0, 0.0, 6.7, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    agent.boma().rate_curve(agent, &SPECIALS_2_RATES, 16.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
    agent.boma().rate_curve(agent, &SPECIALS_2_RATES, 36.0);
    agent.boma().rate_curve(agent, &SPECIALS_2_RATES, 58.0);
}

unsafe extern "C" fn game_specials_gust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, -1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    agent.boma().rate_curve(agent, &SPECIALS_3_RATES, 6.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    agent.boma().rate_curve(agent, &SPECIALS_3_RATES, 9.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    agent.boma().rate_curve(agent, &SPECIALS_3_RATES, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 160, 100, 80, 0, 6.0, 0.0, 6.7, 10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 160, 100, 80, 0, 4.0, 0.0, 6.7, 5.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
    }
    agent.boma().rate_curve(agent, &SPECIALS_3_RATES, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 115, 0, 60, 8.0, 0.0, 6.7, 10.0, Some(0.0), Some(6.7), Some(3.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 1, Hash40::new("top"), 0.0, 361, 115, 0, 36, 12.0, 0.0, 6.7, 21.0, Some(0.0), Some(6.7), Some(4.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
    agent.boma().rate_curve(agent, &SPECIALS_3_RATES, 17.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALS_3_RATES, 23.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
    agent.boma().rate_curve(agent, &SPECIALS_3_RATES, 36.0);
    agent.boma().rate_curve(agent, &SPECIALS_3_RATES, 58.0);
}

unsafe extern "C" fn effect_specials_shocking(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, 9.5, 0, 0, 0, 1, true, *EF_FLIP_NONE);
        EFFECT(agent, Hash40::new("mario_supermant_flash"), Hash40::new("top"), 0, 5.5, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
    for _ in 0..3 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 7, 10, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 0, true);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 9, 14, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 0, true);
        }
        wait(lua_state, 3.0);
    }
}

unsafe extern "C" fn effect_specials_gust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, 9.5, 0, 0, 0, 1.1, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT(agent, Hash40::new("mario_supermant_flash"), Hash40::new("top"), 0, 5.5, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, 20.5, 0, 0, -5, 2, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 6, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn effect_specialairs_shocking(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, 9.5, 0, 0, 0, 1, true, *EF_FLIP_NONE);
        EFFECT(agent, Hash40::new("mario_supermant_flash"), Hash40::new("top"), 0, 5.5, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
    frame(lua_state, 13.0);
    for _ in 0..3 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 7, 10, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 0, true);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 9, 14, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 0, true);
        }
        wait(lua_state, 3.0);
    }
}

unsafe extern "C" fn effect_specialairs_gust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, 9.5, 0, 0, 0, 1.1, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT(agent, Hash40::new("mario_supermant_flash"), Hash40::new("top"), 0, 5.5, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mario_supermant_wind_r"), Hash40::new("mario_supermant_wind_l"), Hash40::new("top"), 2.5, 5, 20.5, 0, 0, -5, 2, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

unsafe extern "C" fn sound_specials_shocking(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_s01"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_elec_l_damage"));
    }
}

unsafe extern "C" fn expression_specials_gust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, 0x50000000);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_01_mantle"), 0);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 1.0, 0.0, 300.0, 0.5, 12.0, 10.0, 30.0, 20.0, 50);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn game_specialhi_super(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 2.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 3.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 4.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 5.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 12.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 20.0);
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 37.0);
}

unsafe extern "C" fn game_specialhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 2.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 3.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 4.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 5.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        SA_SET(agent, *SITUATION_KIND_AIR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 96, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 90, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 17.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 19.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 70, 102, 0, 38, 6.75, 0.0, 13.5, 1.75, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 24.0);
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 37.0);
}

unsafe extern "C" fn game_specialairhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 2.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 3.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 4.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 5.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        SA_SET(agent, *SITUATION_KIND_AIR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 96, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 90, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 17.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 19.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 70, 102, 0, 38, 6.75, 0.0, 13.5, 1.75, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 24.0);
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 37.0);
}

unsafe extern "C" fn effect_specialhi_super(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mario_superjump_power"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mario_final_handfire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, -0.4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_superjump_power"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("mario_final_handfire"), false, false);
    }
}

unsafe extern "C" fn effect_specialairhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mario_superjump_power"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mario_final_handfire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, -0.4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_superjump_power"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("mario_final_handfire"), false, false);
    }
}

unsafe extern "C" fn sound_specialhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if WorkModule::is_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
        frame(lua_state, 7.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_mario_special_h03"));
        }
    } else {
        frame(lua_state, 7.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_mario_special_h03"));
        }
    }
}

unsafe extern "C" fn expression_specialhi_super(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0x50000000);
    }
}

unsafe extern "C" fn expression_specialhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, 0x50000000);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0x50000000);
    }
}

unsafe extern "C" fn effect_speciallwhold_pressure(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    loop {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 0.9, 8, 0, 10, 0, 0, 0, true);
            FLASH(agent, 0.275, 0.588, 1, 0.588);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 0.784, 1, 1, 0.431);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 10.0);
    }
}

unsafe extern "C" fn effect_specialairlwhold_pressure(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    loop {
        if is_excute(agent) {
            FLASH(agent, 0.275, 0.588, 1, 0.588);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 0.784, 1, 1, 0.431);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 10.0);
    }
}

unsafe extern "C" fn effect_speciallwchargemax_pressure(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mario_pump_max"), Hash40::new("rot"), 0, 2.5, 1, 0.00001, 0.00007, 0.00002, 1.4, false);
    }
}

unsafe extern "C" fn expression_speciallwheavy_pressure(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_waterjetl"), 40, true, 0x50000000);
    }
}

pub fn install(agent: &mut Agent) {
    config::custom_acmd!(agent, Special::Neutral, "game_specialn", 2 => game_specialn_fast, 3 => game_specialn_orb);
    config::custom_acmd!(agent, Special::Neutral, "game_specialairn", 2 => game_specialn_fast, 3 => game_specialn_orb);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialn", 3 => effect_specialn_orb);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairn", 3 => effect_specialairn_orb);
    config::custom_acmd!(agent, Special::Neutral, "expression_specialn", 3 => expression_specialn_orb);
    config::custom_acmd!(agent, Special::Neutral, "expression_specialairn", 3 => expression_specialn_orb);
    config::custom_acmd!(agent, Special::Side, "game_specials", 2 => game_specials_shocking, 3 => game_specials_gust);
    config::custom_acmd!(agent, Special::Side, "game_specialairs", 2 => game_specials_shocking, 3 => game_specials_gust);
    config::custom_acmd!(agent, Special::Side, "effect_specials", 2 => effect_specials_shocking, 3 => effect_specials_gust);
    config::custom_acmd!(agent, Special::Side, "effect_specialairs", 2 => effect_specialairs_shocking, 3 => effect_specialairs_gust);
    config::custom_acmd!(agent, Special::Side, "sound_specials", 2 => sound_specials_shocking);
    config::custom_acmd!(agent, Special::Side, "sound_specialairs", 2 => sound_specials_shocking);
    config::custom_acmd!(agent, Special::Side, "expression_specials", 3 => expression_specials_gust);
    config::custom_acmd!(agent, Special::Side, "expression_specialairs", 3 => expression_specials_gust);
    config::custom_acmd!(agent, Special::Up, "game_specialhi", 2 => game_specialhi_super, 3 => game_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "game_specialairhi", 2 => game_specialhi_super, 3 => game_specialairhi_explosive);
    config::custom_acmd!(agent, Special::Up, "effect_specialhi", 2 => effect_specialhi_super, 3 => effect_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "effect_specialairhi", 2 => acmd_stub, 3 => effect_specialairhi_explosive);
    config::custom_acmd!(agent, Special::Up, "sound_specialhi", 3 => sound_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "sound_specialairhi", 3 => sound_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "expression_specialhi", 2 => expression_specialhi_super, 3 => expression_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "expression_specialairhi", 2 => expression_specialhi_super, 3 => expression_specialhi_explosive);
    config::custom_acmd!(agent, Special::Down, "effect_speciallwhold", 3 => effect_speciallwhold_pressure);
    config::custom_acmd!(agent, Special::Down, "effect_specialairlwhold", 3 => effect_specialairlwhold_pressure, _ => acmd_stub);
    config::custom_acmd!(agent, Special::Down, "effect_speciallwchargemax", 3 => effect_speciallwchargemax_pressure);
    config::custom_acmd!(agent, Special::Down, "expression_speciallwheavy", 3 => expression_speciallwheavy_pressure);
    config::custom_acmd!(agent, Special::Down, "expression_specialairlwheavy", 3 => expression_speciallwheavy_pressure);
}
