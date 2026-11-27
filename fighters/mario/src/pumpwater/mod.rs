use super::*;

mod acmd;

pub fn install() {
    let agent = &mut config::agent("mario_pumpwater");
    acmd::install(agent);
    agent.install();
}
