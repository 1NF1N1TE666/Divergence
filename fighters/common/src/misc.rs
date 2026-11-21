use super::*;
use smash::app::sv_animcmd::frame;
use smash_script::macros::FT_MOTION_RATE;

// claude code: plays an ACMD script's animation on a per-frame rate curve instead of a
// few flat FT_MOTION_RATE steps. rates[f] is how many real frames anim frame f lasts
// (same meaning as FT_MOTION_RATE), so slow-downs can sit on frames where the pose holds
// still and fast swings keep close to normal speed. Call it between a script's events:
// it walks the curve up to anim frame `until`, setting each frame's rate as it's reached,
// and returns there. Frames already passed are re-set instantly, so every call can start
// from 0. Frames past the end of `rates` keep the last rate.
pub unsafe fn rate_curve(agent: &mut L2CAgentBase, rates: &[f32], until: f32) {
    let lua_state = agent.lua_state_agent;
    let mut f = 0;
    while (f as f32) < until {
        frame(lua_state, f as f32);
        FT_MOTION_RATE(agent, rates[f.min(rates.len() - 1)]);
        f += 1;
    }
    frame(lua_state, until);
}
