use super::*;
use config::Special;

const SPECIALAIRHI_2_RATES: [f32; 37] = [0.659, 0.523, 0.417, 0.401, 0.500, 1.230, 1.725, 2.060, 2.435, 2.856, 3.130, 3.064, 2.720, 2.367, 2.215, 2.221, 2.217, 2.116, 1.964, 1.825, 1.711, 1.624, 1.562, 1.509, 1.455, 1.415, 1.400, 1.402, 1.414, 1.448, 1.508, 1.597, 1.713, 1.836, 1.922, 1.961, 1.979];
const SPECIALAIRHI_3_RATES: [f32; 37] = [1., 1., 1., 1., 1., 1., 1., 0.7661, 0.6052, 0.4746, 0.3869, 0.3592, 0.3907, 0.4431, 0.465, 0.4517, 0.4488, 0.467, 0.4612, 0.423, 0.453, 0.624, 1., 2.217, 2.835, 2.938, 2.91, 2.918, 2.976, 3.128, 3.412, 3.828, 4.374, 4.951, 5.355, 5.537, 5.623];
const SPECIALAIRLW_2_RATES: [f32; 91] = [1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.004, 1.003, 0.994, 0.964, 0.955, 0.954, 0.962, 1.011, 1.154, 1.405, 1.632, 1.814, 1.962, 2.077, 2.144, 2.152, 2.112, 2.035, 1.918, 1.749, 1.554, 1.427, 1.380, 1.371, 1.351, 1.302, 1.241, 1.201, 1.192, 1.207, 1.238, 1.285, 1.342, 1.400, 1.450, 1.487, 1.502, 1.495, 1.479, 1.466, 1.462, 1.455, 1.413, 1.303, 1.150, 1.037, 0.993, 0.986, 0.986, 0.986, 0.986, 0.986, 0.986, 0.986, 0.986, 0.986, 0.986, 0.986, 0.986, 0.986];
const SPECIALAIRLW_3_RATES: [f32; 91] = [1.428, 1.682, 1.954, 2.212, 2.359, 2.299, 2.090, 1.976, 2.092, 1.908, 1.390, 1.076, 0.967, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.950, 0.972, 1.096, 1.392, 1.765, 1.908, 1.898, 1.962, 2.169, 2.298, 2.132, 1.868, 1.664, 1.571, 1.564, 1.587, 1.591, 1.570, 1.532, 1.485, 1.434, 1.386, 1.352, 1.340, 1.336, 1.318, 1.273, 1.218, 1.181, 1.173, 1.187, 1.215, 1.258, 1.310, 1.362, 1.409, 1.442, 1.455, 1.449, 1.434, 1.423, 1.419, 1.413, 1.375, 1.275, 1.136, 1.034, 0.994, 0.987, 0.987, 0.987, 0.987, 0.987, 0.987, 0.987, 0.987, 0.987, 0.987, 0.987, 0.987, 0.987];
const SPECIALAIRN_2_RATES: [f32; 54] = [0.732, 0.600, 0.473, 0.369, 0.309, 0.289, 0.302, 0.367, 0.503, 0.656, 0.712, 0.629, 0.520, 0.539, 0.689, 0.818, 0.863, 0.861, 0.853, 0.852, 0.855, 0.861, 0.868, 0.875, 0.880, 0.882, 0.879, 0.873, 0.870, 0.877, 0.897, 0.924, 0.947, 0.958, 0.964, 0.969, 0.974, 0.979, 0.981, 0.979, 0.972, 0.960, 0.943, 0.928, 0.924, 0.928, 0.930, 0.926, 0.918, 0.908, 0.898, 0.880, 0.851, 0.809];
const SPECIALAIRN_3_RATES: [f32; 54] = [1.832, 2.378, 2.983, 3.565, 4.029, 4.379, 4.494, 4.089, 3.176, 2.301, 2.156, 2.921, 3.879, 4.019, 3.271, 2.593, 2.387, 2.443, 2.516, 2.534, 2.503, 2.443, 2.373, 2.309, 2.259, 2.241, 2.269, 2.327, 2.357, 2.290, 2.093, 1.824, 1.606, 1.493, 1.440, 1.393, 1.340, 1.294, 1.275, 1.294, 1.344, 1.375, 1.315, 1.170, 1.041, 0.985, 0.976, 0.976, 0.976, 0.976, 0.976, 0.976, 0.976, 0.976];
const SPECIALAIRS_2_RATES: [f32; 58] = [1.026, 1.069, 1.116, 1.155, 1.176, 1.179, 1.156, 1.118, 1.087, 1.082, 1.114, 1.163, 1.189, 1.203, 1.204, 1.249, 1.435, 1.800, 1.677, 1.622, 1.629, 1.664, 1.720, 1.790, 1.853, 1.903, 1.957, 2.013, 2.044, 2.037, 2.011, 1.982, 1.949, 1.906, 1.852, 1.793, 1.741, 1.706, 1.691, 1.689, 1.691, 1.685, 1.669, 1.655, 1.668, 1.716, 1.768, 1.781, 1.739, 1.675, 1.644, 1.672, 1.742, 1.819, 1.881, 1.933, 1.986, 2.044];
const SPECIALAIRS_3_RATES: [f32; 58] = [1.064, 1.173, 1.290, 1.387, 1.440, 1.447, 1.384, 1.291, 1.225, 1.243, 1.303, 1.354, 1.352, 1.322, 1.290, 1.268, 1.267, 1.292, 1.308, 1.309, 1.309, 1.303, 1.279, 1.241, 1.219, 1.216, 1.226, 1.240, 1.247, 1.246, 1.239, 1.232, 1.224, 1.214, 1.201, 1.187, 1.175, 1.167, 1.163, 1.163, 1.163, 1.162, 1.158, 1.156, 1.161, 1.176, 1.191, 1.195, 1.185, 1.169, 1.161, 1.168, 1.186, 1.205, 1.221, 1.234, 1.247, 1.261];
const SPECIALHI_3_RATES: [f32; 37] = [1., 1., 1., 1., 1., 1., 1., 0.7666, 0.6045, 0.4739, 0.3871, 0.3594, 0.3909, 0.4434, 0.4653, 0.452, 0.4491, 0.4672, 0.4615, 0.423, 0.453, 0.624, 1., 2.257, 2.911, 3.041, 3.031, 3.042, 3.066, 3.143, 3.375, 3.806, 4.349, 4.824, 5.122, 5.359, 5.675];
const SPECIALLW_2_RATES: [f32; 91] = [1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.004, 1.003, 0.993, 0.962, 0.953, 0.951, 0.959, 1.012, 1.163, 1.426, 1.656, 1.830, 1.964, 2.064, 2.121, 2.129, 2.094, 2.028, 1.923, 1.764, 1.570, 1.416, 1.303, 1.221, 1.186, 1.191, 1.202, 1.205, 1.215, 1.242, 1.290, 1.356, 1.419, 1.460, 1.478, 1.479, 1.470, 1.462, 1.465, 1.480, 1.504, 1.523, 1.494, 1.370, 1.184, 1.046, 0.992, 0.983, 0.983, 0.983, 0.983, 0.983, 0.983, 0.983, 0.983, 0.983, 0.983, 0.983, 0.983, 0.983];
const SPECIALLW_3_RATES: [f32; 91] = [1.424, 1.660, 1.932, 2.206, 2.370, 2.318, 2.106, 1.985, 2.094, 1.906, 1.389, 1.076, 0.969, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.952, 0.972, 1.086, 1.363, 1.727, 1.893, 1.913, 1.986, 2.182, 2.299, 2.131, 1.869, 1.669, 1.574, 1.563, 1.581, 1.585, 1.567, 1.535, 1.494, 1.448, 1.399, 1.341, 1.268, 1.199, 1.167, 1.172, 1.181, 1.184, 1.194, 1.218, 1.262, 1.322, 1.379, 1.416, 1.433, 1.434, 1.426, 1.418, 1.421, 1.435, 1.457, 1.474, 1.447, 1.335, 1.167, 1.042, 0.993, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984];

