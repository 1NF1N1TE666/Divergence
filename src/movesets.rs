// claude code: custom moveset support. a single-slot moveset (SSM) marks its slots with a .marker file in
// fighter/<name>/model/body/cXX/ of its mod folder, the same signal Ultimate S and SSBEXO read. Divergence's
// fighter edits run on every slot except marked ones, so a marked slot plays its own moveset on top of
// vanilla, and skins on added slots still get Divergence. universal mechanics (common) stay on every slot.
//
// two halves:
// - smashline scripts: make each fighter or article agent with `movesets::agent` instead of
//   `Agent::new`, and smashline itself skips the marked slots (acmd, status, opff and on_start alike)
// - everything smashline doesn't route (skyline hooks, common code that checks for one fighter):
//   `movesets::is_custom` before running the edit
use smash::app::{lua_bind::*, *};
use smash::lib::lua_const::*;
use std::collections::HashMap;

// slots marked per fighter folder name ("lucario", "littlemac"...), filled once by scan()
static mut MARKED: Option<HashMap<String, Vec<usize>>> = None;

// costume slots run from c00 to c255
const SLOT_COUNT: usize = 256;

// finds every marked slot in an enabled mod folder (ARCropolis skips folders starting with a dot)
pub fn scan() {
    let mut marked: HashMap<String, Vec<usize>> = HashMap::new();
    let mods = match std::fs::read_dir("sd:/ultimate/mods") {
        Ok(entries) => entries,
        Err(_) => {
            unsafe {
                MARKED = Some(marked);
            }
            return;
        }
    };
    for folder in mods.flatten() {
        if folder.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        let fighters = match std::fs::read_dir(folder.path().join("fighter")) {
            Ok(entries) => entries,
            Err(_) => continue
        };
        for fighter in fighters.flatten() {
            let name = fighter.file_name().to_string_lossy().into_owned();
            let bodies = match std::fs::read_dir(fighter.path().join("model/body")) {
                Ok(entries) => entries,
                Err(_) => continue
            };
            let mut moveset = vec![];
            for body in bodies.flatten() {
                let slot = body.file_name().to_string_lossy()
                    .strip_prefix('c')
                    .and_then(|number| number.parse::<usize>().ok());
                let slot = match slot {
                    Some(slot) if slot < SLOT_COUNT => slot,
                    _ => continue
                };
                let has_marker = match std::fs::read_dir(body.path()) {
                    Ok(files) => files.flatten().any(|file| file.path().extension().map_or(false, |ext| ext == "marker")),
                    Err(_) => false
                };
                if has_marker {
                    moveset.push(slot);
                    let slots = marked.entry(name.clone()).or_insert_with(Vec::new);
                    if !slots.contains(&slot) {
                        slots.push(slot);
                    }
                }
            }
            if !moveset.is_empty() {
                moveset.sort();
                println!("[Divergence] custom moveset for {} in {}: slots {:?}", name, folder.file_name().to_string_lossy(), moveset);
            }
        }
    }
    for slots in marked.values_mut() {
        slots.sort();
    }
    unsafe {
        MARKED = Some(marked);
    }
}

fn marked(fighter: &str) -> &'static [usize] {
    unsafe {
        MARKED.as_ref()
            .and_then(|marked| marked.get(fighter))
            .map_or(&[], |slots| slots.as_slice())
    }
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
unsafe fn fighter_of(boma: *mut BattleObjectModuleAccessor) -> Option<*mut BattleObjectModuleAccessor> {
    let category = utility::get_category(&mut *boma);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        return Some(boma);
    }
    if category != *BATTLE_OBJECT_CATEGORY_WEAPON {
        return None;
    }
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
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
    let slot = WorkModule::get_int(owner, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    slot >= 0 && marked.contains(&(slot as usize))
}

// claude code: a fighter's params, slotted. ARCropolis patches a param file for every slot, so Divergence's
// per-fighter param patches live with each fighter instead (fighters/<name>/param/: vl.prcxml for its
// vl.prc, fighter_param.prcxml for its row of fighter_param_table), are built into the plugin and are
// handed out by the get_param hooks (function_hooks/params.rs) on unmarked slots only. Ultimate S and
// SSBEXO slot params through the same hooks (via lib_paramconfig); here the prcxml stays the source
#[derive(Clone, Copy)]
pub enum Param {
    Int(i32),
    Float(f32)
}

// (FIGHTER_KIND, folder name, (param, field) -> value), one per fighter that registered its params.
// a fighter attribute (fighter_param.prcxml) is keyed (attribute, 0), the way the game asks for it
static mut PARAMS: Vec<(i32, &'static str, HashMap<(u64, u64), Param>)> = Vec::new();

// a prcxml hash: a name, or a raw hash written as 0x...
fn prc_hash(text: &str) -> Option<u64> {
    match text.strip_prefix("0x") {
        Some(hex) => u64::from_str_radix(hex, 16).ok(),
        None => Some(smash::hash40(text))
    }
}

// the text between `<tag ... attribute="` and the next quote
fn attribute<'a>(line: &'a str, attribute: &str) -> Option<&'a str> {
    let start = line.find(&format!("{}=\"", attribute))? + attribute.len() + 2;
    line[start..].split('"').next()
}

// reads a prcxml into params: every int and float at the top level (keyed with field 0), in a
// top-level struct, or in index 0 of a top-level list (the entry the game reads; the other indexes
// are dummies). anything nested deeper is skipped
fn parse_prcxml(text: &str, params: &mut HashMap<(u64, u64), Param>) {
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
            group = attribute(line, "hash").and_then(prc_hash);
            in_list = line.starts_with("<list");
            active = !in_list;
        } else if line.starts_with("<struct index=") {
            in_entry = true;
            active = attribute(line, "index") == Some("0");
        } else if line.starts_with("</struct") && in_entry {
            in_entry = false;
            active = false;
        } else if line.starts_with("</struct") || line.starts_with("</list") {
            group = None;
            active = false;
        } else if line.starts_with("<float ") || line.starts_with("<int ") {
            let field = match attribute(line, "hash").and_then(prc_hash) {
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
pub fn params(kind: i32, fighter: &'static str, prcxmls: &[&str]) {
    let mut params = HashMap::new();
    for prcxml in prcxmls {
        parse_prcxml(prcxml, &mut params);
    }
    println!("[Divergence] {} params: {}", fighter, params.len());
    unsafe {
        PARAMS.retain(|(registered, _, _)| *registered != kind);
        PARAMS.push((kind, fighter, params));
    }
}

// true when this fighter kind registered params
pub fn has_params(kind: i32) -> bool {
    unsafe { PARAMS.iter().any(|(registered, _, _)| *registered == kind) }
}

// Divergence's value for this param, if the fighter (or the weapon's owner) has one and isn't on a custom
// moveset's slot
pub unsafe fn slot_param(boma: *mut BattleObjectModuleAccessor, param_type: u64, param_hash: u64) -> Option<Param> {
    if PARAMS.is_empty() {
        return None;
    }
    let owner = fighter_of(boma)?;
    let kind = utility::get_kind(&mut *owner);
    let (_, fighter, params) = PARAMS.iter().find(|(registered, _, _)| *registered == kind)?;
    let value = *params.get(&(param_type, param_hash))?;
    if is_custom(owner, fighter) {
        return None;
    }
    Some(value)
}
