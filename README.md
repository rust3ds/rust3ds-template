# Rust 3DS Template

A project template for Rust projects to be built on the Nintendo 3DS Homebrew Launcher.

## What you need

 * devKitARM
 * Rust nightly (1.4 as of this writing)
 * [my modified `cargo-build`](https://github.com/Furyhunter/cargo-build) -- subject to change if changes get merged.
 * `libcore` compiled from Rust sources. Instructions below on building.
 * `libcompiler-rt.a` for 3ds. Provided in sysroot folder. Use the sysroot target to build the rust dependency libraries as mentioned below, and everything should work automatically.

## Environment configuration

The following environment variables need to be set:

 * `$DEVKITPRO`
 * `$DEVKITARM`
 * `$CTRULIB`
 * `$RUST_3DS_SYSROOT` - path to sysroot such that `libcore.rlib` is located in the path `lib/rustlib/3ds.json/lib/libcore.rlib`. This is set to `sysroot` by default. **You can use the sysroot target to generate the necessary libcore locally; see below**
 * `$CARGO_BUILD` - path to `cargo-build` binary

The sysroot libraries can be generated using the `sysroot` target. For that, you need to set this environment variable:

 * `$RUST_SRC_PATH` -- path to root of rust checkout.

The following rustc libraries can be built. **Notably, libstd is not currently
available. It requires some unwinding features that are not possible.**

 * `core` -- platform-agnostic basics + prelude
 * `alloc` -- memory allocation functions
 * `libc` -- libc bindings. note: using some functions may result in undefined symbols
 * `alloc_system` -- implementation of memory allocation functions using libc (required by `alloc`, requires `libc`). likely will not work without linking `libctru`
 * `rustc_unicode` -- unicode stuff
 * `collections` -- std collections (requires `alloc`, `rustc_unicode`)

The make target `sysroot` automatically builds these. The default code only
refers to `core` however.
