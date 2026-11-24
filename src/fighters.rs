// hdr: one module per fighter behind a feature of its name, each with HDR's acmd / opff / status
// layout (fighters/<name>/src)
#[path = "../fighters/common/src/lib.rs"]
mod common;
#[cfg(feature = "bayonetta")]
#[path = "../fighters/bayonetta/src/lib.rs"]
mod bayonetta;
#[cfg(feature = "brave")]
#[path = "../fighters/brave/src/lib.rs"]
mod brave;
#[cfg(feature = "buddy")]
#[path = "../fighters/buddy/src/lib.rs"]
mod buddy;
#[cfg(feature = "captain")]
#[path = "../fighters/captain/src/lib.rs"]
mod captain;
#[cfg(feature = "chrom")]
#[path = "../fighters/chrom/src/lib.rs"]
mod chrom;
#[cfg(feature = "daisy")]
#[path = "../fighters/daisy/src/lib.rs"]
mod daisy;
#[cfg(feature = "dedede")]
#[path = "../fighters/dedede/src/lib.rs"]
mod dedede;
#[cfg(feature = "demon")]
#[path = "../fighters/demon/src/lib.rs"]
mod demon;
#[cfg(feature = "diddy")]
#[path = "../fighters/diddy/src/lib.rs"]
mod diddy;
#[cfg(feature = "dolly")]
#[path = "../fighters/dolly/src/lib.rs"]
mod dolly;
#[cfg(feature = "donkey")]
#[path = "../fighters/donkey/src/lib.rs"]
mod donkey;
#[cfg(feature = "duckhunt")]
#[path = "../fighters/duckhunt/src/lib.rs"]
mod duckhunt;
#[cfg(feature = "edge")]
#[path = "../fighters/edge/src/lib.rs"]
mod edge;
#[cfg(feature = "eflame")]
#[path = "../fighters/eflame/src/lib.rs"]
mod eflame;
#[cfg(feature = "elight")]
#[path = "../fighters/elight/src/lib.rs"]
mod elight;
#[cfg(feature = "falco")]
#[path = "../fighters/falco/src/lib.rs"]
mod falco;
#[cfg(feature = "fox")]
#[path = "../fighters/fox/src/lib.rs"]
mod fox;
#[cfg(feature = "gamewatch")]
#[path = "../fighters/gamewatch/src/lib.rs"]
mod gamewatch;
#[cfg(feature = "ganon")]
#[path = "../fighters/ganon/src/lib.rs"]
mod ganon;
#[cfg(feature = "gaogaen")]
#[path = "../fighters/gaogaen/src/lib.rs"]
mod gaogaen;
#[cfg(feature = "gekkouga")]
#[path = "../fighters/gekkouga/src/lib.rs"]
mod gekkouga;
#[cfg(feature = "iceclimber")]
#[path = "../fighters/iceclimber/src/lib.rs"]
mod iceclimber;
#[cfg(feature = "ike")]
#[path = "../fighters/ike/src/lib.rs"]
mod ike;
#[cfg(feature = "inkling")]
#[path = "../fighters/inkling/src/lib.rs"]
mod inkling;
#[cfg(feature = "jack")]
#[path = "../fighters/jack/src/lib.rs"]
mod jack;
#[cfg(feature = "kamui")]
#[path = "../fighters/kamui/src/lib.rs"]
mod kamui;
#[cfg(feature = "ken")]
#[path = "../fighters/ken/src/lib.rs"]
mod ken;
#[cfg(feature = "kirby")]
#[path = "../fighters/kirby/src/lib.rs"]
mod kirby;
#[cfg(feature = "koopa")]
#[path = "../fighters/koopa/src/lib.rs"]
mod koopa;
#[cfg(feature = "koopajr")]
#[path = "../fighters/koopajr/src/lib.rs"]
mod koopajr;
#[cfg(feature = "krool")]
#[path = "../fighters/krool/src/lib.rs"]
mod krool;
#[cfg(feature = "link")]
#[path = "../fighters/link/src/lib.rs"]
mod link;
#[cfg(feature = "littlemac")]
#[path = "../fighters/littlemac/src/lib.rs"]
mod littlemac;
#[cfg(feature = "lucario")]
#[path = "../fighters/lucario/src/lib.rs"]
mod lucario;
#[cfg(feature = "lucas")]
#[path = "../fighters/lucas/src/lib.rs"]
mod lucas;
#[cfg(feature = "lucina")]
#[path = "../fighters/lucina/src/lib.rs"]
mod lucina;
#[cfg(feature = "luigi")]
#[path = "../fighters/luigi/src/lib.rs"]
mod luigi;
#[cfg(feature = "mario")]
#[path = "../fighters/mario/src/lib.rs"]
mod mario;
#[cfg(feature = "mariod")]
#[path = "../fighters/mariod/src/lib.rs"]
mod mariod;
#[cfg(feature = "marth")]
#[path = "../fighters/marth/src/lib.rs"]
mod marth;
#[cfg(feature = "master")]
#[path = "../fighters/master/src/lib.rs"]
mod master;
#[cfg(feature = "metaknight")]
#[path = "../fighters/metaknight/src/lib.rs"]
mod metaknight;
#[cfg(feature = "mewtwo")]
#[path = "../fighters/mewtwo/src/lib.rs"]
mod mewtwo;
#[cfg(feature = "miifighter")]
#[path = "../fighters/miifighter/src/lib.rs"]
mod miifighter;
#[cfg(feature = "miigunner")]
#[path = "../fighters/miigunner/src/lib.rs"]
mod miigunner;
#[cfg(feature = "miiswordsman")]
#[path = "../fighters/miiswordsman/src/lib.rs"]
mod miiswordsman;
#[cfg(feature = "murabito")]
#[path = "../fighters/murabito/src/lib.rs"]
mod murabito;
#[cfg(feature = "ness")]
#[path = "../fighters/ness/src/lib.rs"]
mod ness;
#[cfg(feature = "packun")]
#[path = "../fighters/packun/src/lib.rs"]
mod packun;
#[cfg(feature = "pacman")]
#[path = "../fighters/pacman/src/lib.rs"]
mod pacman;
#[cfg(feature = "palutena")]
#[path = "../fighters/palutena/src/lib.rs"]
mod palutena;
#[cfg(feature = "peach")]
#[path = "../fighters/peach/src/lib.rs"]
mod peach;
#[cfg(feature = "pfushigisou")]
#[path = "../fighters/pfushigisou/src/lib.rs"]
mod pfushigisou;
#[cfg(feature = "pichu")]
#[path = "../fighters/pichu/src/lib.rs"]
mod pichu;
#[cfg(feature = "pickel")]
#[path = "../fighters/pickel/src/lib.rs"]
mod pickel;
#[cfg(feature = "pikachu")]
#[path = "../fighters/pikachu/src/lib.rs"]
mod pikachu;
#[cfg(feature = "pikmin")]
#[path = "../fighters/pikmin/src/lib.rs"]
mod pikmin;
#[cfg(feature = "pit")]
#[path = "../fighters/pit/src/lib.rs"]
mod pit;
#[cfg(feature = "pitb")]
#[path = "../fighters/pitb/src/lib.rs"]
mod pitb;
#[cfg(feature = "plizardon")]
#[path = "../fighters/plizardon/src/lib.rs"]
mod plizardon;
#[cfg(feature = "purin")]
#[path = "../fighters/purin/src/lib.rs"]
mod purin;
#[cfg(feature = "pzenigame")]
#[path = "../fighters/pzenigame/src/lib.rs"]
mod pzenigame;
#[cfg(feature = "reflet")]
#[path = "../fighters/reflet/src/lib.rs"]
mod reflet;
#[cfg(feature = "richter")]
#[path = "../fighters/richter/src/lib.rs"]
mod richter;
#[cfg(feature = "ridley")]
#[path = "../fighters/ridley/src/lib.rs"]
mod ridley;
#[cfg(feature = "robot")]
#[path = "../fighters/robot/src/lib.rs"]
mod robot;
#[cfg(feature = "rockman")]
#[path = "../fighters/rockman/src/lib.rs"]
mod rockman;
#[cfg(feature = "rosetta")]
#[path = "../fighters/rosetta/src/lib.rs"]
mod rosetta;
#[cfg(feature = "roy")]
#[path = "../fighters/roy/src/lib.rs"]
mod roy;
#[cfg(feature = "ryu")]
#[path = "../fighters/ryu/src/lib.rs"]
mod ryu;
#[cfg(feature = "samus")]
#[path = "../fighters/samus/src/lib.rs"]
mod samus;
#[cfg(feature = "samusd")]
#[path = "../fighters/samusd/src/lib.rs"]
mod samusd;
#[cfg(feature = "sheik")]
#[path = "../fighters/sheik/src/lib.rs"]
mod sheik;
#[cfg(feature = "shizue")]
#[path = "../fighters/shizue/src/lib.rs"]
mod shizue;
#[cfg(feature = "shulk")]
#[path = "../fighters/shulk/src/lib.rs"]
mod shulk;
#[cfg(feature = "simon")]
#[path = "../fighters/simon/src/lib.rs"]
mod simon;
#[cfg(feature = "snake")]
#[path = "../fighters/snake/src/lib.rs"]
mod snake;
#[cfg(feature = "sonic")]
#[path = "../fighters/sonic/src/lib.rs"]
mod sonic;
#[cfg(feature = "szerosuit")]
#[path = "../fighters/szerosuit/src/lib.rs"]
mod szerosuit;
#[cfg(feature = "tantan")]
#[path = "../fighters/tantan/src/lib.rs"]
mod tantan;
#[cfg(feature = "toonlink")]
#[path = "../fighters/toonlink/src/lib.rs"]
mod toonlink;
#[cfg(feature = "trail")]
#[path = "../fighters/trail/src/lib.rs"]
mod trail;
#[cfg(feature = "wario")]
#[path = "../fighters/wario/src/lib.rs"]
mod wario;
#[cfg(feature = "wiifit")]
#[path = "../fighters/wiifit/src/lib.rs"]
mod wiifit;
#[cfg(feature = "wolf")]
#[path = "../fighters/wolf/src/lib.rs"]
mod wolf;
#[cfg(feature = "yoshi")]
#[path = "../fighters/yoshi/src/lib.rs"]
mod yoshi;
#[cfg(feature = "younglink")]
#[path = "../fighters/younglink/src/lib.rs"]
mod younglink;
#[cfg(feature = "zelda")]
#[path = "../fighters/zelda/src/lib.rs"]
mod zelda;

