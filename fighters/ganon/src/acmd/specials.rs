use super::*;
use config::Special;

unsafe fn draw_sword(boma: &mut BattleObjectModuleAccessor) {
    ArticleModule::generate_article(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    if !ArticleModule::is_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD) {
        return;
    }
    let article = ArticleModule::get_article(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD);
    let sword = sv_battle_object::module_accessor(lua_bind::Article::get_battle_object_id(article) as u32);
    let flags = *CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT;
    LinkModule::set_model_constraint_pos_ort(sword, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("havel"), flags as u32, true);
    LinkModule::set_constraint_translate_offset(sword, &Vector3f { x: 0.0, y: 0.0, z: 0.0 });
    LinkModule::set_constraint_rot_offset(sword, &Vector3f { x: 0.0, y: 0.0, z: 0.0 });
}

unsafe extern "C" fn game_specialn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        draw_sword(boma);
    }
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 51.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 61.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 66.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 20, 100, 30, 0, 6.0, 0.0, 8.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ADD_SPEED_NO_LIMIT(agent, 0.8, 0.0);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 68.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 40, 44, 0, 95, 3.5, 0.0, 10.5, 4.25, Some(0.0), Some(10.5), Some(25.25), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 22.0, 40, 55, 0, 100, 3.907, 0.0, 10.5, 32.25, Some(0.0), Some(10.5), Some(30.25), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 74.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 15.0);
        ATK_POWER(agent, 1, 19.0);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 84.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 104.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialnturn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        draw_sword(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 51.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 61.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 63.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 20, 100, 30, 0, 6.0, 0.0, 8.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 66.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.8, 0.0);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 68.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 35, 44, 0, 95, 3.5, 0.0, 10.5, -2.84, Some(0.0), Some(10.5), Some(-25.84), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 35, 55, 0, 100, 3.907, 0.0, 10.5, -32.84, Some(0.0), Some(10.5), Some(-30.84), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 74.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 17.0);
        ATK_POWER(agent, 1, 21.0);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 84.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 104.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialairn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        draw_sword(boma);
    }
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN);
    }
    frame(lua_state, 51.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 61.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 63.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 20, 100, 30, 0, 6.0, 0.0, 8.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 66.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(boma, 1, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE);
        ADD_SPEED_NO_LIMIT(agent, 0.8, 0.0);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 69.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 30, 40, 0, 95, 3.5, 0.0, 13.0, 1.5, Some(0.0), Some(13.0), Some(22.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 30, 50, 0, 100, 3.907, 0.0, 13.0, 29.5, Some(0.0), Some(13.0), Some(27.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 74.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 17.0);
        ATK_POWER(agent, 1, 21.0);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::set_int(boma, 2, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE);
    }
    frame(lua_state, 84.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 104.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialairnturn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        draw_sword(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 51.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 61.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 63.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 20, 100, 30, 0, 6.0, 0.0, 8.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 66.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(boma, 1, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE);
        ADD_SPEED_NO_LIMIT(agent, 0.8, 0.0);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 69.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 30, 40, 0, 95, 3.5, 0.0, 12.5, -2.0, Some(0.0), Some(12.5), Some(-23.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 26.0, 30, 50, 0, 100, 3.907, 0.0, 12.5, -30.0, Some(0.0), Some(12.5), Some(-28.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 74.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 19.0);
        ATK_POWER(agent, 1, 23.0);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::set_int(boma, 2, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE);
    }
    frame(lua_state, 84.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 104.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN);
    }
    FT_MOTION_RATE(agent, 0.4);
    frame(lua_state, 61.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 70.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 16.0, 361, 22, 0, 115, 7.256, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 9.0, 70, 33, 0, 80, 17.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 74.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 76.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 90.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 115.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialnturn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    FT_MOTION_RATE(agent, 0.4);
    frame(lua_state, 61.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 70.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 19.0, 361, 22, 0, 115, 6.14, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 12.0, 60, 33, 0, 75, 17.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 74.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 76.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 90.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 115.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN);
    }
    FT_MOTION_RATE(agent, 0.4);
    frame(lua_state, 61.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 66.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(boma, 1, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 16.0, 361, 20, 0, 115, 7.256, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 9.0, 70, 30, 0, 80, 17.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 74.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::set_int(boma, 2, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE);
    }
    frame(lua_state, 76.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 90.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 115.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairnturn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    FT_MOTION_RATE(agent, 0.4);
    frame(lua_state, 61.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 66.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(boma, 1, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 19.0, 361, 20, 0, 115, 6.14, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 12.0, 60, 30, 0, 75, 17.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 74.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::set_int(boma, 2, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE);
    }
    frame(lua_state, 76.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 90.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 115.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"), false, true);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_majinken_flash"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 10.5, 29, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 10.5, 27.5, 0, -10, 0, 0.5, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, true);
    }
}

unsafe extern "C" fn effect_specialn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        EffectModule::enable_sync_init_pos_last(boma);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"), false, true);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 12.5, 22.5, 0, -10, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

unsafe extern "C" fn sound_specialn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_n01"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n01"));
    }
    wait(lua_state, 48.0);
    wait(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

unsafe extern "C" fn sound_specialn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_n01"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n01"));
    }
    wait(lua_state, 48.0);
    wait(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

unsafe extern "C" fn expression_specialn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, 0x50000000);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2.0, 75.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 50);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0x50000000);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_23_hold"), 0, true, 0x50000000);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0x50000000);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, 0x50000000);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3.0, 120.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 0);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 101.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn expression_specialn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, false, 0x50000000);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2.0, 75.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 50);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, 0x50000000);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3.0, 120.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 0);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 101.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn effect_specialnturn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"), false, true);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_majinken_flash"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 10.5, -24, 0, 180, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 10.5, -23.5, 0, 170, 0, 0.5, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, true);
    }
}

