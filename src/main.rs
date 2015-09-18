#![feature(no_std)]
#![feature(start)]
#![feature(alloc)]
#![feature(collections)]
#![no_main]
#![no_std]

extern crate ctru;
extern crate alloc;
#[macro_use] extern crate collections;

use ctru::{Srv, Gfx};
use ctru::services::{Hid, Apt, Application};
use ctru::services::hid::PadKey;
use ctru::services::gsp;

#[no_mangle]
pub extern fn main(_: isize, _: *const *const u8) -> isize {
    main_3ds();
    0
}

struct MyApplication {
    hid: Hid,
    gfx: Gfx,
    running: bool
}

impl Application for MyApplication {
    fn main_loop(&mut self, _: &mut Apt) {
        use ctru::gfx::{Screen, Side};
        use ctru::services::gsp::FramebufferFormat;

        self.gfx.set_framebuffer_format(Screen::Top, FramebufferFormat::Bgr8);
        let (fb, _, _) = self.gfx.get_framebuffer(Screen::Top, Side::Left);

        gsp::wait_for_event(gsp::Event::VBlank0);
        for i in fb.iter_mut() {
            *i = 127u8;
        }
        self.gfx.flush_buffers();
        self.gfx.swap_buffers();

        self.hid.scan_input();
        if self.hid.key_down(PadKey::Start) {
            self.running = true;
        }
    }

    fn ready_to_quit(&self) -> bool { !self.running }
}

fn main_3ds() -> () {
    let srv = match Srv::new() {
        Ok(s) => s,
        _ => return
    };
    let mut apt = match Apt::new() {
        Ok(a) => a,
        _ => return
    };
    let hid = match Hid::new() {
        Ok(h) => h,
        _ => return
    };
    let gfx = Gfx::default();

    let mut myapp = MyApplication {
        hid: hid,
        gfx: gfx,
        running: true
    };

    apt.main_loop(&mut myapp);

    drop(myapp);
    drop(apt);
    drop(srv);
}
