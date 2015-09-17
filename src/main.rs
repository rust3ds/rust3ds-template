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
use ctru::gfx;
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

fn main_loop() -> bool {
    hid::scan_input();
    if hid::key_down(hid::PadKey::Start) {
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

    let v: Vec<u8> = vec![64u8, 128u8, 192u8, 255u8, 192u8, 128u8, 64u8, 0u8];

    let mut iterate: usize = 0;

    apt::main_loop(|| {
        let mut w = 0u16;
        let mut h = 0u16;
        let tb: *mut u8 = unsafe {
            raw::gfx::gfxGetFramebuffer(raw::gfx::gfxScreen_t::GFX_TOP,
                raw::gfx::gfx3dSide_t::GFX_LEFT, &mut w as *mut u16, &mut h as *mut u16)
        };
        gsp::wait_for_event(gsp::Event::VBlank0);
        for xx in 0..(w as isize * h as isize) {
            unsafe { *tb.offset(xx * 3 + 2) = v[iterate] }
        }
        gfx::flush_buffers();
        gfx::swap_buffers();
        iterate += 1usize;
        if iterate >= v.len() { iterate = 0; }
        main_loop()
    });

    gfx::exit();
    hid::exit();
    apt::exit();
    srv::exit();
}
