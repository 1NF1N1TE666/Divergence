use super::*;
use globals::*;

// warlock blade: the punch's statuses with the blade's own motions. vanilla's main picks a motion, and
// later landing or leaving the ground picks again from the two ints (inheriting the frame)
unsafe fn blade_motions(fighter: &mut L2CFighterCommon, ground: u64, air: u64) {
    let boma = fighter.module_accessor;
    WorkModule::set_int64(boma, ground, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(boma, air, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_AIR_MOT);
    let is_ground = fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND;
    let motion = if is_ground { ground } else { air };
    MotionModule::change_motion(boma, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
}

unsafe extern "C" fn blade_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let main = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    blade_motions(fighter, hash40("special_n2"), hash40("special_air_n2"));
    main
}

unsafe extern "C" fn blade_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let main = smashline::original_status(Main, fighter, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN)(fighter);
    blade_motions(fighter, hash40("special_n2_turn"), hash40("special_air_n2_turn"));
    main
}

// the sword goes away however the move ends
unsafe extern "C" fn blade_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

// Custom::statuses
pub unsafe fn blade(fighter: &mut L2CFighterCommon) {
    let (n, turn) = (*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN);
    config::custom_status(fighter, Special::Neutral, n, *LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN, blade_main as *const ());
    config::custom_status(fighter, Special::Neutral, turn, *LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN, blade_turn_main as *const ());
    config::custom_status(fighter, Special::Neutral, n, *LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS, blade_exit as *const ());
    config::custom_status(fighter, Special::Neutral, turn, *LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS, blade_exit as *const ());
}