unsafe extern "C" fn effect_specialnturn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"), false, true);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 12.5, -22.5, 0, 170, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

unsafe extern "C" fn sound_specialnturn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_n01"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_n01"));
    }
    wait(lua_state, 48.0);
    wait(lua_state, 20.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_n02"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

unsafe extern "C" fn sound_specialnturn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_n01"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_n01"));
    }
    wait(lua_state, 48.0);
    wait(lua_state, 20.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_n02"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

unsafe extern "C" fn expression_specialnturn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2.0, 75.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 50);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, 0x50000000);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0x50000000);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_23_hold"), 0, true, 0x50000000);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0x50000000);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, 0x50000000);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3.0, 120.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 0);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 101.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn expression_specialnturn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2.0, 75.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 50);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, false, 0x50000000);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, 0x50000000);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3.0, 120.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 0);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 101.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn effect_specialairn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_majinken_flash"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 13, 24, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 12.5, 22.5, 0, -10, 0, 0.5, true);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, true);
    }
}

unsafe extern "C" fn effect_specialairn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 14, 22.5, 0, -10, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}

unsafe extern "C" fn expression_specialairn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, 0x50000000);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2.0, 75.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 50);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0x50000000);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_23_hold"), 0, true, 0x50000000);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0x50000000);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, 0x50000000);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3.0, 120.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 0);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 101.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn expression_specialairn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, false, 0x50000000);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2.0, 75.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 50);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, 0x50000000);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3.0, 120.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 0);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 101.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn effect_specialairnturn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_majinken_flash"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 12.5, -23, 0, 180, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 12.5, -22.5, 0, 170, 0, 0.5, true);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, true);
    }
}

unsafe extern "C" fn effect_specialairnturn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 14, -22.5, 0, 170, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}

