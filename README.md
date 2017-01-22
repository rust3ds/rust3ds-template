# Rust 3DS Template

An example project demonstrating how to compile and run Rust programs on the Nintendo 3DS using [ctru-rs](https://github.com/rust3ds/ctru-rs).

## Requirements

A nightly version of Rust is required to use this project. If you don't have Rust installed or don't have a nightly compiler, check out [rustup.rs](https://rustup.rs).

Next, you will need [Xargo](https://github.com/japaric/xargo) to facilitate cross-compilation to the 3DS. It is easily installable via Cargo: `$ cargo install xargo`

Finally, you will need the most recent version of [devkitARM](http://sourceforge.net/projects/devkitpro/files/devkitARM/). A detailed tutorial on how to set up devkitARM can be found on the [3dbrew wiki](http://3dbrew.org/wiki/Setting_up_Development_Environment).


## Useage

Use the included `Makefile` to build your program. Under the hood, `make` calls `xargo` to create a custom sysroot containing cross-compiled versions of the Rust core libraries, as well as a limited version of the Rust standard library. `xargo` caches the sysroot after it has been built for the first time. 

Once the sysroot is in place, a Homebrew Launcher-compatible `3dsx` version of your program will be generated.

## Troubleshooting

The required version of devkitARM is r46. The required version of ctrulib is 1.2.

devkitARM requires two environment variables to be set:

* $DEVKITPRO = /path/to/devkitPro/ (usually `/opt/devkitPro/`)
* $DEVKITARM = $DEVKITPRO/devkitARM`

If you encounter the following error:

```
process didn't exit successfully: `/tmp/xargo.dBBjgk2BZXsS/target/release/build/compiler_builtins-68d510bc1c7df42e/build-script-build` (exit code: 1)
--- stdout
cargo:rerun-if-changed=build.rs

--- stderr
error: Error loading target specification: Could not find specification for target "3ds"
help: If using a custom target, set the RUST_TARGET_PATH env var to the directory where its .json file is stored.
```

set `$RUST_TARGET_PATH` to the directory where you placed this template project. The `Makefile` passes this argument when invoked, but you will need to manually set the variable if you run `xargo build` by itself.
