#![no_std]
#![no_main]

extern crate ctru;

use ctru::console::Console;
use ctru::gfx::{Gfx, Screen};
use ctru::services::{Hid, Apt};
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const u8) -> i32 {
    main_3ds();
    0
}

fn main_3ds() {
    use ctru::services::gspgpu::FramebufferFormat;
    use ctru::services::hid::PadKey;

    let mut apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let mut gfx = Gfx::default();

    gfx.set_framebuffer_format(Screen::Top, FramebufferFormat::Bgr8);
    gfx.set_framebuffer_format(Screen::Bottom, FramebufferFormat::Bgr8);

    let mut console = Console::default();

    writeln!(&mut console, "Hello, {}", "world!").unwrap();
    writeln!(&mut console, "\x1b[28;15HPress START to exit").unwrap();

    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();

        hid.scan_input();
        if hid.key_down(PadKey::Start) {
            break;
        }
    }
}