unsafe extern "C" fn expression_specialairnturn_blade(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2.0, 75.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 50);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, 0x50000000);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0x50000000);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, 0x50000000);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3.0, 120.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 0);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 101.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn expression_specialairnturn_thrust(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2.0, 75.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 50);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, false, 0x50000000);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, 0x50000000);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3.0, 120.0, 2.0, 1.0, 0.0, 12.0, 50.0, 30.0, 0);
        sv_animcmd::AREA_WIND_2ND(lua_state);
    }
    frame(lua_state, 101.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn game_specialsstart_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 6.8, 6.7, 7.5, 3.8);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -0.8, 0.0);
        CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 8.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_G);
        CATCH(agent, 1, Hash40::new("top"), 1.0, 0.0, 8.0, 7.2, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.4, 0.0);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.4, 0.0);
    }
    frame(lua_state, 37.0);
    FT_MOTION_RATE(agent, 1.7);
    frame(lua_state, 42.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 7.0, 7.0, 7.5, 7.5);
    }
    frame(lua_state, 57.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairsstart_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.6);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -0.8, 0.0);
        CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_G);
        CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 11.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.4, 0.0);
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.4, 0.0);
    }
    frame(lua_state, 37.0);
    FT_MOTION_RATE(agent, 1.7);
    frame(lua_state, 57.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specials_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 361, 90, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        let target_object = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_hit_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_hit_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target_object, target_hit_group, target_hit_no);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_SET_FALL);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("havel"), 8.0, 70, 85, 0, 100, 7.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 65, 0, 100, 22.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairs_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 361, 82, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        let target_object = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_hit_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_hit_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target_object, target_hit_group, target_hit_no);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 90, 0, 95, 7.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 70, 65, 0, 100, 24.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialsstart_chain(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 30, 100, 20, 0, 6.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 6.8, 6.7, 7.5, 3.8);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 368, 50, 20, 0, 8.0, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(5.0), 0.8, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 368, 100, 30, 0, 8.0, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(5.0), 0.8, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f { x: 23.0, y: 9.0 }, 3, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f { x: 23.0, y: 9.0 }, 3, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 2.8, 0.0);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -1.5, 0.0);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f { x: 17.5, y: 9.0 }, 3, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f { x: 17.5, y: 9.0 }, 3, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -0.8, 0.0);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f { x: 14.0, y: 9.0 }, 3, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f { x: 14.0, y: 9.0 }, 3, false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 30, 90, 0, 60, 9.0, 0.0, 10.0, 9.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    FT_MOTION_RATE(agent, 1.8);
    frame(lua_state, 42.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 7.0, 7.0, 7.5, 7.5);
    }
    frame(lua_state, 54.0);
    FT_MOTION_RATE(agent, 1.5);
}

unsafe extern "C" fn game_specialairsstart_chain(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.2);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 2.5, 0.0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 368, 50, 20, 0, 9.0, 0.0, 9.5, 10.0, Some(0.0), Some(9.5), Some(5.0), 0.8, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 368, 100, 30, 0, 9.0, 0.0, 9.5, 10.0, Some(0.0), Some(9.5), Some(5.0), 0.8, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f { x: 23.0, y: 10.0 }, 3, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f { x: 23.0, y: 10.0 }, 3, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -1.5, 0.0);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f { x: 17.5, y: 10.0 }, 3, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f { x: 17.5, y: 10.0 }, 3, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -0.8, 0.0);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f { x: 14.0, y: 10.0 }, 3, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f { x: 14.0, y: 10.0 }, 3, false);
    }
    frame(lua_state, 29.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 30, 90, 0, 60, 9.0, 0.0, 11.0, 9.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialsstart_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..7 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 4.0);
    }
}

unsafe extern "C" fn effect_specialairsstart_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), 0, 0, 0, -0.28609, -45, 25, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specials_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("sys_catch"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_catch"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ganon_attack_purple"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn effect_specialairs_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ganon_attack_purple"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_engokua"), true, true);
        EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn effect_specialsstart_chain(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1.1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1.1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 12, 27, 0, 0, 0, 1.25, true);
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 12, 27, 0, 0, 0, 1.25, true);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairsstart_chain(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), 0, 0, 0, -0.28609, -45, 25, 1.1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), 0, 0, 0, -0.28609, -45, 25, 1.1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 12, 27, 0, 0, 0, 1.25, true);
        EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 12, 27, 0, 0, 0, 1.25, true);
    }
}

unsafe extern "C" fn sound_specialsstart_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_s01"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_s02"));
    }
    wait(lua_state, 32.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_right_m"));
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
}

unsafe extern "C" fn sound_specialsstart_chain(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_s01"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_s02"));
    }
    wait(lua_state, 32.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_right_m"));
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
}

