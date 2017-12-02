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
    `$ cargo install xargo --vers 0.3.8`

    NOTE: Xargo 0.3.8 is currently required because 0.3.9 fails to build the `compiler_builtins` crate

#### Installing devkitARM

As of this writing, the current version of devkitARM is r47 and the current version of libctru is 1.4.0.

A detailed tutorial on how to set up devkitARM can be found on the [3dbrew wiki](http://3dbrew.org/wiki/Setting_up_Development_Environment).

On macOS & Linux, devkitARM requires two environment variables to be set:

* `$DEVKITPRO` = `/path/to/devkitPro/` (usually `/opt/devkitpro/`)
* `$DEVKITARM` = `$DEVKITPRO/devkitARM`

## Usage

Use the included `Makefile` to build your program. Under the hood, `make` calls `xargo` to create a custom sysroot containing cross-compiled versions of the Rust core libraries, as well as a limited version of the Rust standard library. `xargo` caches the sysroot after it has been built for the first time. 

Once the sysroot is in place, a Homebrew Launcher-compatible `3dsx` version of your program will be generated.