unsafe extern "C" fn game_specialairhi_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRHI_2_RATES, 4.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_2_RATES, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_2_RATES, 12.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_2_RATES, 20.0);
    agent.boma().rate_curve(agent, &SPECIALAIRHI_2_RATES, 37.0);
}

unsafe extern "C" fn game_specialairhi_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 5.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 105, 130, 0, 7.0, 0.0, 6.0, 5.0, None, None, None, 1.333, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 80, 105, 140, 0, 6.0, 0.0, 6.0, 9.0, None, None, None, 1.333, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 19.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 70, 100, 0, 50, 7.0, 0.0, 9.0, 3.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 24.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRHI_3_RATES, 37.0);
}

unsafe extern "C" fn effect_specialairhi_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    wait(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, -0.4, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairhi_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 0, 1, 0, 0, 0, 1.05, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 1, 0, 0, 0, 0, 1.05, true);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_power"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1.45, true);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, -0.4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_superjump_fnish"), -1);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_fire"), false, true);
    }
}

unsafe extern "C" fn sound_specialairhi_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_h03"));
    }
}

unsafe extern "C" fn sound_specialairhi_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_h03"));
    }
}

unsafe extern "C" fn game_specialairlw_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 27.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 34.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 40.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, 1.35);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 42, 142, 0, 90, 15.167, 0.0, 8.182, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 51.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 75.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 91.0);
}

