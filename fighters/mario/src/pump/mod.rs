use super::*;

mod acmd;

pub fn install() {
    let agent = &mut config::agent("mario_pump");
    acmd::install(agent);
    agent.install();
}