unsafe extern "C" fn sound_specialairsstart_wave(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_s01"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_s02"));
    }
}

unsafe extern "C" fn sound_specialairsstart_chain(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_s01"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_s02"));
    }
}

unsafe extern "C" fn game_specialhi_fists(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, -0.08);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 3.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 2.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 81, 100, 160, 0, 5.7, 0.0, 6.0, 10.5, None, None, None, 0.9, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 81, 100, 160, 0, 5.5, 0.0, 12.5, 9.5, None, None, None, 0.9, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 26.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 34.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 11.0, 80, 90, 0, 80, 9.5, 6.0, -1.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 11.0, 80, 90, 0, 80, 7.7, -3.5, -1.5, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
}

unsafe extern "C" fn game_specialhi_vault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.4, 0.0, 16.0, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 6.0, 0.0, 8.0, 11.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, -1.0);
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE(agent, 3.0);
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, 3.0);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 34.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 15.5, 6.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 8.5, 3.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, -0.5);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, -0.5);
    }
}

unsafe extern "C" fn game_specialhicatch_vault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.583, 361, 100, 0, 50, 7.0, 0.0, 9.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhithrow_vault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 40, 96, 0, 88, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        let target_object = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_hit_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_hit_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target_object, target_hit_group, target_hit_no);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, 0.4);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_FALL);
    }
}

unsafe extern "C" fn effect_specialhi_fists(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 5, 8, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("haver"), 0, 0, 0, 0, -45, 25, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_engokua_flash"), false, true);
    }
}

unsafe extern "C" fn effect_specialhi_vault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_raijin_hold"), Hash40::new("armr"), 2, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 14.0);
    for _ in 0..6 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ganon_attack_elec"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(lua_state, 3.0);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 33.0);
    for _ in 0..6 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ganon_attack_elec"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(lua_state, 3.0);
    }
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_raijin_hold"), false, true);
    }
}

unsafe extern "C" fn sound_specialhi_fists(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h05"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_h01"));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_h05"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_h01"));
    }
    wait(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h02"));
    }
}

unsafe extern "C" fn sound_specialhi_vault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h05"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_h01"));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_h05"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_h01"));
    }
    wait(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h02"));
    }
}

unsafe extern "C" fn expression_specialhi_fists(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, 0x50000000);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, 0x50000000);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn expression_specialairhi_fists(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, 0x50000000);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, 0x50000000);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn game_speciallw_dropkick(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, 2.1);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 3.0, 6.0, 8.5, 9.5);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        ADD_SPEED_NO_LIMIT(agent, 0.0, -1.6);
    }
    FT_MOTION_RATE(agent, 5.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 2.0, 6.0, 8.5, 10.0);
        ADD_SPEED_NO_LIMIT(agent, 2.0, -0.4);
        let energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut app::FighterKineticEnergyMotion;
        lua_bind::FighterKineticEnergyMotion::set_angle_whole(energy, -30.0, 1);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -1.2, 0.0);
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 11.0, 35, 65, 0, 65, 3.333, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 12.0, 35, 65, 0, 65, 4.571, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 8.0, 8.0, 8.0, 4.0);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.8, -0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn game_specialairlw_dropkick(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        let energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut app::FighterKineticEnergyMotion;
        lua_bind::FighterKineticEnergyMotion::set_angle_whole(energy, 42.0, 1);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.3, 0.0);
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("legl"), 11.0, 30, 80, 0, 60, 4.569, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 11.0, 30, 80, 0, 60, 2.831, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 9.0, 45, 80, 0, 60, 4.569, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 9.0, 45, 80, 0, 60, 2.831, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -1.0, 0.0);
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn game_speciallw_assault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 4.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 3.0, 6.0, 8.5, 9.5);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 2.0, 6.0, 8.5, 10.0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 1.2, 0.0);
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 12.0, 130, 79, 0, 70, 3.0, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 14.667, 130, 79, 0, 70, 4.0, 9.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8.0);
        ATK_POWER(agent, 1, 9.333);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 35.0);
    if is_excute(agent) {
        lua_bind::FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 8.0, 8.0, 8.0, 4.0);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -1.2, 0.0);
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn game_specialairlw_assault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 3.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut app::FighterKineticEnergyMotion;
        lua_bind::FighterKineticEnergyMotion::set_angle_whole(energy, -17.0, 1);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("legl"), 14.0, 280, 100, 0, 55, 4.31, 12.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 14.0, 70, 100, 0, 55, 3.088, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, -1.0);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 12.0);
        ATK_POWER(agent, 1, 12.0);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 12.0, 70, 95, 0, 55, 4.31, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 12.0, 70, 95, 0, 55, 3.088, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, 1.0);
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn game_speciallwend_dropkick(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_CLIFF_CHECK);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    }
}