unsafe extern "C" fn game_specialairlw_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.333, 30, 65, 0, 60, 4.821, 0.0, 8.0, 10.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.333, 30, 65, 0, 60, 4.821, 0.0, 8.0, -10.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 1, Hash40::new("top"), 0.0, 40, 60, 20, 0, 8.571, 0.0, 8.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 15, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 85, 118, 0, 70, 10.292, 0.0, 12.5, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 42.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 75.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 91.0);
}

unsafe extern "C" fn effect_specialairlw_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, -1.5, 0, 0, 180, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.25);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_l"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 3.25, 0, 0, 0, 0, 0, 360, false);
    }
}

unsafe extern "C" fn effect_specialairlw_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.39, 0.71, 1.0);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, -1.5, 0, 0, 180, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.39, 0.71, 1.0);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_l"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_impact"), Hash40::new("mariod_cyclone_impact"), Hash40::new("top"), 3, 14.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_cyclone_impact"), -1);
    }
}

unsafe extern "C" fn sound_specialairlw_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_l01"));
    }
    wait(lua_state, 28.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_l02"));
        PLAY_SE(agent, Hash40::new("vc_mariod_009"));
    }
}

unsafe extern "C" fn sound_specialairlw_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_l01"));
    }
    wait(lua_state, 28.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_l02"));
        PLAY_SE(agent, Hash40::new("vc_mariod_009"));
    }
}

unsafe extern "C" fn game_specialairn_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 14.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, -1);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 43.0);
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 54.0);
}

unsafe extern "C" fn game_specialairn_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 14.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, -1);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 43.0);
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 54.0);
}

unsafe extern "C" fn effect_specialairn_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 8, 0, 0, 0, 0.69, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 8, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 0, 0.353);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 22.0);
    frame(lua_state, 27.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_specialairn_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_n01"));
    }
}

unsafe extern "C" fn game_specialairs_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, -1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_2_RATES, 6.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_2_RATES, 9.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_2_RATES, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.2, 361, 105, 0, 30, 4.9, 0.0, 12.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.2, 361, 105, 0, 30, 5.0, 0.0, 6.7, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_2_RATES, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_2_RATES, 17.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_2_RATES, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_2_RATES, 45.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, ArticleOperationTarget(0));
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_2_RATES, 45.0);
    agent.boma().rate_curve(agent, &SPECIALAIRS_2_RATES, 58.0);
}

unsafe extern "C" fn game_specialairs_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, -1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 6.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 9.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 160, 100, 80, 0, 6.0, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 160, 100, 80, 0, 4.0, 0.0, 6.7, 5.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 115, 0, 60, 8.0, 0.0, 6.7, 10.0, Some(0.0), Some(6.7), Some(3.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 1, Hash40::new("top"), 0.0, 361, 115, 0, 36, 12.0, 0.0, 6.7, 21.0, Some(0.0), Some(6.7), Some(4.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 23.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 45.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, ArticleOperationTarget(0));
    }
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 45.0);
    agent.boma().rate_curve(agent, &SPECIALAIRS_3_RATES, 58.0);
}

unsafe extern "C" fn effect_specialairs_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    } else {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_supermant_wind_r"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_supermant_wind_l"), -1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 9, 10, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 14, 14, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 9, 10, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 14, 14, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 9, 10, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 14, 14, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
}

unsafe extern "C" fn effect_specialairs_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        }
    } else {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_supermant_wind_r"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_supermant_wind_l"), -1);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 16, 0, 35, 0, 1.5, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 16, 0, -35, 0, 1.5, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
}

unsafe extern "C" fn sound_specialairs_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_elec_l_damage"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_s01"));
    }
}

unsafe extern "C" fn sound_specialairs_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_s01"));
    }
}

unsafe extern "C" fn game_specialhi_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 5.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 105, 130, 0, 7.0, 0.0, 6.0, 5.0, None, None, None, 1.333, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 80, 105, 140, 0, 6.0, 0.0, 6.0, 9.0, None, None, None, 1.333, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 19.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 70, 100, 0, 50, 7.0, 0.0, 9.0, 3.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 24.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 37.0);
}

unsafe extern "C" fn effect_specialhi_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, -0.4, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
}

unsafe extern "C" fn effect_specialhi_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 0, 1, 0, 0, 0, 1.05, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 1, 0, 0, 0, 0, 1.05, true);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_power"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1.45, true);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_superjump_fnish"), -1);
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
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_fire"), false, true);
    }
}

unsafe extern "C" fn game_speciallw_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 2.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 10.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 27.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.714, 0.0, 8.5, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 34.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 39.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 40.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        SET_SPEED_EX(agent, 0.0, 2.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 42, 142, 0, 90, 15.167, 0.0, 8.182, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 51.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 75.0);
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 91.0);
}

