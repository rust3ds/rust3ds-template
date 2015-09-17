#![feature(no_std)]
#![feature(start)]
#![feature(alloc)]
#![feature(collections)]
#![no_main]
#![no_std]

extern crate ctru;
extern crate alloc;
#[macro_use] extern crate collections;

use ctru::srv;
use ctru::gfx::Gfx;
use ctru::services::apt;
use ctru::services::hid;
use ctru::services::gsp;

use ctru::raw;

use collections::vec::Vec;

#[no_mangle]
pub extern fn main(_: isize, _: *const *const u8) -> isize {
    main_3ds();
    0
}

fn main_3ds() -> () {
    srv::init();
    apt::init();
    hid::init();
    let mut gfx = Gfx::default();

    let v: Vec<u8> = vec![64u8, 128u8, 192u8, 255u8, 192u8, 128u8, 64u8, 0u8];

    let mut iterate: usize = 0;

    apt::main_loop(|| {
        use ctru::gfx::{Screen, Side};
        use ctru::services::gsp::FramebufferFormat;

        gfx.set_framebuffer_format(Screen::Top, FramebufferFormat::Bgr8);
        let (fb, _, _) = gfx.get_framebuffer(Screen::Top, Side::Left);

        gsp::wait_for_event(gsp::Event::VBlank0);
        let value = v[iterate];
        for i in fb.iter_mut() {
            *i = value;
        }
        gfx.flush_buffers();
        gfx.swap_buffers();
        iterate += 1usize;
        if iterate >= v.len() { iterate = 0; }

        hid::scan_input();
        if hid::key_down(hid::PadKey::Start) {
            return false;
        }
        true
    });

    drop(gfx);
    hid::exit();
    apt::exit();
    srv::exit();
}
