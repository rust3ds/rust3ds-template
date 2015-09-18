# Rust 3DS Template

A project template for Rust projects to be built on the Nintendo 3DS Homebrew Launcher.

## What you need

 * devKitARM
 * Rust nightly (1.4 as of this writing)
 * [my modified `cargo-build`](https://github.com/Furyhunter/cargo-build) -- subject to change if changes get merged.
 * `libcore` compiled from Rust sources. Instructions below on building.
 * `libcompiler-rt.a` for 3ds. Provided in sysroot folder. Use the sysroot target to build the rust dependency libraries as mentioned below, and everything should work automatically.

## Environment configuration

The file `~/.cargo/config` must be created with these contents:

```toml
[target.3ds]
ar = "/path/to/arm-none-eabi-ar"
```

The archiver is not settable using target json, so you have to set this
manually. Otherwise the system `ar` will be used, and you'll get sad linker
errors.

The following environment variables need to be set:

 * `$DEVKITPRO`
 * `$DEVKITARM`
 * `$CTRULIB`
 * `$RUST_3DS_SYSROOT` - path to sysroot such that `libcore.rlib` is located in the path `lib/rustlib/3ds.json/lib/libcore.rlib`. This is set to `sysroot` by default. **You can use the sysroot target to generate the necessary libcore locally; see below**
 * `$CARGO_BUILD` - path to `cargo-build` binary

The sysroot libraries can be generated using the `sysroot` target. For that, you need to set this environment variable:

 * `$RUST_SRC_PATH` -- path to `src` of rust checkout

The following rustc libraries can be built. **Notably, libstd is not currently
available. It requires some unwinding features that are not possible.**

 * `core` -- platform-agnostic basics + prelude
 * `alloc` -- memory allocation functions
 * `libc` -- libc bindings. note: using some functions may result in undefined symbols
 * `rustc_unicode` -- unicode stuff
 * `collections` -- std collections (requires `alloc`, `rustc_unicode`)

The make target `sysroot` automatically builds these. Allocators are provided by a simple implementation of `alloc_system`. This means that `Box` is available, so `collections` will work in its entirety.