macro_rules! install_fighters {
    ($func:ident; $($name:ident = $feature:expr),*) => {{
        $(
            #[cfg(feature = $feature)]
            { $name::$func() }
        )*
    }}
}

pub fn install() {
    crate::movesets::scan();
    common::install();

    install_fighters! {
        install;
        bayonetta = "bayonetta",
        brave = "brave",
        buddy = "buddy",
        captain = "captain",
        chrom = "chrom",
        daisy = "daisy",
        dedede = "dedede",
        demon = "demon",
        diddy = "diddy",
        dolly = "dolly",
        donkey = "donkey",
        duckhunt = "duckhunt",
        edge = "edge",
        eflame = "eflame",
        elight = "elight",
        falco = "falco",
        fox = "fox",
        gamewatch = "gamewatch",
        ganon = "ganon",
        gaogaen = "gaogaen",
        gekkouga = "gekkouga",
        iceclimber = "iceclimber",
        ike = "ike",
        inkling = "inkling",
        jack = "jack",
        kamui = "kamui",
        ken = "ken",
        kirby = "kirby",
        koopa = "koopa",
        koopajr = "koopajr",
        krool = "krool",
        link = "link",
        littlemac = "littlemac",
        lucario = "lucario",
        lucas = "lucas",
        lucina = "lucina",
        luigi = "luigi",
        mario = "mario",
        mariod = "mariod",
        marth = "marth",
        master = "master",
        metaknight = "metaknight",
        mewtwo = "mewtwo",
        miifighter = "miifighter",
        miigunner = "miigunner",
        miiswordsman = "miiswordsman",
        murabito = "murabito",
        ness = "ness",
        packun = "packun",
        pacman = "pacman",
        palutena = "palutena",
        peach = "peach",
        pfushigisou = "pfushigisou",
        pichu = "pichu",
        pickel = "pickel",
        pikachu = "pikachu",
        pikmin = "pikmin",
        pit = "pit",
        pitb = "pitb",
        plizardon = "plizardon",
        purin = "purin",
        pzenigame = "pzenigame",
        reflet = "reflet",
        richter = "richter",
        ridley = "ridley",
        robot = "robot",
        rockman = "rockman",
        rosetta = "rosetta",
        roy = "roy",
        ryu = "ryu",
        samus = "samus",
        samusd = "samusd",
        sheik = "sheik",
        shizue = "shizue",
        shulk = "shulk",
        simon = "simon",
        snake = "snake",
        sonic = "sonic",
        szerosuit = "szerosuit",
        tantan = "tantan",
        toonlink = "toonlink",
        trail = "trail",
        wario = "wario",
        wiifit = "wiifit",
        wolf = "wolf",
        yoshi = "yoshi",
        younglink = "younglink",
        zelda = "zelda"
    }
}
