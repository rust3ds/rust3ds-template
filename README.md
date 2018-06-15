# Rust 3DS Template

An example project demonstrating how to compile and run Rust programs on the Nintendo 3DS using [ctru-rs](https://github.com/rust3ds/ctru-rs).

## Requirements

A nightly version of Rust is required to use this project. If you don't have Rust installed or don't have a nightly compiler, check out [rustup.rs](https://rustup.rs).

Next, you will need [Xargo](https://github.com/japaric/xargo) to facilitate cross-compilation to the 3DS.

Finally, you will need the most recent version of [devkitARM](http://sourceforge.net/projects/devkitpro/files/devkitARM/).

### Preparation of the requirements

#### Installing Rust

If you don't have Rust installed (macOS & Linux): 

    `$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly`

If you have already installed rustup: 

    `$ rustup default nightly`

#### Installing Xargo

    `$ rustup component add rust-src`
    `$ cargo install xargo`

#### Installing devkitARM

As of this writing, the current version of devkitARM is r48 and the current version of libctru is 1.5.0.

A detailed tutorial on how to set up devkitARM can be found on the [3dbrew wiki](http://3dbrew.org/wiki/Setting_up_Development_Environment).

On macOS & Linux, devkitARM requires two environment variables to be set:

* `$DEVKITPRO` = `/path/to/devkitPro/` (usually `/opt/devkitpro/`)
* `$DEVKITARM` = `$DEVKITPRO/devkitARM`

## Usage

Use the included `Makefile` to build your program. Under the hood, `make` calls `xargo` to create a custom sysroot containing cross-compiled versions of the Rust core libraries, as well as a limited version of the Rust standard library. `xargo` caches the sysroot after it has been built for the first time. Once the sysroot is in place, a Homebrew Launcher-compatible `3dsx` version of your program will be generated.

Running `make` will build your program in release mode. Running `make test` will open your program in the `Citra` emulator if you have it installed. Running `make send` will send your program to your 3DS via `3dslink` (press Y in the Homebrew Launcher to receive the program).

## Troubleshooting

>I get an error saying that the `3ds` target can't be found when I run `xargo build`

Recent versions of Rust require you to set the `$RUST_TARGET_PATH` env variable to the directory where your target spec is located. The Makefile does this automatically when you invoke it, or you can manually set the variable yourself.

>`std` or `<some other crate>` is failing to build

Nightly Rust moves fast and things tend to break a lot. If you want to use a known-working nightly, try the one listed in the [travis config file](https://github.com/rust3ds/ctru-rs/blob/master/.travis.yml#L3-L4) for `ctru-rs`. And feel free to file an issue detailing the error at the repo too!

>I'm running my homebrew in Citra and I don't see any error message if my program panics

You need to enable Debug SVC output in Citra's logger. There are two configuration files in `~/.config/citra-emu`: `sdl2-config.ini` and `qt-config.ini`. The first is for the normal `citra` executable and the second for `citra-qt`. Add `Debug.Emulated:Debug` to the `log_filter` variable for whichever version you intend to use. The Makefile calls the sdl2 version by default when running `make test`.

>I got `hello world` working, but now what? Is there documentation for how to do anything else?

Not quite yet, but we're working on that. Feel free to browse the list of [example programs](https://github.com/rust3ds/ctru-rs/tree/master/examples/src/bin) in the meantime.

You should also become familiar with [libctru's documentation](http://smealum.github.io/ctrulib/) if you aren't already. It will help you understand what the current `ctru-rs` APIs are doing, plus you can use raw `libctru` functions directly via the `ctru-sys` crate if you want access to anything not yet exposed in `ctru-rs`.
