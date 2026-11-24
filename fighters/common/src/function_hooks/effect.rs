use super::*;

// claude code: the effect and sound commands of every ACMD script, so a divergent variant can swap the
// effect, trail or sound its base's scripts ask for (common/src/variants.rs). each hook takes the
// command's arguments off the lua stack, lets the variant change them, and puts them back. the
// argument counts are the commands' own (smash_script's macros push the same ones as vanilla scripts)

unsafe fn rewrite(lua_state: u64, count: usize, change: unsafe fn(*mut BattleObjectModuleAccessor, &mut Vec<L2CValue>)) {
    if !crate::fighters::common::variants::is_variant(sv_system::battle_object_module_accessor(lua_state)) {
        return;
    }
    let mut agent = L2CAgent::new(lua_state);
    let mut params: Vec<L2CValue> = (0..count).map(|i| agent.pop_lua_stack(i as i32 + 1)).collect();
    agent.clear_lua_stack();
    change(sv_system::battle_object_module_accessor(lua_state), &mut params);
    for param in params.iter_mut() {
        agent.push_lua_stack(param);
    }
}

// trails: 0 and 1 are the trail textures, 12 the flare effect on the blade
#[skyline::hook(replace = smash::app::sv_animcmd::AFTER_IMAGE4_ON_arg29)]
unsafe fn divergence_after_image4_on(lua_state: u64) {
    rewrite(lua_state, 29, crate::fighters::common::variants::on_after_image);
    original!()(lua_state);
}

// 0 is the effect
#[skyline::hook(replace = smash::app::sv_animcmd::EFFECT_FOLLOW)]
unsafe fn divergence_effect_follow(lua_state: u64) {
    rewrite(lua_state, 10, crate::fighters::common::variants::on_effect);
    original!()(lua_state);
}

// 0 is the effect
#[skyline::hook(replace = smash::app::sv_animcmd::EFFECT_OFF_KIND)]
unsafe fn divergence_effect_off_kind(lua_state: u64) {
    rewrite(lua_state, 3, crate::fighters::common::variants::on_effect);
    original!()(lua_state);
}

// 0 is the sound
#[skyline::hook(replace = smash::app::sv_animcmd::PLAY_SE)]
unsafe fn divergence_play_se(lua_state: u64) {
    rewrite(lua_state, 1, crate::fighters::common::variants::on_sound);
    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        divergence_after_image4_on,
        divergence_effect_follow,
        divergence_effect_off_kind,
        divergence_play_se
    );
}
