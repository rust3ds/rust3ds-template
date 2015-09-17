#![feature(no_std)]
#![feature(start)]
#![no_main]
#![no_std]

extern crate ctru;

use ctru::srv;
use ctru::gfx;
use ctru::services::apt;
use ctru::services::hid;
use ctru::services::gsp;

#[no_mangle]
pub extern fn main(_: isize, _: *const *const u8) -> isize {
    main_3ds();
    0
}

fn main_loop() -> bool {
    gsp::wait_for_event(gsp::Event::VBlank0);
    hid::scan_input();
    if hid::key_down(hid::PadKey::Start) {
        gfx::exit();
        return false;
    }
    true
}

fn main_3ds() -> () {
    srv::init();
    apt::init();
    hid::init();
    gfx::init_default();
    gfx::set_3d_enabled(false);

    apt::main_loop(main_loop);

    gfx::exit();
    hid::exit();
    apt::exit();
    srv::exit();
}
