use super::*;

// claude code: no getup mixups. a fighter knocked down lies there with every getup option (stand,
// roll, getup attack) locked, for as long as the hitstun they still had left when they landed
// (DOWN_FRAME), then can act straight out of lying the way tumble can act out of the air: CancelModule's
// cancel and every ground action, but no getups and no automatic stand (2026-11-27, Grant's design). a
// launch in hitstun floor-bounces instead of landing (damage.rs), so a tumble landing (hitstun already
// over) is actionable at once, and only a down that comes while hitstun is still running lies there.
//
// a grab that slams (Ganondorf's Flame Choke) downs them with no hitstun at all: its throw hit's
// knockback never launches them. that down lies for the hitstun the throw would have dealt, worked out
// the way the game does for any hit (throw_hitstun)

// claude code: the lying statuses
unsafe fn down_statuses() -> [i32; 6] {
    [
        *FIGHTER_STATUS_KIND_DOWN,
        *FIGHTER_STATUS_KIND_DOWN_WAIT,
        *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
        *FIGHTER_STATUS_KIND_DOWN_CONTINUE,
        *FIGHTER_STATUS_KIND_DOWN_SPOT,
        *FIGHTER_STATUS_KIND_DOWN_REFLECT_LR
    ]
}

// claude code: the hit and launch statuses
unsafe fn hit_statuses() -> [i32; 10] {
    [
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY
    ]
}

// claude code: free = the fighter's own statuses (moving, attacking, shielding, specials...). every
// status done TO a fighter (grabs, hits, lying, burying, sleep...) sits from CAPTURE_PULLED up to the
// specials
unsafe fn is_free_status(status: i32) -> bool {
    status < *FIGHTER_STATUS_KIND_CAPTURE_PULLED || status >= *FIGHTER_STATUS_KIND_SPECIAL_N
}

// claude code: getup inputs are always off (transition.rs)
pub unsafe fn is_getup_locked(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    boma.is_fighter()
    && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_STAND
}

// claude code: the ground action groups a downed fighter gets once the lying time is up
unsafe fn ground_groups() -> [i32; 8] {
    [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND
    ]
}

// claude code: a throw hit (ATTACK_ABS kind THROW, attack.rs) on a held fighter keeps its numbers on
// them, for the down it may put them in. damage is the damage dealt (Divergence's 1.2x on throws in)
pub unsafe fn note_throw(victim: *mut BattleObjectModuleAccessor, damage: f32, kbg: f32, fkb: f32, bkb: f32) {
    let object = (*victim).object();
    VarModule::on_flag(object, vars::common::instance::THROW_PENDING);
    VarModule::set_float(object, vars::common::instance::THROW_DAMAGE, damage);
    VarModule::set_float(object, vars::common::instance::THROW_KBG, kbg);
    VarModule::set_float(object, vars::common::instance::THROW_FKB, fkb);
    VarModule::set_float(object, vars::common::instance::THROW_BKB, bkb);
}

// claude code: the hitstun the noted throw deals, the game's own way (Rubendal's SSBU calculator,
// js/formulas.js): knockback ((((p+d)/10 + (p+d)*d/20) * 1.4 * 200/(weight+100)) + 18) * kbg/100 + bkb,
// or for set knockback (((1 + fkb/2) * 200/(weight+100) * 1.4) + 18) * kbg/100 + bkb, then hitstun
// floor(max(knockback * damage_frame_mul, 5)) - 1. p+d is their percent now, the throw's damage in.
// Divergence has no rage, so none is counted
unsafe fn throw_hitstun(fighter: &mut L2CFighterCommon) -> i32 {
    let object = fighter.battle_object;
    let damage = VarModule::get_float(object, vars::common::instance::THROW_DAMAGE);
    let kbg = VarModule::get_float(object, vars::common::instance::THROW_KBG);
    let fkb = VarModule::get_float(object, vars::common::instance::THROW_FKB);
    let bkb = VarModule::get_float(object, vars::common::instance::THROW_BKB);
    let percent = DamageModule::damage(fighter.module_accessor, 0);
    let weight_mul = 200.0 / (fighter.get_param_float("weight", "") + 100.0);
    let base = if fkb > 0.0 {
        (1.0 + fkb / 2.0) * weight_mul * 1.4
    } else {
        (percent / 10.0 + percent * damage / 20.0) * weight_mul * 1.4
    };
    let knockback = (base + 18.0) * kbg / 100.0 + bkb;
    let mul = fighter.get_param_float("battle_object", "damage_frame_mul");
    ((knockback * mul).max(5.0).floor() - 1.0) as i32
}

