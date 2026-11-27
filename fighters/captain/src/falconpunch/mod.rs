use super::*;

mod acmd;

pub fn install() {
    let agent = &mut config::agent("captain_falconpunch");
    acmd::install(agent);
    agent.install();
}
