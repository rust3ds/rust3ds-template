# Rust 3DS Template

A project template for Rust projects to be built on the Nintendo 3DS Homebrew Launcher.


## What you need

First of all, you need to be running a Nightly build of Rust. This project will NOT currently compile on Stable Rust.

Secondly, you need to install [Xargo](https://github.com/japaric/xargo). All you have to do is type `cargo install xargo` into your terminal and wait for it to compile.

Finally, you will need to install [devkitARM](http://sourceforge.net/projects/devkitpro/files/devkitARM/). A more detailed tutorial on how to set up dkA for the 3DS can be found [here](http://3dbrew.org/wiki/Setting_up_Development_Environment)


## Environment configuration

The following environment variables need to be set:

 * `$DEVKITPRO`
 * `$DEVKITARM`
 * `$CTRULIB`
 
These should already be in place if you've properly installed devkitARM. If you missed that step somewhere along the line, refer again to the [3DS homebrew environment setup tutorial](http://3dbrew.org/wiki/Setting_up_Development_Environment)
 

## Building libcore (and friends) for the 3ds

When you build your program using `xargo build` instead of `cargo build`, a custom 3DS-compatible sysroot will be compiled. While the full Rust standard libary is currently unavailable, the following crates are able to be used in your project: 

 * `core` -- platform-agnostic basics + prelude
 * `alloc` -- memory allocation functions
 * `rustc_unicode` -- unicode stuff
 * `collections` -- std collections (requires `alloc`, `rustc_unicode`)

Allocators are provided by a simple implementation of `alloc_system`. This means that `Box` is available, so `collections` will work in its entirety.


## Building your 3ds homebrew project

Run `xargo build` or `xargo build --release` and a 3DS-compatible elf file will be generated. Additionally, you can simply run the `make` command and a Homebrew Launcher-compatible 3dsx file will also be created.

Other useful Make commands are `make clean` to clear out old build artifacts and `make dist` to put the resulting 3ds homebrew files in an easily distributable folder.