// claude code: every frame (sys_line, opff.rs). on the first lying frame (DOWN_STARTED) DOWN_FRAME is
// set: the hitstun left (FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME), or with none left and
// a throw noted, the throw's (throw_hitstun). a noted throw is dropped on any hit, launch or free
// status in between (not DAMAGE_FALL: a grab slam ends in it), so it only stands for its own slam.
// DOWN_FRAME counts down while they lie there; at 0 they act (act_from_lying), here for the lying
// statuses whose main isn't replaced
pub unsafe fn forced_down(fighter: &mut L2CFighterCommon) {
    let status = fighter.status();
    let object = fighter.battle_object;
    if (hit_statuses().contains(&status) && status != *FIGHTER_STATUS_KIND_DAMAGE_FALL)
    || is_free_status(status) {
        VarModule::off_flag(object, vars::common::instance::THROW_PENDING);
    }
    if !down_statuses().contains(&status) {
        VarModule::off_flag(object, vars::common::instance::DOWN_STARTED);
        return;
    }
    if !VarModule::is_flag(object, vars::common::instance::DOWN_STARTED) {
        VarModule::on_flag(object, vars::common::instance::DOWN_STARTED);
        let hitstun = fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME).max(0.0).ceil() as i32;
        let lie = if hitstun == 0 && VarModule::is_flag(object, vars::common::instance::THROW_PENDING) {
            throw_hitstun(fighter)
        } else {
            hitstun
        };
        VarModule::off_flag(object, vars::common::instance::THROW_PENDING);
        VarModule::set_int(object, vars::common::instance::DOWN_FRAME, lie);
    }
    if VarModule::get_int(object, vars::common::instance::DOWN_FRAME) > 0 {
        VarModule::dec_int(object, vars::common::instance::DOWN_FRAME);
        return;
    }
    // the two lying loops act from their own main (sub_down_wait_common below)
    if [*FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE].contains(&status)
    || StatusModule::is_changing(fighter.module_accessor)
    || StopModule::is_stop(fighter.module_accessor) {
        return;
    }
    act_from_lying(fighter);
}

// claude code: cancel on, the ground action groups on, and the ground action check WAIT runs. true =
// they acted
unsafe fn act_from_lying(fighter: &mut L2CFighterCommon) -> bool {
    CancelModule::enable_cancel(fighter.module_accessor);
    for group in ground_groups() {
        WorkModule::enable_transition_term_group(fighter.module_accessor, group);
    }
    fighter.sub_wait_ground_check_common(false.into()).get_bool()
}

// claude code: the body of both lying loops (DOWN_WAIT, DOWN_WAIT_CONTINUE), replaced on the ground.
// vanilla's asks for a getup on any input and after its own timer; with getups sent back to DOWN_WAIT,
// a held input restarted DOWN_WAIT every frame and a status that's changing every frame never got to
// act, so the fighter was stuck down. this one never asks for a getup: it lies until the lying time is
// up, then acts like WAIT. off the ground vanilla's runs
#[skyline::hook(replace = L2CFighterCommon_sub_down_wait_common)]
unsafe fn sub_down_wait_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        return call_original!(fighter);
    }
    if VarModule::get_int(fighter.battle_object, vars::common::instance::DOWN_FRAME) > 0 {
        return false.into();
    }
    act_from_lying(fighter).into()
}

// claude code: a getup that starts anyway (vanilla's own auto getup, or anything the locked input
// misses) goes back to lying instead. no automatic stand: a downed fighter gets up only by acting
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DownStand)]
unsafe fn status_pre_DownStand(fighter: &mut L2CFighterCommon) -> L2CValue {
    getup_to_lying(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DownStandFb)]
unsafe fn status_pre_DownStandFb(fighter: &mut L2CFighterCommon) -> L2CValue {
    getup_to_lying(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DownStandAttack)]
unsafe fn status_pre_DownStandAttack(fighter: &mut L2CFighterCommon) -> L2CValue {
    getup_to_lying(fighter)
}

unsafe fn getup_to_lying(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.boma().set_status_kind_interrupt(*FIGHTER_STATUS_KIND_DOWN_WAIT);
    1.into()
}

// hdr: this and the hook after it are HDR's (fighters/common/src/general_statuses/down.rs)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Down)]
unsafe fn status_pre_Down(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_SLOPE_TOP_UNLIMIT as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_DamageFlyChkUniq)]
unsafe fn sub_DamageFlyChkUniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = call_original!(fighter);
    if ret.get_bool() {
        if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP) {
            fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
            fighter.set_float(0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
        }
    }
    ret
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_DamageFlyChkUniq,
            sub_down_wait_common,
            status_pre_Down,
            status_pre_DownStand,
            status_pre_DownStandFb,
            status_pre_DownStandAttack
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
