// claude code: Divergence's player settings, read from sd:/ultimate/divergence/config.toml. three
// tables, and nothing is on unless the file adds it:
// - [modes] adds roster-wide systems, each one `name = fixed` (always on), `name = rand` (a coin flip
//   at boot) or `name = rand_solo` (a coin flip too, but only when no other mode came on, and if
//   several rand_solo modes win theirs, one of them is picked evenly). resolved once at boot (resolve_modes)
// - [variants] adds divergent variants, one name per line (marth_reflet), nothing else needed
// - [customs.<fighter>] gives costume slots Sm4sh custom specials (below; a slot not listed has none)
// fixed/rand/rand_solo and bare variant names aren't TOML, so the file is tidied into TOML first
// (to_toml). the file is read once, the first time anything asks (config()), which happens at boot
// while fighters install. so changes need a restart. with no file, a default one is written for the
// player to edit. a file that doesn't parse is logged and the defaults are used, so a typo never
// stops the game
use crate::{consts::{globals::*, *}, ext::*, VarModule};
use serde::Deserialize;
use smash::app::{lua_bind::*, *};
use smash::lib::{L2CValue, lua_const::*, utility::Variadic};
use smash::lua2cpp::L2CFighterCommon;
use std::collections::{BTreeMap, HashMap};
use std::fs::DirEntry;
use std::path::Path;
use std::sync::OnceLock;

const FOLDER: &str = "sd:/ultimate/divergence";
const PATH: &str = "sd:/ultimate/divergence/config.toml";

const DEFAULT_FILE: &str = "\
# e.g.
#
# [modes]
# mode = fixed
# mode = rand
# mode = rand_solo
#
# [variants]
# fighter_variant
#
# [customs.fighter]
# cXX = \"1111\"
# cXX = \"2222\"
# cXX = \"3333\"
";

// the modes in play this boot, worked out from [modes] by resolve_modes
#[derive(Default, Debug)]
pub struct Modes {
    pub faux_air: bool,
    pub aura: bool,
}