unsafe extern "C" fn game_speciallwend_assault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_CLIFF_CHECK);
        WorkModule::off_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    }
}

unsafe extern "C" fn game_specialairlwend_dropkick(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 40, 100, 0, 50, 5.0, 0.0, 3.5, 11.5, Some(0.0), Some(3.5), Some(-9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 1.5);
}

unsafe extern "C" fn game_specialairlwend_assault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 85, 90, 0, 60, 5.0, 0.0, 3.5, 11.5, Some(0.0), Some(3.5), Some(-9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    }
}

unsafe extern "C" fn game_specialairlwendair_dropkick(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, -0.5, 0.0);
    }
}

unsafe extern "C" fn effect_speciallw_dropkick(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15, 15, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 2, 0.5, 1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneer"), 12, -1.5, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_COLOR(agent, 0.196, 0.784, 0.98);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    loop {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 6.0);
    }
}

unsafe extern "C" fn effect_speciallw_assault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -6.5, 2.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 2, 0.5, 1);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneer"), 12, -1.5, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_COLOR(agent, 1.569, 0.588, 0.588);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    loop {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 6.0);
    }
}

unsafe extern "C" fn effect_specialairlw_dropkick(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 2, 0.5, 1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneel"), 12, -1.5, 0, 0, 0, 0, 0.85, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_COLOR(agent, 0.196, 0.784, 0.98);
    }
}

unsafe extern "C" fn effect_specialairlw_assault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 3, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 2, 0.5, 1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneel"), 12, -1.5, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_COLOR(agent, 1.569, 0.588, 0.588);
    }
}

unsafe extern "C" fn sound_speciallw_dropkick(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_dash_start"));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_ganon_special_l01"));
    }
}

unsafe extern "C" fn sound_speciallw_assault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_dash_start"));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_ganon_special_l01"));
    }
}

unsafe extern "C" fn sound_specialairlw_assault(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_dash_start"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_ganon_special_l02"));
    }
}

