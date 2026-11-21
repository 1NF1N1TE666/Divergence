#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(unexpected_cfgs)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]
#![feature(proc_macro_hygiene)]
#![feature(seek_stream_len)]
#![feature(repr_simd)]
#![feature(simd_ffi)]

extern crate skyline;
extern crate smash;
#[macro_use] extern crate modular_bitfield;
#[macro_use] extern crate smash_script;

pub mod offsets;
pub mod consts;
pub mod ext;
pub mod movesets;
pub use offsets::*;
mod fighters;

#[unsafe(no_mangle)]
pub fn smashline_install() {
    fighters::install();
}

#[skyline::main(name = "divergence")]
pub fn main() {
    init();
    fighters::install();
}

pub fn init() {
    offsets::init();
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>"
            }
        };

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);
        show_error(
            69,
            "Skyline plugin as panicked! Please open the details and send a screenshot to the developer, then close the game.\n",
            err_msg.as_str()
        );
    }));
}

fn show_error(code: u32, message: &str, details: &str) {
    use skyline::nn::{err, settings};

    let mut message_bytes = String::from(message).into_bytes();
    let mut details_bytes = String::from(details).into_bytes();

    if message_bytes.len() >= 2048 {
        message_bytes.truncate(2044);
        message_bytes.append(&mut String::from("...\0").into_bytes());
    }

    if details_bytes.len() >= 2048 {
        details_bytes.truncate(2044);
        details_bytes.append(&mut String::from("...\0").into_bytes());
    }

    unsafe {
        let error = err::ApplicationErrorArg::new_with_messages(
            code,
            core::str::from_utf8(&message_bytes).unwrap().as_bytes().as_ptr(),
            core::str::from_utf8(&details_bytes).unwrap().as_bytes().as_ptr(),
            &settings::LanguageCode_Make(settings::Language_Language_English)
        );

        err::ShowApplicationError(&error);
    }
}