#![no_main]
#![no_std]

extern crate ctru;

use core::fmt::Write;

use ctru::Gfx;
use ctru::console::Console;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, PadKey};

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const u8) -> i32 {
    main_3ds();
    0
}

fn main_3ds() {
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let mut gfx = Gfx::default();
    let mut console = Console::default();

    writeln!(console, "Hello, world!").unwrap();

    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();

        hid.scan_input();
        if hid.key_down(PadKey::Start) {
            break;
        }
    }
}
