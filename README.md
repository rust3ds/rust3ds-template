# Rust 3DS Template

A template for writing Nintendo 3DS homebrew in Rust with [ctru-rs](https://github.com/rust3ds/ctru-rs).


## What you need

First of all, you need to be running a nightly build of Rust. If you don't have Rust installed or don't have a nightly compiler, check out [rustup.rs](https://rustup.rs/)

Next, you need to install [Xargo](https://github.com/japaric/xargo). All you have to do is type `cargo install xargo` in your terminal and wait for it to compile.

Finally, you will need to install [devkitARM](http://sourceforge.net/projects/devkitpro/files/devkitARM/). A more detailed tutorial on how to set up dkA for the 3DS can be found [here](http://3dbrew.org/wiki/Setting_up_Development_Environment)


## Environment configuration

The following environment variables need to be set:

 * `$DEVKITPRO` = `/path/to/devkitPro/`
 * `$DEVKITARM` = `$DEVKITPRO/devkitARM`
 * `$CTRULIB` (usually is `$DEVKITPRO/libctru`)

These should already be in place if you've properly installed devkitARM. If you missed that step somewhere along the line, refer again to the [3DS homebrew environment setup tutorial](http://3dbrew.org/wiki/Setting_up_Development_Environment)


## Building libcore (and friends) for the 3DS

The Rust standard library is not available for 3DS homebrew, as devkitARM does not expose the full set of functions that would be required to support it. However, there are a number of freestanding crates that can be used, and that's where Xargo comes in. When you first compile your homebrew project, the following crates will be built:

 * `core` -- platform-agnostic basics + prelude
 * `alloc` -- memory allocation functions
 * `rustc_unicode` -- unicode stuff
 * `collections` -- std collections (requires `alloc`, `rustc_unicode`)

Allocators are provided by a simple implementation of `alloc_system`. This means that `Box` is available, so `collections` will work in its entirety.

Additional system functionality such as screen and file IO is exposed via [ctru-rs](https://github.com/rust3ds/ctru-rs), a Rust library that wraps around [ctrulib](https://github.com/smealum/ctrulib/)


## Building your homebrew project

Simply run the Makefile. `make` calls Xargo, which builds a 3DS-compatible elf and converts it to the .3dsx format for use in the Homebrew Launcher.

`make test` will build your project and open it in the Citra emulator if you have it installed, and `make send` will send your program to a 3DS running the Homebrew Launcher via 3dslink.
