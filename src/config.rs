// claude code: Divergence's player settings, read from sd:/ultimate/divergence/config.toml. two
// tables: [modes] turns roster-wide systems on (each one a bool, all off by default), and [variants]
// turns divergent variants off (any variant not listed is on). the file is read once, the first time
// anything asks (config()), which happens at boot while fighters install. so changes need a restart.
// with no file, a default one is written for the player to edit. a file that doesn't parse is logged
// and the defaults are used, so a typo never stops the game
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

const PATH: &str = "sd:/ultimate/divergence/config.toml";

const DEFAULT_FILE: &str = "\
[modes]
faux_air = false
aura = false

[variants]
# e.g. example = false
";

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Modes {
    pub faux_air: bool,
    pub aura: bool,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub modes: Modes,
    pub variants: HashMap<String, bool>,
}

impl Config {
    // a variant is on unless the file says false
    pub fn variant_enabled(&self, name: &str) -> bool {
        self.variants.get(name).copied().unwrap_or(true)
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG.get_or_init(load)
}

// the [modes] table, for the checks that run every frame
pub fn modes() -> &'static Modes {
    &config().modes
}

fn load() -> Config {
    let text = match std::fs::read_to_string(PATH) {
        Ok(text) => text,
        Err(_) => {
            let _ = std::fs::create_dir_all("sd:/ultimate/divergence");
            match std::fs::write(PATH, DEFAULT_FILE) {
                Ok(_) => println!("[Divergence] wrote default {}", PATH),
                Err(error) => println!("[Divergence] couldn't write {}: {:?}", PATH, error)
            }
            DEFAULT_FILE.to_string()
        }
    };
    match toml::from_str::<Config>(&text) {
        Ok(config) => {
            println!("[Divergence] config: faux_air = {}, aura = {}", config.modes.faux_air, config.modes.aura);
            config
        }
        Err(error) => {
            println!("[Divergence] {} didn't parse, using defaults: {}", PATH, error);
            Config::default()
        }
    }
}
