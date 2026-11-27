// hdr: HDR's L2CFighterCommon global table indexes (dynamic/src/consts.rs)
pub mod globals {
    // 0x1
    pub const FIGHTER_KIND: i32 = 0x2;
    pub const OBJECT_ID: i32 = 0x3;
    pub const FIGHTER: i32 = 0x4;
    pub const MODULE_ACCESSOR: i32 = 0x5;
    // 0x6
    pub const INIT_STATUS_FUNC: i32 = 0x7;
    pub const IS_STOPPING: i32 = 0x8;
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9;
    pub const PREV_STATUS_KIND: i32 = 0xA;
    pub const STATUS_KIND: i32 = 0xB;
    pub const STATUS_COUNT: i32 = 0xC;
    // 0xD
    pub const CURRENT_FRAME: i32 = 0xE;
    pub const CURRENT_FRAME2: i32 = 0xF;
    // 0x10
    // 0x11 func ptr
    // 0x12
    pub const SUB_STATUS3: i32 = 0x13;
    pub const SUB_STATUS2: i32 = 0x14;
    pub const SUB_STATUS: i32 = 0x15;
    pub const SITUATION_KIND: i32 = 0x16;
    pub const PREV_SITUATION_KIND: i32 = 0x17;
    pub const PREV_STATUS_FRAME: i32 = 0x18;
    // 0x19
    pub const STICK_X: i32 = 0x1A;
    pub const STICK_Y: i32 = 0x1B;
    pub const FLICK_X: i32 = 0x1C;
    pub const FLICK_Y: i32 = 0x1D;
    pub const FLICK_Y_DIR: i32 = 0x1E;
    pub const PAD_FLAG: i32 = 0x1F;
    pub const CMD_CAT1: i32 = 0x20;
    pub const CMD_CAT2: i32 = 0x21;
    pub const CMD_CAT3: i32 = 0x22;
    pub const CMD_CAT4: i32 = 0x23;
    // 0x24
    // 0x25
    // 0x26
    // 0x27
    // 0x28 some substatus
    pub const DASH_CALLBACK: i32 = 0x29;
    // 0x2A
    pub const CUSTOM_ROUTINE: i32 = 0x2B;
    // 0x2C
    // 0x2D
    // 0x2E
    // 0x2F
    // 0x30
    // 0x31
    // 0x32 some substatus
    pub const USE_SPECIAL_N_CALLBACK: i32 = 0x38;
    pub const USE_SPECIAL_S_CALLBACK: i32 = 0x39;
    pub const USE_SPECIAL_HI_CALLBACK: i32 = 0x3A;
    pub const USE_SPECIAL_LW_CALLBACK: i32 = 0x3B;
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C;
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D;
    pub const STATUS_CHANGE_CALLBACK: i32 = 0x3E;
    pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
    pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;
}

// hdr: VarModule IDs, laid out the way HDR lays out theirs (instance / status, flags / ints /
// floats). the ones without a claude code note are HDR's own
pub mod vars {
    pub mod common {
        pub mod instance {
            // flags
            pub const SPECIAL_WALL_JUMP: i32 = 0x0004;
            pub const IS_LATE_PIVOT: i32 = 0x0007;
            pub const CAN_PERFECT_PIVOT: i32 = 0x0008;
            pub const IS_SMASH_TURN: i32 = 0x0009;
            pub const FORCE_KNOCKDOWN: i32 = 0x000B;
            pub const IS_GETTING_POSITION_FOR_ECB: i32 = 0x000D;
            // claude code: universal no special fall, "used this airtime" locks
            pub const SPECIAL_N_USED: i32 = 0x0010;
            pub const SPECIAL_S_USED: i32 = 0x0011;
            pub const SPECIAL_HI_USED: i32 = 0x0012;
            pub const SPECIAL_LW_USED: i32 = 0x0013;
            pub const IN_SPECIAL: i32 = 0x0014; // claude code: a special started on the ground (and its follow-up statuses) is running
            pub const SLIDE_OFF_AIR: i32 = 0x0015; // claude code: a ground move slid off and keeps going in the air
            pub const PARRY_SHIELD_HELD: i32 = 0x0016; // claude code: shield still held since a parry, so no startup parry yet
            pub const SLIDE_OFF_THROWN: i32 = 0x0017; // claude code: being thrown by a fighter who slid off an edge
            pub const CUSTOMS_PENDING: i32 = 0x0018; // claude code: Sm4sh customs not applied yet this match (src/config.rs)
            pub const TETHER_WALL: i32 = 0x0019; // claude code: a tether is latched onto a wall (general_statuses/lasso.rs)
            // claude code: faux air mode (fighters/common/src/modes.rs)
            pub const FAUX_AIR_GROUND_STATUS: i32 = 0x0020; // in a ground status, which always sees the ground
            pub const FAUX_AIR_AIRBORNE: i32 = 0x0021; // a ground status is off the ground, faux gravity on
            pub const FAUX_AIR_DIVE: i32 = 0x0022; // faux fast-fall
            pub const FAUX_AIR_REAL_SITUATION: i32 = 0x0023; // StatusModule::situation_kind tells the truth while on
            pub const FAUX_AIR_PASSING: i32 = 0x0025; // dropping through a platform, no landing until clear of it
            pub const FAUX_AIR_FROM_HITSTUN: i32 = 0x0026; // a ground status just took over in the air out of hitstun
            // claude code: aura mode (fighters/common/src/modes.rs)
            pub const AURA_WEAPON_SCALED: i32 = 0x0030; // a weapon's size has had its Aura applied
            pub const AURA_IN_MOVE: i32 = 0x0031; // the current status is an attack or special, or follows on from one
            // claude code: wall jump lock override (fighters/common/src/general_statuses/walljump.rs)
            pub const SPECIAL_N_AIR_USED: i32 = 0x0040; // started in the air since landing or the last wall jump
            pub const SPECIAL_S_AIR_USED: i32 = 0x0041;
            pub const SPECIAL_HI_AIR_USED: i32 = 0x0042;
            pub const SPECIAL_LW_AIR_USED: i32 = 0x0043;
            pub const SPECIAL_N_REFRESHED: i32 = 0x0044; // given back by a wall jump, whatever the fighter's own lock says
            pub const SPECIAL_S_REFRESHED: i32 = 0x0045;
            pub const SPECIAL_HI_REFRESHED: i32 = 0x0046;
            pub const SPECIAL_LW_REFRESHED: i32 = 0x0047;
            pub const SPECIALS_FORCED: i32 = 0x0048; // inside the air special check, refreshed specials let through
            // claude code: shield breaks (fighters/common/src/general_statuses/shield.rs)
            pub const SHIELD_BROKEN: i32 = 0x0050; // shield broke this stock, no shield until respawn

