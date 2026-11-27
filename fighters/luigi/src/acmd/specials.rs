use super::*;
use config::Special;

const SPECIALHI_2_RATES: [f32; 23] = [0.859, 0.859, 0.859, 0.859, 0.859, 0.886, 1.084, 1.735, 3.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000];
const SPECIALSEND_3_RATES: [f32; 51] = [1.106, 1.150, 1.208, 1.270, 1.307, 1.302, 1.279, 1.269, 1.280, 1.311, 1.374, 1.486, 1.655, 1.879, 2.119, 2.306, 2.404, 2.440, 2.451, 2.452, 2.434, 2.361, 2.207, 2.004, 1.833, 1.743, 1.712, 1.684, 1.632, 1.564, 1.504, 1.471, 1.467, 1.476, 1.478, 1.467, 1.448, 1.424, 1.395, 1.362, 1.303, 1.183, 1.013, 0.889, 0.826, 0.795, 0.769, 0.747, 0.732, 0.720, 0.709];
const SPECIALAIRSEND_3_RATES: [f32; 30] = [1.539, 1.547, 1.560, 1.567, 1.558, 1.528, 1.488, 1.450, 1.415, 1.378, 1.333, 1.279, 1.223, 1.184, 1.170, 1.176, 1.185, 1.187, 1.183, 1.177, 1.169, 1.155, 1.117, 1.032, 0.910, 0.825, 0.786, 0.768, 0.758, 0.753];
const SPECIALN_2_RATES: [f32; 54] = [0.978, 0.978, 0.978, 0.978, 0.978, 0.978, 0.978, 0.978, 0.978, 0.978, 0.978, 0.978, 0.978, 0.978, 0.988, 1.057, 1.263, 1.623, 1.962, 2.167, 2.246, 2.272, 2.288, 2.299, 2.304, 2.304, 2.297, 2.280, 2.248, 2.177, 2.024, 1.784, 1.530, 1.358, 1.290, 1.279, 1.284, 1.292, 1.301, 1.314, 1.335, 1.374, 1.439, 1.529, 1.607, 1.658, 1.669, 1.664, 1.665, 1.685, 1.738, 1.857, 2.076, 2.381];
const SPECIALAIRN_2_RATES: [f32; 49] = [0.968, 0.968, 0.968, 0.968, 0.968, 0.968, 0.968, 0.968, 0.968, 0.968, 0.968, 0.968, 0.968, 0.968, 0.986, 1.093, 1.369, 1.753, 2.021, 2.118, 2.136, 2.152, 2.181, 2.219, 2.260, 2.291, 2.291, 2.241, 2.136, 1.988, 1.825, 1.667, 1.528, 1.421, 1.358, 1.338, 1.352, 1.382, 1.407, 1.422, 1.446, 1.498, 1.563, 1.606, 1.616, 1.655, 1.743, 1.893, 2.093];
const SPECIALN_3_RATES: [f32; 54] = [1.203, 1.234, 1.238, 1.239, 1.277, 1.353, 1.455, 1.591, 1.749, 1.859, 1.847, 1.731, 1.571, 1.435, 1.441, 1.604, 1.673, 1.458, 1.155, 1.003, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.974, 0.975, 0.981, 0.994, 1.000, 0.999, 1.001, 1.001, 1.001, 1.001, 1.001, 1.001, 1.001];
const SPECIALAIRN_3_RATES: [f32; 49] = [1.184, 1.205, 1.252, 1.325, 1.394, 1.434, 1.471, 1.547, 1.661, 1.754, 1.769, 1.703, 1.589, 1.474, 1.472, 1.609, 1.657, 1.440, 1.149, 1.003, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.975, 0.976, 0.982, 0.995, 1.001, 1.000, 1.002, 1.002];
const SPECIALLW_2_RATES: [f32; 91] = [1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.007, 1.008, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.003, 0.958, 0.834, 0.639, 0.561, 0.593, 0.607, 0.610, 0.590, 0.591, 0.609, 0.553, 0.647, 1.000, 1.622, 1.991, 2.127, 2.148, 2.112, 2.026, 2.014, 2.040, 2.061, 2.064, 2.058, 2.056, 2.041, 1.945, 1.695, 1.333, 1.096, 0.999, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.988, 1.000];
const SPECIALAIRLW_2_RATES: [f32; 91] = [1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 0.999, 1.001, 1.008, 1.007, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.003, 0.958, 0.834, 0.639, 0.561, 0.593, 0.607, 0.609, 0.591, 0.592, 0.608, 0.552, 0.648, 1.000, 1.574, 1.953, 2.135, 2.187, 2.151, 2.052, 2.022, 2.038, 2.056, 2.059, 2.053, 2.051, 2.037, 1.941, 1.691, 1.333, 1.096, 0.999, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.984, 0.988, 1.000];
const SPECIALLW_3_RATES: [f32; 91] = [1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.007, 1.008, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.003, 0.958, 0.834, 0.639, 0.561, 0.593, 0.607, 0.610, 0.590, 0.591, 0.609, 0.553, 0.647, 1.000, 1.420, 1.679, 1.780, 1.808, 1.824, 1.848, 1.886, 1.926, 1.947, 1.949, 1.944, 1.942, 1.944, 1.948, 1.955, 1.964, 1.974, 1.978, 1.948, 1.821, 1.593, 1.364, 1.260, 1.296, 1.400, 1.512, 1.624, 1.739, 1.836, 1.889, 1.890, 1.857, 1.811, 1.753, 1.671, 1.569, 1.474, 1.413, 1.387, 1.386, 1.395, 1.403, 1.401, 1.361, 1.231, 1.000];
const SPECIALAIRLW_3_RATES: [f32; 91] = [1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 1.000, 0.999, 1.001, 1.008, 1.007, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.010, 1.003, 0.958, 0.834, 0.639, 0.561, 0.593, 0.607, 0.609, 0.591, 0.592, 0.608, 0.552, 0.648, 1.000, 1.376, 1.635, 1.765, 1.820, 1.848, 1.876, 1.914, 1.952, 1.972, 1.973, 1.969, 1.967, 1.969, 1.972, 1.976, 1.976, 1.976, 1.966, 1.919, 1.792, 1.582, 1.375, 1.272, 1.288, 1.371, 1.478, 1.597, 1.717, 1.823, 1.895, 1.915, 1.878, 1.802, 1.711, 1.623, 1.545, 1.482, 1.437, 1.413, 1.409, 1.415, 1.416, 1.398, 1.339, 1.206, 1.000];