unsafe extern "C" fn game_speciallw_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 2.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 10.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.333, 30, 65, 0, 60, 4.821, 0.0, 8.0, 10.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.333, 30, 65, 0, 60, 4.821, 0.0, 8.0, -10.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 1, Hash40::new("top"), 0.0, 40, 60, 20, 0, 8.571, 0.0, 8.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 15, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 85, 118, 0, 70, 10.292, 0.0, 12.5, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 42.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 75.0);
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 91.0);
}

unsafe extern "C" fn effect_speciallw_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, 1, 0, 0, 90, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.25);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_l"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 3.7, 0, 0, 0, 0, 0, 360, false);
    }
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_speciallw_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.39, 0.71, 1.0);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, 1, 0, 0, 180, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.39, 0.71, 1.0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_l"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_impact"), Hash40::new("mariod_cyclone_impact"), Hash40::new("top"), 3, 14.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_cyclone_impact"), -1);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn effect_specialn_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.92, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 0, 0.353);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 22.0);
    frame(lua_state, 27.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn effect_specials_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        }
    } else {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 16, 0, 35, 0, 1.5, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 16, 0, -35, 0, 1.5, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 6, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}


unsafe extern "C" fn effect_specials_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    } else {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 9, 10, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 14, 14, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 9, 10, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 14, 14, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 9, 10, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 14, 14, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 360, true);
    }
}

pub fn install(agent: &mut Agent) {
    config::custom_acmd!(agent, Special::Up, "game_specialairhi", 2 => game_specialairhi_c2, 3 => game_specialairhi_c3);
    config::custom_acmd!(agent, Special::Up, "effect_specialairhi", 2 => effect_specialairhi_c2, 3 => effect_specialairhi_c3);
    config::custom_acmd!(agent, Special::Up, "sound_specialairhi", 2 => sound_specialairhi_c2, 3 => sound_specialairhi_c3);
    config::custom_acmd!(agent, Special::Down, "game_specialairlw", 2 => game_specialairlw_c2, 3 => game_specialairlw_c3);
    config::custom_acmd!(agent, Special::Down, "effect_specialairlw", 2 => effect_specialairlw_c2, 3 => effect_specialairlw_c3);
    config::custom_acmd!(agent, Special::Down, "sound_specialairlw", 2 => sound_specialairlw_c2, 3 => sound_specialairlw_c3);
    config::custom_acmd!(agent, Special::Neutral, "game_specialairn", 2 => game_specialairn_c2, 3 => game_specialairn_c3);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairn", 3 => effect_specialairn_c3);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialairn", 2 => sound_specialairn_c2, 3 => sound_specialairn_c2);
    config::custom_acmd!(agent, Special::Side, "game_specialairs", 2 => game_specialairs_c2, 3 => game_specialairs_c3);
    config::custom_acmd!(agent, Special::Side, "effect_specialairs", 2 => effect_specialairs_c2, 3 => effect_specialairs_c3);
    config::custom_acmd!(agent, Special::Side, "sound_specialairs", 2 => sound_specialairs_c2, 3 => sound_specialairs_c3);
    config::custom_acmd!(agent, Special::Up, "game_specialhi", 2 => game_specialairhi_c2, 3 => game_specialhi_c3);
    config::custom_acmd!(agent, Special::Up, "effect_specialhi", 2 => effect_specialhi_c2, 3 => effect_specialhi_c3);
    config::custom_acmd!(agent, Special::Up, "sound_specialhi", 2 => sound_specialairhi_c2, 3 => sound_specialairhi_c3);
    config::custom_acmd!(agent, Special::Down, "game_speciallw", 2 => game_speciallw_c2, 3 => game_speciallw_c3);
    config::custom_acmd!(agent, Special::Down, "effect_speciallw", 2 => effect_speciallw_c2, 3 => effect_speciallw_c3);
    config::custom_acmd!(agent, Special::Down, "sound_speciallw", 2 => sound_specialairlw_c2, 3 => sound_specialairlw_c3);
    config::custom_acmd!(agent, Special::Neutral, "game_specialn", 2 => game_specialairn_c2, 3 => game_specialairn_c3);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialn", 3 => effect_specialn_c3);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialn", 2 => sound_specialairn_c2, 3 => sound_specialairn_c2);
    config::custom_acmd!(agent, Special::Side, "game_specials", 2 => game_specialairs_c2, 3 => game_specialairs_c3);
    config::custom_acmd!(agent, Special::Side, "effect_specials", 2 => effect_specials_c2, 3 => effect_specials_c3);
    config::custom_acmd!(agent, Special::Side, "sound_specials", 2 => sound_specialairs_c2, 3 => sound_specialairs_c3);
}