impl Modes {
    // turns a mode on by its config name. false for a name that isn't a mode
    fn add(&mut self, name: &str) -> bool {
        match name {
            "faux_air" => self.faux_air = true,
            "aura" => self.aura = true,
            _ => return false
        }
        true
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Config {
    // mode -> fixed / rand / rand_solo, as written
    #[serde(rename = "modes")]
    mode_rules: BTreeMap<String, String>,
    #[serde(skip)]
    pub modes: Modes,
    // every variant the file lists (to_toml writes them as name = true)
    variants: BTreeMap<String, bool>,
    // fighter -> slot -> numbers, as written. slots that read right end up in custom_slots
    customs: BTreeMap<String, BTreeMap<String, toml::Value>>,
    #[serde(skip)]
    pub custom_slots: Vec<CustomSlot>,
}

// one slot of [customs.<fighter>]
pub struct CustomSlot {
    pub fighter: String,
    pub slot: usize,
    // neutral, side, up, down, each 1 to 3
    pub numbers: [u8; 4]
}

// costume slots run from c00 to c255
const SLOT_COUNT: usize = 256;

// "c01" -> 1
fn parse_slot(name: &str) -> Option<usize> {
    name.strip_prefix('c')?.parse().ok().filter(|slot| *slot < SLOT_COUNT)
}

// "1121" (or 1121 without the quotes) -> [1, 1, 2, 1]
fn parse_numbers(value: &toml::Value) -> Option<[u8; 4]> {
    let text = value.as_str().map_or_else(|| value.to_string(), str::to_string);
    let numbers: Vec<u8> = text.bytes().map(|digit| digit.wrapping_sub(b'0')).collect();
    let valid = numbers.len() == 4 && numbers.iter().all(|number| (1..=3).contains(number));
    if valid { Some([numbers[0], numbers[1], numbers[2], numbers[3]]) } else { None }
}

// a random number below `count`, from the console's random number generator
fn random_below(count: usize) -> usize {
    let mut bytes = [0u8; 8];
    unsafe { skyline::nn::os::GenerateRandomBytes(bytes.as_mut_ptr(), bytes.len() as u64); }
    (u64::from_le_bytes(bytes) % count as u64) as usize
}

fn coin_flip() -> bool {
    random_below(2) == 0
}

// the file as TOML: in [modes], a bare word after the = gets quotes (aura = rand -> aura = "rand"),
// and in [variants], a line that's only a name becomes name = true. everything else is left as written
fn to_toml(text: &str) -> String {
    let mut table = String::new();
    let mut out = String::new();
    for line in text.lines() {
        let code = line.split('#').next().unwrap_or("").trim();
        if code.starts_with('[') {
            table = code.trim_matches(|c| c == '[' || c == ']').trim().to_string();
        } else if table == "modes" {
            if let Some((name, value)) = code.split_once('=') {
                out.push_str(&format!("{} = \"{}\"\n", name.trim(), value.trim().trim_matches('"')));
                continue;
            }
        } else if table == "variants" && !code.is_empty() && !code.contains('=') {
            out.push_str(&format!("{} = true\n", code.trim_matches('"')));
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

impl Config {
    // fixed modes come on, rand modes on a coin flip each. then, only if none of those came on, every
    // rand_solo mode flips its own coin, and one of the winners is picked evenly
    fn resolve_modes(&mut self) {
        let mut any = false;
        let mut solo = vec![];
        for (name, rule) in &self.mode_rules {
            let add = match rule.as_str() {
                "fixed" => true,
                "rand" => coin_flip(),
                "rand_solo" => {
                    if coin_flip() {
                        solo.push(name.as_str());
                    }
                    continue;
                }
                _ => {
                    crate::log(&format!("[Divergence] modes: {} = {} skipped, it should be fixed, rand or rand_solo", name, rule));
                    continue;
                }
            };
            if !add {
                continue;
            }
            if self.modes.add(name) {
                any = true;
            } else {
                crate::log(&format!("[Divergence] modes: no mode called {}", name));
            }
        }
        if !any && !solo.is_empty() {
            let name = solo[random_below(solo.len())];
            if !self.modes.add(name) {
                crate::log(&format!("[Divergence] modes: no mode called {}", name));
            }
        }
    }

    fn load() -> Config {
        let text = std::fs::read_to_string(PATH).unwrap_or_else(|_| {
            let _ = std::fs::create_dir_all(FOLDER);
            let _ = std::fs::write(PATH, DEFAULT_FILE);
            DEFAULT_FILE.to_string()
        });
        let mut config: Config = toml::from_str(&to_toml(&text)).unwrap_or_else(|error| {
            crate::log(&format!("[Divergence] {} didn't parse, using defaults: {}", PATH, error));
            Config::default()
        });
        config.resolve_modes();
        crate::log(&format!("[Divergence] config: {:?}", config.modes));
        for (fighter, slots) in &config.customs {
            for (slot, numbers) in slots {
                match (parse_slot(slot), parse_numbers(numbers)) {
                    (Some(slot), Some(numbers)) => config.custom_slots.push(CustomSlot { fighter: fighter.clone(), slot, numbers }),
                    _ => crate::log(&format!("[Divergence] customs: {} {} = {} skipped, it should look like c01 = \"1121\" (numbers 1 to 3)", fighter, slot, numbers))
                }
            }
        }
        config
    }

    // a variant is on when the file lists it
    pub fn variant_enabled(&self, name: &str) -> bool {
        self.variants.get(name).copied().unwrap_or(false)
    }
}

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(Config::load)
}

// the [modes] table, for the checks that run every frame
pub fn modes() -> &'static Modes {
    &config().modes
}

// claude code: custom moveset support. a single-slot moveset (SSM) marks its slots with a .marker file in
// fighter/<name>/model/body/cXX/ of its mod folder, the same signal Ultimate S and SSBEXO read. Divergence's
// fighter edits run on every slot except marked ones, so a marked slot plays its own moveset on top of
// vanilla, and skins on added slots still get Divergence. universal mechanics (common) stay on every slot.
//
// two halves:
// - smashline scripts: make each fighter or article agent with `config::agent` instead of
//   `Agent::new`, and smashline itself skips the marked slots (acmd, status, opff and on_start alike)
// - everything smashline doesn't route (skyline hooks, common code that checks for one fighter):
//   `config::is_custom` before running the edit

// slots marked per fighter folder name ("lucario", "littlemac"...), filled once by scan_movesets()
static MARKED: OnceLock<HashMap<String, Vec<usize>>> = OnceLock::new();

// what's in a folder, nothing if it isn't there
pub fn entries(folder: impl AsRef<Path>) -> impl Iterator<Item = DirEntry> {
    std::fs::read_dir(folder).into_iter().flatten().flatten()
}

// finds every marked slot in an enabled mod folder (ARCropolis skips folders starting with a dot)
pub fn scan_movesets() {
    let mut marked: HashMap<String, Vec<usize>> = HashMap::new();
    for folder in entries("sd:/ultimate/mods") {
        if folder.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        for fighter in entries(folder.path().join("fighter")) {
            let mut moveset: Vec<usize> = entries(fighter.path().join("model/body"))
                .filter(|body| entries(body.path()).any(|file| file.path().extension().map_or(false, |ext| ext == "marker")))
                .filter_map(|body| parse_slot(&body.file_name().to_string_lossy()))
                .collect();
            if moveset.is_empty() {
                continue;
            }
            moveset.sort();
            let name = fighter.file_name().to_string_lossy().into_owned();
            crate::log(&format!("[Divergence] custom moveset for {} in {}: slots {:?}", name, folder.file_name().to_string_lossy(), moveset));
            marked.entry(name).or_default().extend(moveset);
        }
    }
    let _ = MARKED.set(marked);
}

fn marked(fighter: &str) -> &'static [usize] {
    MARKED.get()
        .and_then(|marked| marked.get(fighter))
        .map_or(&[], |slots| slots.as_slice())
}

pub fn is_marked(fighter: &str, slot: usize) -> bool {
    marked(fighter).contains(&slot)
}

// a smashline agent for a fighter ("lucario") or one of its articles ("lucario_auraball"), limited to
// the fighter's unmarked slots. with nothing marked it stays a plain agent for every slot
pub fn agent(name: &str) -> smashline::Agent {
    let mut agent = smashline::Agent::new(name);
    // Nana follows Popo's slots, so the Ice Climbers' marks sit in Popo's folder
    let fighter = match name.split('_').next().unwrap_or(name) {
        "nana" => "popo",
        fighter => fighter
    };
    let marked = marked(fighter);
    if !marked.is_empty() {
        agent.set_costume((0..SLOT_COUNT).filter(|slot| !marked.contains(slot)).collect());
    }
    agent
}

// the fighter itself, or the fighter that owns a weapon. None for anything else
pub unsafe fn fighter_of(boma: *mut BattleObjectModuleAccessor) -> Option<*mut BattleObjectModuleAccessor> {
    let category = utility::get_category(&mut *boma);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        return Some(boma);
    }
    if category != *BATTLE_OBJECT_CATEGORY_WEAPON {
        return None;
    }
    let owner_id = (*boma).get_int(*WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if !sv_battle_object::is_active(owner_id) {
        return None;
    }
    Some(sv_battle_object::module_accessor(owner_id))
}

// claude code: Dionysus clones count as custom movesets too. a clone runs its own scripts, but
// utility::get_kind answers its base's kind, so by kind alone a Lucario clone looks like Lucario and
// would get Divergence's Lucario edits. Dionysus exports clone_engine_fighter_kind_v1, which answers a
// fighter's clone kind (-1 for a vanilla fighter). it's looked up once, the first time a match asks;
// without Dionysus the symbol isn't there and no fighter is a clone
type CloneKind = extern "C" fn(*mut BattleObjectModuleAccessor) -> i32;

unsafe fn is_clone(fighter: *mut BattleObjectModuleAccessor) -> bool {
    static mut CLONE_KIND: Option<Option<CloneKind>> = None;
    if CLONE_KIND.is_none() {
        let mut address: usize = 0;
        skyline::nn::ro::LookupSymbol(&mut address, "clone_engine_fighter_kind_v1\0".as_ptr() as _);
        CLONE_KIND = Some(if address == 0 { None } else { Some(std::mem::transmute(address)) });
    }
    match CLONE_KIND {
        Some(Some(clone_kind)) => clone_kind(fighter) >= 0,
        _ => false
    }
}

// true when this fighter, or the fighter that owns this weapon, is on a slot `fighter`'s custom
// moveset marked, or is a Dionysus clone, so Divergence's edits for that fighter should leave it alone
pub unsafe fn is_custom(boma: *mut BattleObjectModuleAccessor, fighter: &str) -> bool {
    let owner = match fighter_of(boma) {
        Some(owner) => owner,
        None => return false
    };
    if is_clone(owner) {
        return true;
    }
    let marked = marked(fighter);
    if marked.is_empty() {
        return false;
    }
    let slot = (*owner).get_int(*FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    slot >= 0 && marked.contains(&(slot as usize))
}

// claude code: a fighter's params, slotted. ARCropolis patches a param file for every slot, so Divergence's
// per-fighter param patches live with each fighter instead (fighters/<name>/param/: vl.prcxml for its
// vl.prc, fighter_param.prcxml for its row of fighter_param_table), are built into the plugin and are
// handed out by the get_param hooks (function_hooks/get_param.rs) on unmarked slots only. Ultimate S and
// SSBEXO slot params through the same hooks (via lib_paramconfig); here the prcxml stays the source
#[derive(Clone, Copy)]
pub enum Param {
    Int(i32),
    Float(f32)
}

// (param, field) -> value. a fighter attribute (fighter_param.prcxml) is keyed (attribute, 0), the way
// the game asks for it
type ParamMap = HashMap<(u64, u64), Param>;

// one per fighter that registered its params. every: for all its unmarked slots. customs: the rows of
// its Sm4sh customs (below) that the game doesn't read by itself, over that
struct FighterParams {
    kind: i32,
    fighter: &'static str,
    every: ParamMap,
    customs: Vec<(Special, u8, ParamMap)>
}

static mut PARAMS: Vec<FighterParams> = Vec::new();

// a prcxml hash: a name, or a raw hash written as 0x...
fn prc_hash(text: &str) -> Option<u64> {
    match text.strip_prefix("0x") {
        Some(hex) => u64::from_str_radix(hex, 16).ok(),
        None => Some(smash::hash40(text))
    }
}

// the text between `<tag ... attribute="` and the next quote
fn prc_attribute<'a>(line: &'a str, attribute: &str) -> Option<&'a str> {
    let start = line.find(&format!("{}=\"", attribute))? + attribute.len() + 2;
    line[start..].split('"').next()
}

// reads a prcxml into params: every int and float at the top level (keyed with field 0), in a
// top-level struct, or in one row of a top-level list ("0", the entry the game reads, or a Sm4sh
// custom's; the other indexes don't count). anything nested deeper is skipped
fn parse_prcxml(text: &str, row: &str, params: &mut ParamMap) {
    let mut group: Option<u64> = None;
    let mut in_list = false;
    let mut in_entry = false;
    let mut active = false;
    let mut skip_depth = 0;
    for line in text.lines().map(str::trim) {
        let opens_group = line.starts_with("<list hash=") || line.starts_with("<struct hash=");
        if skip_depth > 0 {
            if opens_group {
                skip_depth += 1;
            } else if line.starts_with("</struct") || line.starts_with("</list") {
                skip_depth -= 1;
            }
            continue;
        }
        if opens_group && group.is_some() {
            skip_depth = 1;
        } else if opens_group {
            group = prc_attribute(line, "hash").and_then(prc_hash);
            in_list = line.starts_with("<list");
            active = !in_list;
        } else if line.starts_with("<struct index=") {
            in_entry = true;
            active = prc_attribute(line, "index") == Some(row);
        } else if line.starts_with("</struct") && in_entry {
            in_entry = false;
            active = false;
        } else if line.starts_with("</struct") || line.starts_with("</list") {
            group = None;
            active = false;
        } else if line.starts_with("<float ") || line.starts_with("<int ") {
            let field = match prc_attribute(line, "hash").and_then(prc_hash) {
                Some(field) => field,
                None => continue
            };
            let key = match group {
                None => (field, 0),
                Some(group) if active => (group, field),
                Some(_) => continue
            };
            let value = match line.split('>').nth(1).and_then(|rest| rest.split('<').next()) {
                Some(value) => value.trim(),
                None => continue
            };
            let param = if line.starts_with("<float ") {
                value.parse::<f32>().ok().map(Param::Float)
            } else {
                value.parse::<i32>().ok().map(Param::Int)
            };
            if let Some(param) = param {
                params.insert(key, param);
            }
        }
    }
}

// registers a fighter's param patches (from its lib.rs install, each with include_str!)
pub fn register_params(kind: i32, fighter: &'static str, prcxmls: &[&str]) {
    let mut params = HashMap::new();
    for prcxml in prcxmls {
        parse_prcxml(prcxml, "0", &mut params);
    }
    crate::log(&format!("[Divergence] {} params: {}", fighter, params.len()));
    params_entry(kind, fighter).every = params;
}

// registers the rows of a fighter's Sm4sh customs: row 1 for number 2, row 2 for 3
pub fn custom_params(kind: i32, fighter: &'static str, lists: &[(&str, Special)], vl: &str) {
    let mut customs = vec![];
    for number in 2..=3u8 {
        let mut row = HashMap::new();
        parse_prcxml(vl, &(number - 1).to_string(), &mut row);
        for (list, special) in lists {
            let list = smash::hash40(list);
            let params: ParamMap = row.iter()
                .filter(|((group, _), _)| *group == list)
                .map(|(key, value)| (*key, *value))
                .collect();
            customs.push((*special, number, params));
        }
    }
    params_entry(kind, fighter).customs = customs;
}

// a fighter's entry in PARAMS, made on the first call
fn params_entry(kind: i32, fighter: &'static str) -> &'static mut FighterParams {
    unsafe {
        if !has_params(kind) {
            PARAMS.push(FighterParams { kind, fighter, every: HashMap::new(), customs: vec![] });
        }
        PARAMS.iter_mut().find(|params| params.kind == kind).unwrap()
    }
}

// true when this fighter kind registered params
pub fn has_params(kind: i32) -> bool {
    unsafe { PARAMS.iter().any(|params| params.kind == kind) }
}

// Divergence's value for this param, if the fighter (or the weapon's owner) has one and isn't on a custom
// moveset's slot
pub unsafe fn slot_param(boma: *mut BattleObjectModuleAccessor, param_type: u64, param_hash: u64) -> Option<Param> {
    if PARAMS.is_empty() {
        return None;
    }
    let owner = fighter_of(boma)?;
    let kind = utility::get_kind(&mut *owner);
    let params = PARAMS.iter().find(|params| params.kind == kind)?;
    let key = (param_type, param_hash);
    let value = *params.customs.iter()
        .filter_map(|(special, number, custom)| custom.get(&key).map(|value| (special, number, value)))
        .find(|(special, number, _)| custom_number(owner, **special) == **number)
        .map(|(_, _, value)| value)
        .or_else(|| params.every.get(&key))?;
    if is_custom(owner, params.fighter) {
        return None;
    }
    Some(value)
}

// claude code: Sm4sh custom specials, chosen per costume slot in config.toml ([customs.<fighter>],
// above). a slot's four numbers are neutral, side, up and down: 1 is the Divergence special, 2
// and 3 the fighter's two Sm4sh customs. a number nobody has ported yet stays 1, so a slot is free to
// ask for anything. an SSM's marked slot and a Dionysus clone (above) ignore their customs: the
// moveset owns them
//
// the switch is the game's own custom move system, the way LilyLambda found it
// (github.com/LilyLavender/ssbu-custom-moves-research): waza_customize(special, which one) writes each
// special's number into the fighter (FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_*_NO), then runs the
// fighter's WAZA_CUSTOMIZE_CONTROL function, which swaps status scripts. the game calls it on every
// fighter as it's made, with 1s (Miis aside); Divergence calls it again on the first frame of a match.
// so the numbers belong to the fighter, not the slot, and set_custom() can change them any time
//
// three things look at the numbers:
// - ACMD: custom_acmd! installs one script that runs the custom's version, or the usual one
// - params: the lists in vl.prc have a row per number, index number - 1 (romfs/fighter/<x>/param/vl.prcxml
//   fills rows 1 and 2, whole, since vanilla leaves them zeros). the game reads that row by itself, but
//   only in param_special_n, _s, _hi and _lw. any other list (an article's, a reflector's) always gives
//   row 0, so slot_param hands out those rows, read from the same romfs file when the plugin
//   is built (custom_params above)
// - statuses: Custom::statuses, called from the control function, swaps them with custom_status
//
// to port one, the fighter's lib.rs lists its customs:
//     config::register_customs(*FIGHTER_KIND_EXAMPLE, "example", CUSTOMS);
//     config::custom_params(*FIGHTER_KIND_EXAMPLE, "example", &[("param_example", Special::Neutral)], include_str!("../../../romfs/fighter/example/param/vl.prcxml"));
// (no custom_params call when its customs only change param_special_n, _s, _hi and _lw)
// and its scripts (the fighter's or an article's) go on its usual agents with custom_acmd!

#[derive(Clone, Copy, PartialEq)]
pub enum Special {
    Neutral = 0,
    Side = 1,
    Up = 2,
    Down = 3
}

impl Special {
    const ALL: [Special; 4] = [Special::Neutral, Special::Side, Special::Up, Special::Down];
    const NAMES: [&'static str; 4] = ["neutral", "side", "up", "down"];
}

// one ported custom. number: 2 or 3, as in the config. statuses: swaps the statuses it changes, one
// custom_status call each
pub struct Custom {
    pub name: &'static str,
    pub special: Special,
    pub number: u8,
    pub statuses: Option<unsafe fn(&mut L2CFighterCommon)>
}

// a fighter with ported customs
struct Registered {
    kind: i32,
    fighter: &'static str,
    customs: &'static [Custom]
}

static mut REGISTERED: Vec<Registered> = Vec::new();

impl Registered {
    fn custom(&self, special: Special, number: u8) -> Option<&'static Custom> {
        self.customs.iter().find(|custom| custom.special == special && custom.number == number)
    }
}

// a fighter's ported customs, from its lib.rs install
pub fn register_customs(kind: i32, fighter: &'static str, customs: &'static [Custom]) {
    unsafe {
        REGISTERED.retain(|other| other.kind != kind);
        REGISTERED.push(Registered { kind, fighter, customs });
    }
}

unsafe fn registered(fighter: *mut BattleObjectModuleAccessor) -> Option<&'static Registered> {
    let kind = utility::get_kind(&mut *fighter);
    REGISTERED.iter().find(|registered| registered.kind == kind)
}

// which custom this fighter, or the fighter that owns this weapon, plays for a special (1 = the usual).
// 1 for any fighter without ported customs
pub unsafe fn custom_number(boma: *mut BattleObjectModuleAccessor, special: Special) -> u8 {
    let owner = match fighter_of(boma) {
        Some(owner) if registered(owner).is_some() => owner,
        _ => return 1
    };
    let number = (*owner).get_int(*FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_N_NO + special as i32);
    number.max(0) as u8 + 1
}

// the game's waza_customize, at the same place in 13.0.4 and 13.0.5
#[skyline::from_offset(0x6dd2a0)]
fn waza_customize(lua_module: u64, status: i32, customize_to: i32);

// switches a special to one of its customs, or back to 1
pub unsafe fn set_custom(fighter: &mut L2CFighterCommon, special: Special, number: u8) {
    let boma = fighter.module_accessor;
    let lua_module = *(boma as *mut u64).add(0x190 / 8);
    let customize_to = special as i32 * 4 + number as i32 - 1;
    waza_customize(lua_module, *FIGHTER_STATUS_KIND_SPECIAL_N + special as i32, customize_to);
}

// what a status script was before a custom swapped it, to put back when the special changes again
struct Swapped {
    object: u32,
    special: Special,
    status: i32,
    line: i32,
    original: L2CValue
}

static mut SWAPPED: Vec<Swapped> = Vec::new();

// swaps one script of a status (line: LUA_SCRIPT_STATUS_FUNC_*), for Custom::statuses
pub unsafe fn custom_status(fighter: &mut L2CFighterCommon, special: Special, status: i32, line: i32, function: *const ()) {
    let object = (*fighter.battle_object).battle_object_id;
    let original = fighter.sv_get_status_func(&status.into(), &line.into());
    SWAPPED.push(Swapped { object, special, status, line, original });
    fighter.sv_set_status_func(status.into(), line.into(), &mut *(function as *mut skyline::libc::c_void));
}

// WAZA_CUSTOMIZE_CONTROL: waza_customize runs this once it has written the numbers
unsafe extern "C" fn customs_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let org = fighter.boma().get_int(*FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_ORG);
    let to = fighter.boma().get_int(*FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    let special = match Special::ALL.iter().find(|special| *FIGHTER_STATUS_KIND_SPECIAL_N + **special as i32 == org) {
        Some(special) => *special,
        None => return 0.into()
    };
    let object = (*fighter.battle_object).battle_object_id;
    // the newest swap first, so a script swapped twice ends on its first original
    while let Some(index) = SWAPPED.iter().rposition(|swapped| swapped.object == object && swapped.special == special) {
        let swapped = SWAPPED.remove(index);
        if swapped.original.val_type == smash::lib::L2CValueType::Pointer {
            fighter.sv_set_status_func(swapped.status.into(), swapped.line.into(), &mut *(swapped.original.get_ptr() as *mut skyline::libc::c_void));
        } else {
            fighter.sv_delete_status_func(&swapped.status.into(), &swapped.line.into());
        }
    }
    let number = (to - special as i32 * 4 + 1) as u8;
    let custom = registered(fighter.module_accessor).and_then(|registered| registered.custom(special, number));
    if let Some(statuses) = custom.and_then(|custom| custom.statuses) {
        statuses(fighter);
    }
    0.into()
}

// a match starts. the numbers wait for the first frame (frame), when the fighter's scripts are running
unsafe extern "C" fn customs_start(fighter: &mut L2CFighterCommon) {
    let registered = match registered(fighter.module_accessor) {
        Some(registered) if !is_custom(fighter.module_accessor, registered.fighter) => registered,
        _ => return
    };
    let object = (*fighter.battle_object).battle_object_id;
    SWAPPED.retain(|swapped| swapped.object != object);
    fighter.global_table[WAZA_CUSTOMIZE_CONTROL].assign(&L2CValue::Ptr(customs_control as *const () as _));
    VarModule::on_flag(fighter.battle_object, vars::common::instance::CUSTOMS_PENDING);
}

unsafe extern "C" fn customs_frame(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.battle_object, vars::common::instance::CUSTOMS_PENDING) {
        return;
    }
    VarModule::off_flag(fighter.battle_object, vars::common::instance::CUSTOMS_PENDING);
    let registered = match registered(fighter.module_accessor) {
        Some(registered) => registered,
        None => return
    };
    let slot = fighter.boma().get_int(*FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let request = config().custom_slots.iter()
        .find(|request| request.fighter == registered.fighter && request.slot as i32 == slot);
    if let Some(request) = request {
        for special in Special::ALL {
            let wanted = request.numbers[special as usize];
            if registered.custom(special, wanted).is_some() {
                set_custom(fighter, special, wanted);
            }
        }
    }
}

pub fn install_customs() {
    smashline::Agent::new("fighter")
        .on_start(customs_start)
        .on_line(smashline::Main, customs_frame)
        .install();
}

// runs vanilla's version of a script. the game's scripts take a Variadic after the agent, which
// smashline's original_acmd leaves out of its type: without one they read whatever is in that register
// and crash
pub unsafe fn vanilla_acmd(agent: &mut smash::lua2cpp::L2CAgentBase, name: &str) {
    let script = smashline::original_acmd(agent, smashline::Hash40::new(name));
    let script: unsafe extern "C" fn(&mut smash::lua2cpp::L2CAgentBase, &mut Variadic) = std::mem::transmute(script);
    script(agent, &mut Variadic::new())
}

// one script of a special with customs: custom_acmd!(agent, Special::Up, "game_specialhi", 2 => a, 3 => b)
// runs a or b while that custom is on. anything else runs vanilla's script, or the one after `_ =>`:
// Divergence's own where it has one, acmd_stub where vanilla has none. Priority::High, so it takes the
// place of any other script of that name on the agent
macro_rules! custom_acmd {
    ($agent:expr, $special:expr, $name:literal, $($number:literal => $custom:expr),+) => {{
        unsafe extern "C" fn vanilla(agent: &mut L2CAgentBase) {
            $crate::config::vanilla_acmd(agent, $name)
        }
        $crate::config::custom_acmd!($agent, $special, $name, $($number => $custom),+, _ => vanilla)
    }};
    ($agent:expr, $special:expr, $name:literal, $($number:literal => $custom:expr),+, _ => $usual:expr) => {{
        unsafe extern "C" fn script(agent: &mut L2CAgentBase) {
            match $crate::config::custom_number(agent.module_accessor, $special) {
                $($number => $custom(agent),)+
                _ => $usual(agent)
            }
        }
        $agent.acmd($name, script, Priority::High);
    }};
}

pub(crate) use custom_acmd;

// logs what became of every slot in the config, once all fighters are in
pub fn report_customs() {
    for request in &config().custom_slots {
        let (fighter, slot) = (&request.fighter, request.slot);
        if is_marked(fighter, slot) {
            crate::log(&format!("[Divergence] customs: {} c{:02} is a custom moveset's slot, its customs are ignored", fighter, slot));
            continue;
        }
        let registered = unsafe { REGISTERED.iter().find(|registered| registered.fighter == fighter.as_str()) };
        for special in Special::ALL {
            let number = request.numbers[special as usize];
            if number == 1 {
                continue;
            }
            match registered.and_then(|registered| registered.custom(special, number)) {
                Some(custom) => crate::log(&format!("[Divergence] customs: {} c{:02} {} {} is {}", fighter, slot, Special::NAMES[special as usize], number, custom.name)),
                None => crate::log(&format!("[Divergence] customs: {} c{:02} {} {} isn't ported, it stays default", fighter, slot, Special::NAMES[special as usize], number))
            }
        }
    }
}