unsafe extern "C" fn game_specialhi_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 70, 88, 0, 50, 4.0, 1.2, 5.5, 7.0, Some(-1.2), Some(5.5), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 11.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_2_RATES, 23.0);
}

unsafe extern "C" fn game_specialhi_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 4.531, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 3.766, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhi_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 75, 85, 0, 55, 4.0, 0.0, 5.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhi_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 4.531, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 3.766, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhi2sub_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhi2sub_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 4.531, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 3.766, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhi_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 0.53, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_superjump_power"), Hash40::new("handl"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_superjump_critical"), Hash40::new("handl"), 1, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn effect_specialhi_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 0.53, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_superjump_power"), Hash40::new("handl"), 1, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn effect_specialairhi_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 2.008, 0.698, 0.698);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_superjump_power"), Hash40::new("handl"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.298, 1.0, 0.298);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_superjump_critical"), Hash40::new("handl"), 1, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn effect_specialairhi_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 0.53, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_superjump_power"), Hash40::new("handl"), 1, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn game_specialhilandingfall_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhilandingfall_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 25, 0, 25, 9.0, 0.0, 6.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specials_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 30, 80, 0, 30, 4.8, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 30, 80, 0, 30, 4.8, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

unsafe extern "C" fn game_specials_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 4.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 4.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

