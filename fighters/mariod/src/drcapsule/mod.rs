use super::*;

mod acmd;

pub fn install() {
    let agent = &mut config::agent("mariod_drcapsule");
    acmd::install(agent);
    agent.install();
}
