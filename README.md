# Rust 3DS Template

A project template for Rust projects to be built on the Nintendo 3DS Homebrew Launcher.


## What you need

First of all, you need to be running a Nightly build of Rust. This project will NOT currently compile on Stable Rust. 

You will also need to download the Rust Nightly source (making sure that the source you download is the same version as the Rust build you are actually running) and save it to your computer. Both the Nightly builds and the source code can be found at the bottom of the page [here](https://www.rust-lang.org/downloads.html). 

You will also need the following:
 * [devkitARM](http://sourceforge.net/projects/devkitpro/files/devkitARM/). A more detailed tutorial on how to set up dkA for the 3DS can be found [here](http://3dbrew.org/wiki/Setting_up_Development_Environment)
 * `libcompiler-rt.a` for 3ds. This is already provided in the 'sysroot' folder.
 * `libcore` compiled from Rust source code you downloaded above. Instructions on how to build libcore (and other rust libraries) for the 3ds are detailed later in this readme.


## Environment configuration

The following environment variables need to be set:

 * `$DEVKITPRO`
 * `$DEVKITARM`
 * `$CTRULIB`
 
These should already be in place if you've properly installed devkitARM. If you missed that step somewhere along the line, refer again to the [3DS homebrew environment setup tutorial](http://3dbrew.org/wiki/Setting_up_Development_Environment)
 
 * `$RUST_SRC_PATH` -- path to the `src` folder of the Rust source code you downloaded above. This is required so that you can build libcore as detailed below.


## Building libcore (and friends) for the 3ds

Instead of invoking Cargo directly, you should use the included Makefile to start building your 3ds code. Before you can compile your project, you need to use the Make command `make sysroot` to build 3ds versions of the following libraries:

 * `core` -- platform-agnostic basics + prelude
 * `alloc` -- memory allocation functions
 * `libc` -- libc bindings. note: using some functions may result in undefined symbols
 * `rustc_unicode` -- unicode stuff
 * `collections` -- std collections (requires `alloc`, `rustc_unicode`)

Allocators are provided by a simple implementation of `alloc_system`. This means that `Box` is available, so `collections` will work in its entirety.
**Notably, libstd is not currently available. It requires some unwinding features that are not (yet?) possible to implement for the 3ds.**


## Building your 3ds homebrew project

Simply run the `make` command and, if everything is all set up properly, your homebrew project will compile!

Other useful commands are `make clean` to clear out old build artifacts and `make dist` to put the resulting 3ds homebrew files in an easily distributable folder.
