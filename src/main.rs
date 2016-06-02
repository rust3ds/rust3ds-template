#![feature(start)]
#![no_main]
#![no_std]

#[macro_use]
extern crate ctru;
use ctru::prelude::*;

use ctru::Gfx;
use ctru::console::Console;
use ctru::services::{Hid, Apt};
use ctru::services::hid::PadKey;

#[no_mangle]
pub extern "C" fn main(_: isize, _: *const *const u8) -> isize {
    main_3ds();
    0
}

fn main_3ds() -> () {
    use ctru::gfx::Screen;
    use ctru::services::gspgpu::FramebufferFormat;

    let mut apt = Apt::new();
    let mut hid = Hid::new();
    let mut gfx = Gfx::default();

    gfx.set_framebuffer_format(Screen::Top, FramebufferFormat::Bgr8);
    gfx.set_framebuffer_format(Screen::Bottom, FramebufferFormat::Bgr8);

    let mut console = Console::default();

    console.println("Hello, this is a walrus!");

    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();

        hid.scan_input();
        if hid.key_down(PadKey::Start) {
            break;
        }
    }
}
