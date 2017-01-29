extern crate ctru;

use ctru::Gfx;
use ctru::console::Console;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, PadKey};

fn main() {
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let mut gfx = Gfx::default();
    let _console = Console::default();

    println!("Hello, world!");

    println!("\x1b[28;15HPress Start to exit");

    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();

        hid.scan_input();
        if hid.key_down(PadKey::Start) {
            break;
        }
    }
}