            // ints
            pub const LEFT_STICK_FLICK_X: i32 = 0x0003;
            pub const LEFT_STICK_FLICK_Y: i32 = 0x0004;
            pub const RIGHT_STICK_FLICK_X: i32 = 0x0005;
            pub const RIGHT_STICK_FLICK_Y: i32 = 0x0006;
            pub const PARRY_WINDOW_FRAME: i32 = 0x0008; // claude code: frames left in the open parry window
            pub const LAST_SPECIAL_KIND: i32 = 0x0007; // claude code: generic SPECIAL_* status last started, 0 = none
            pub const DJC_KINETIC: i32 = 0x0009; // claude code: kinetic a DJC'd non-aerial goes back to when its jump ends
            pub const DJC_FRAMES_LEFT: i32 = 0x000A; // claude code: frames left of a DJC'd non-aerial's double jump
            pub const STATUS_REQUESTS_THIS_FRAME: i32 = 0x000B; // claude code: status requests so far this frame (crash guard, lua_bind_hook.rs)
            pub const TETHER_WALL_COUNT: i32 = 0x000C; // claude code: wall latches since landing (general_statuses/lasso.rs)
            // claude code: held SDI (fighters/common/src/general_statuses/mod.rs)
            pub const SDI_COUNT: i32 = 0x000E; // SDI nudges so far this hitlag
            pub const SDI_WAIT: i32 = 0x000F; // frames until the next nudge
            pub const AURA_EFFECT_L: i32 = 0x0010; // claude code: aura mode's left effect handle, 0 = none
            pub const AURA_EFFECT_R: i32 = 0x0011; // claude code: aura mode's right effect handle, 0 = none
            pub const AURA_SPOT: i32 = 0x0012; // claude code: aura mode, where the effect goes (modes.rs, SPOT_*), 0 = not worked out yet

            // floats
            pub const ECB_BOTTOM_Y_OFFSET: i32 = 0x0006;
            pub const CURR_DASH_SPEED: i32 = 0x0007;
            pub const ECB_CENTER_Y_OFFSET: i32 = 0x0008;
            pub const SLIDE_OFF_SPEED_Y: i32 = 0x0009; // claude code: fall speed of a slid-off move, kept across its statuses
            pub const TETHER_WALL_X: i32 = 0x000A; // claude code: where a tether is latched (general_statuses/lasso.rs)
            pub const TETHER_WALL_Y: i32 = 0x000B;
            pub const TETHER_WALL_LR: i32 = 0x000C; // claude code: the way to the wall
            pub const FAUX_AIR_SPEED_Y: i32 = 0x0020; // claude code: faux vertical speed, kept across status changes
            // claude code: aura mode (fighters/common/src/modes.rs)
            pub const AURA_POWER: i32 = 0x0030; // a fighter's Aura multiplier, or a weapon's (taken from its owner), 0 = not worked out yet
            pub const AURA_STOCK_MUL: i32 = 0x0031; // Stock Aura multiplier now (eases toward the target)
            pub const AURA_STOCK_TARGET: i32 = 0x0032; // Stock Aura multiplier the current stock standings call for
            pub const AURA_STOCK_STEP: i32 = 0x0033; // how far the Stock Aura multiplier moves per frame toward the target

        }
        pub mod status {
            // flags
            pub const DISABLE_ECB_SHIFT: i32 = 0x1001;
            pub const IS_DASH_TO_RUN_FRAME: i32 = 0x1002;
            pub const IS_AFTER_DASH_TO_RUN_FRAME: i32 = 0x1003;
            pub const CHECK_HOLD_INPUT: i32 = 0x1011;
            pub const FORCE_OPPONENT_KNOCKDOWN: i32 = 0x1012; // claude code: fighters this status hits get FORCE_KNOCKDOWN
            pub const EDGE_STOP_SLIPPED: i32 = 0x1013; // claude code: this status asked to stop at edges and got plain ground correction instead
            pub const SLID_OFF_OWN_STATUS: i32 = 0x1014; // claude code: it slid off and runs on in the air under slide-off physics (opff.rs)
            pub const AURA_STATUS_ATTACKED: i32 = 0x1015; // claude code: this fighter-specific status has had a hitbox out, so it's a move (modes.rs)
            pub const BOUNCE_RELEASED: i32 = 0x1016; // claude code: this bounce's DI and ASDI are done (general_statuses/damage.rs)

            // ints

            // floats
            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1001;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1002;
        }
    }
}

pub mod statuses {}

pub mod articles {}