unsafe extern "C" fn game_specialsdischarge_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 23.0, 30, 100, 0, 20, 5.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 23.0, 30, 100, 0, 20, 5.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

unsafe extern "C" fn game_specialsdischarge_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 20.0, 361, 90, 0, 20, 5.5, 4.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 20.0, 361, 90, 0, 20, 5.5, 4.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

unsafe extern "C" fn game_specialsend_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALSEND_3_RATES, 42.0);
    agent.boma().rate_curve(agent, &SPECIALSEND_3_RATES, 51.0);
}

unsafe extern "C" fn game_specialairsend_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRSEND_3_RATES, 24.0);
    agent.boma().rate_curve(agent, &SPECIALAIRSEND_3_RATES, 30.0);
}

unsafe extern "C" fn effect_specialshold_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.471, 0.471, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    loop {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 15.0);
    }
}

unsafe extern "C" fn effect_specialshold_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.471, 0.471);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    loop {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 10.0);
    }
}

unsafe extern "C" fn effect_specialairshold_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.471, 0.471, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

unsafe extern "C" fn effect_specialairshold_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.098, 0.471, 0.471);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn game_specialn_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALN_2_RATES, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    agent.boma().rate_curve(agent, &SPECIALN_2_RATES, 44.0);
    agent.boma().rate_curve(agent, &SPECIALN_2_RATES, 54.0);
}

unsafe extern "C" fn game_specialairn_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 44.0);
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 49.0);
}

unsafe extern "C" fn game_specialn_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 44.0);
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 54.0);
}

unsafe extern "C" fn game_specialairn_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 44.0);
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 49.0);
}

unsafe extern "C" fn effect_specialn_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 1, 0.353);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn effect_specialairn_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 17.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 1, 0.353);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn game_speciallw_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 4.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 9.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 11.0, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 36.0);
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 38.0);
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 40.0);
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 42.0);
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 43.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 44.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        ADD_SPEED_NO_LIMIT(agent, 0.0, 2.5);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 48, 129, 0, 96, 15.2, 0.0, 9.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 45.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 50.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 60.0);
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 90.0);
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 91.0);
}

unsafe extern "C" fn game_specialairlw_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 8.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 10.0, 0.0, 8.5, 0.0, None, None, None, 0.615, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 36.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 38.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 40.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 42.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 44.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, 1.5);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 48, 130, 0, 96, 15.2, 0.0, 9.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 45.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 50.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 60.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 90.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 91.0);
}

unsafe extern "C" fn game_speciallw_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 4.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 9.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.3, 30, 65, 0, 60, 4.95, 0.0, 8.0, 11.0, None, None, None, 1.6, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.3, 30, 65, 0, 60, 4.95, 0.0, 8.0, -11.0, None, None, None, 1.6, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 40, 60, 0, 20, 8.8, 0.0, 8.0, 0.0, None, None, None, 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 15, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 36.0);
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 38.0);
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 40.0);
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 42.0);
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 44.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 90, 108, 0, 74, 3.75, 0.0, 12.0, -10.0, Some(0.0), Some(12.0), Some(10.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 90, 108, 0, 74, 8.67, 0.0, 10.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 45.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 90.0);
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 91.0);
}

unsafe extern "C" fn game_specialairlw_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 8.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.3, 30, 65, 0, 60, 4.5, 0.0, 7.5, 11.0, None, None, None, 1.23, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.3, 30, 65, 0, 60, 4.5, 0.0, 7.5, -11.0, None, None, None, 1.23, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 40, 60, 0, 20, 8.0, 0.0, 8.0, 0.0, None, None, None, 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 15, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 36.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 38.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 40.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 42.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 44.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 90, 108, 0, 74, 3.75, 0.0, 12.0, -10.0, Some(0.0), Some(12.0), Some(10.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 90, 108, 0, 74, 8.67, 0.0, 10.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 45.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 90.0);
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 91.0);
}