pub fn install(agent: &mut Agent) {
    config::custom_acmd!(agent, Special::Neutral, "game_specialn", 2 => game_specialn_blade, 3 => game_specialn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialn", 2 => effect_specialn_blade, 3 => effect_specialn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialn", 2 => sound_specialn_blade, 3 => sound_specialn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "expression_specialn", 2 => expression_specialn_blade, 3 => expression_specialn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "game_specialnturn", 2 => game_specialnturn_blade, 3 => game_specialnturn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialnturn", 2 => effect_specialnturn_blade, 3 => effect_specialnturn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialnturn", 2 => sound_specialnturn_blade, 3 => sound_specialnturn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "expression_specialnturn", 2 => expression_specialnturn_blade, 3 => expression_specialnturn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "game_specialairn", 2 => game_specialairn_blade, 3 => game_specialairn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairn", 2 => effect_specialairn_blade, 3 => effect_specialairn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialairn", 2 => sound_specialn_blade, 3 => sound_specialn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "expression_specialairn", 2 => expression_specialairn_blade, 3 => expression_specialairn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "game_specialairnturn", 2 => game_specialairnturn_blade, 3 => game_specialairnturn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairnturn", 2 => effect_specialairnturn_blade, 3 => effect_specialairnturn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialairnturn", 2 => sound_specialnturn_blade, 3 => sound_specialnturn_thrust);
    config::custom_acmd!(agent, Special::Neutral, "expression_specialairnturn", 2 => expression_specialairnturn_blade, 3 => expression_specialairnturn_thrust);
    config::custom_acmd!(agent, Special::Side, "game_specialsstart", 2 => game_specialsstart_wave, 3 => game_specialsstart_chain);
    config::custom_acmd!(agent, Special::Side, "game_specialairsstart", 2 => game_specialairsstart_wave, 3 => game_specialairsstart_chain);
    config::custom_acmd!(agent, Special::Side, "game_specials", 2 => game_specials_wave);
    config::custom_acmd!(agent, Special::Side, "game_specialairs", 2 => game_specialairs_wave);
    config::custom_acmd!(agent, Special::Side, "effect_specialsstart", 2 => effect_specialsstart_wave, 3 => effect_specialsstart_chain);
    config::custom_acmd!(agent, Special::Side, "effect_specialairsstart", 2 => effect_specialairsstart_wave, 3 => effect_specialairsstart_chain);
    config::custom_acmd!(agent, Special::Side, "effect_specials", 2 => effect_specials_wave);
    config::custom_acmd!(agent, Special::Side, "effect_specialairs", 2 => effect_specialairs_wave);
    config::custom_acmd!(agent, Special::Side, "sound_specialsstart", 2 => sound_specialsstart_wave, 3 => sound_specialsstart_chain);
    config::custom_acmd!(agent, Special::Side, "sound_specialairsstart", 2 => sound_specialairsstart_wave, 3 => sound_specialairsstart_chain);
    config::custom_acmd!(agent, Special::Up, "game_specialhi", 2 => game_specialhi_fists, 3 => game_specialhi_vault);
    config::custom_acmd!(agent, Special::Up, "game_specialairhi", 2 => game_specialhi_fists, 3 => game_specialhi_vault);
    config::custom_acmd!(agent, Special::Up, "game_specialhicatch", 3 => game_specialhicatch_vault);
    config::custom_acmd!(agent, Special::Up, "game_specialhithrow", 3 => game_specialhithrow_vault);
    config::custom_acmd!(agent, Special::Up, "effect_specialhi", 2 => effect_specialhi_fists, 3 => effect_specialhi_vault);
    config::custom_acmd!(agent, Special::Up, "effect_specialairhi", 2 => effect_specialhi_fists, 3 => effect_specialhi_vault);
    config::custom_acmd!(agent, Special::Up, "sound_specialhi", 2 => sound_specialhi_fists, 3 => sound_specialhi_vault);
    config::custom_acmd!(agent, Special::Up, "sound_specialairhi", 2 => sound_specialhi_fists, 3 => sound_specialhi_vault);
    config::custom_acmd!(agent, Special::Up, "expression_specialhi", 2 => expression_specialhi_fists);
    config::custom_acmd!(agent, Special::Up, "expression_specialairhi", 2 => expression_specialairhi_fists);
    config::custom_acmd!(agent, Special::Down, "game_speciallw", 2 => game_speciallw_dropkick, 3 => game_speciallw_assault);
    config::custom_acmd!(agent, Special::Down, "game_specialairlw", 2 => game_specialairlw_dropkick, 3 => game_specialairlw_assault);
    config::custom_acmd!(agent, Special::Down, "game_speciallwend", 2 => game_speciallwend_dropkick, 3 => game_speciallwend_assault);
    config::custom_acmd!(agent, Special::Down, "game_specialairlwend", 2 => game_specialairlwend_dropkick, 3 => game_specialairlwend_assault);
    config::custom_acmd!(agent, Special::Down, "game_specialairlwendair", 2 => game_specialairlwendair_dropkick, _ => acmd_stub);
    config::custom_acmd!(agent, Special::Down, "effect_speciallw", 2 => effect_speciallw_dropkick, 3 => effect_speciallw_assault);
    config::custom_acmd!(agent, Special::Down, "effect_specialairlw", 2 => effect_specialairlw_dropkick, 3 => effect_specialairlw_assault);
    config::custom_acmd!(agent, Special::Down, "sound_speciallw", 2 => sound_speciallw_dropkick, 3 => sound_speciallw_assault);
    config::custom_acmd!(agent, Special::Down, "sound_specialairlw", 3 => sound_specialairlw_assault);
}
