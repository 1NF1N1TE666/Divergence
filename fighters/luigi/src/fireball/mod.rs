use super::*;

mod acmd;

pub fn install() {
    let agent = &mut config::agent("luigi_fireball");
    acmd::install(agent);
    agent.install();
}
