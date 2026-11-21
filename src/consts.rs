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
            pub const IN_SPECIAL: i32 = 0x0014; // claude code: a special (and its follow-up statuses) is running
            pub const SLIDE_OFF_AIR: i32 = 0x0015; // claude code: a ground move slid off and keeps going in the air
            pub const PARRY_SHIELD_HELD: i32 = 0x0016; // claude code: shield still held since a parry, so no startup parry yet
            pub const SLIDE_OFF_THROWN: i32 = 0x0017; // claude code: being thrown by a fighter who slid off an edge

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

            // floats
            pub const ECB_BOTTOM_Y_OFFSET: i32 = 0x0006;
            pub const CURR_DASH_SPEED: i32 = 0x0007;
            pub const ECB_CENTER_Y_OFFSET: i32 = 0x0008;
            pub const SLIDE_OFF_SPEED_Y: i32 = 0x0009; // claude code: fall speed of a slid-off move, kept across its statuses

        }
        pub mod status {
            // flags
            pub const DISABLE_ECB_SHIFT: i32 = 0x1001;
            pub const IS_DASH_TO_RUN_FRAME: i32 = 0x1002;
            pub const IS_AFTER_DASH_TO_RUN_FRAME: i32 = 0x1003;
            pub const CHECK_HOLD_INPUT: i32 = 0x1011;
            pub const FORCE_OPPONENT_KNOCKDOWN: i32 = 0x1012; // claude code: fighters this status hits get FORCE_KNOCKDOWN

            // ints

            // floats
            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1001;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1002;
        }
    }
}

pub mod statuses {}

pub mod articles {}