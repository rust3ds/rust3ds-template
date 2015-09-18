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

        let (fb, _, _) = self.gfx.get_framebuffer(Screen::Top, Side::Left);
        let (rfb, _, _) = self.gfx.get_framebuffer(Screen::Top, Side::Right);
        let (bfb, _, _) = self.gfx.get_framebuffer(Screen::Bottom, Side::Left);

        let topdepth = self.gfx.get_framebuffer_format(Screen::Top).pixel_depth_bytes();
        let botdepth = self.gfx.get_framebuffer_format(Screen::Bottom).pixel_depth_bytes();

        gsp::wait_for_event(gsp::Event::VBlank0);
        for i in fb.chunks_mut(topdepth) {
            i[0] = 127u8;
            i[1] = 127u8;
            i[2] = 127u8;
        }
        for i in rfb.chunks_mut(topdepth) {
            i[0] = 0u8;
            i[1] = 127u8;
            i[2] = 255u8;
        }
        for i in bfb.chunks_mut(botdepth) {
            i[0] = 255u8;
            i[1] = 127u8;
            i[2] = 0u8;

        }
        self.gfx.flush_buffers();
        self.gfx.swap_buffers();

        self.hid.scan_input();
        if self.hid.key_down(PadKey::Start) {
            self.running = false;
        }
    }

    fn ready_to_quit(&self) -> bool { !self.running }
}

fn main_3ds() -> () {
    use ctru::gfx::Screen;
    use ctru::services::gsp::FramebufferFormat;

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
    let mut gfx = Gfx::default();

    gfx.set_framebuffer_format(Screen::Top, FramebufferFormat::Bgr8);
    gfx.set_framebuffer_format(Screen::Bottom, FramebufferFormat::Bgr8);

    gfx.set_3d_enabled(true);

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