unsafe extern "C" fn effect_speciallw_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_cyclone_tornado_r"), Hash40::new("luigi_cyclone_tornado_l"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 1, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.05);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_l"), false, true);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 3.4, 0, 0, 0, 0, 0, 360, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

unsafe extern "C" fn effect_specialairlw_c2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_cyclone_tornado_r"), Hash40::new("luigi_cyclone_tornado_l"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.05);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_l"), false, true);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 3.4, 0, 0, 0, 0, 0, 360, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

unsafe extern "C" fn effect_speciallw_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.5, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_COLOR(agent, 0.392, 0.706, 1.0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.5, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_COLOR(agent, 0.392, 0.706, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_cyclone_tornado_r"), Hash40::new("luigi_cyclone_tornado_l"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 0.95);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_l"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, true);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("luigi_cyclone_finish"), Hash40::new("top"), 0, 7, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairlw_c3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.5, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_COLOR(agent, 0.392, 0.706, 1.0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.5, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_COLOR(agent, 0.392, 0.706, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_cyclone_tornado_r"), Hash40::new("luigi_cyclone_tornado_l"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 0.95);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_l"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, true);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("luigi_cyclone_finish"), Hash40::new("top"), 0, 7, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install(agent: &mut Agent) {
    config::custom_acmd!(agent, Special::Neutral, "game_specialn", 2 => game_specialn_c2, 3 => game_specialn_c3);
    config::custom_acmd!(agent, Special::Neutral, "game_specialairn", 2 => game_specialairn_c2, 3 => game_specialairn_c3);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialn", 3 => effect_specialn_c3);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairn", 3 => effect_specialairn_c3);

    config::custom_acmd!(agent, Special::Side, "game_specials", 2 => game_specials_c2, 3 => game_specials_c3);
    config::custom_acmd!(agent, Special::Side, "game_specialsdischarge", 2 => game_specialsdischarge_c2, 3 => game_specialsdischarge_c3);
    config::custom_acmd!(agent, Special::Side, "game_specialsend", 3 => game_specialsend_c3);
    config::custom_acmd!(agent, Special::Side, "game_specialairsend", 3 => game_specialairsend_c3);
    config::custom_acmd!(agent, Special::Side, "effect_specialshold", 2 => effect_specialshold_c2, 3 => effect_specialshold_c3);
    config::custom_acmd!(agent, Special::Side, "effect_specialairshold", 2 => effect_specialairshold_c2, 3 => effect_specialairshold_c3);

    config::custom_acmd!(agent, Special::Up, "game_specialhi", 2 => game_specialhi_c2, 3 => game_specialhi_c3);
    config::custom_acmd!(agent, Special::Up, "game_specialairhi", 2 => game_specialairhi_c2, 3 => game_specialairhi_c3);
    config::custom_acmd!(agent, Special::Up, "game_specialhi2sub", 2 => game_specialhi2sub_c2, 3 => game_specialhi2sub_c3);
    config::custom_acmd!(agent, Special::Up, "effect_specialhi", 2 => effect_specialhi_c2, 3 => effect_specialhi_c3);
    config::custom_acmd!(agent, Special::Up, "effect_specialairhi", 2 => effect_specialairhi_c2, 3 => effect_specialairhi_c3);
    config::custom_acmd!(agent, Special::Up, "game_specialhilandingfall", 2 => game_specialhilandingfall_c2, 3 => game_specialhilandingfall_c3, _ => acmd_stub);

    config::custom_acmd!(agent, Special::Down, "game_speciallw", 2 => game_speciallw_c2, 3 => game_speciallw_c3);
    config::custom_acmd!(agent, Special::Down, "game_specialairlw", 2 => game_specialairlw_c2, 3 => game_specialairlw_c3);
    config::custom_acmd!(agent, Special::Down, "effect_speciallw", 2 => effect_speciallw_c2, 3 => effect_speciallw_c3);
    config::custom_acmd!(agent, Special::Down, "effect_specialairlw", 2 => effect_specialairlw_c2, 3 => effect_specialairlw_c3);
}
