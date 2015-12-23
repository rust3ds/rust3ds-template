# Project info
CRATE_NAME := rust3ds-template
PROG_NAME := Rust 3DS Template
PROG_DESC := Rust, a modern, safe systems language.
PROG_AUTHOR := You
PROG_ICON := $(CTRULIB)/default_icon.png

3DSXTOOL := $(DEVKITARM)/bin/3dsxtool
SMDHTOOL := $(DEVKITARM)/bin/smdhtool

# Optional variable
ifeq ($(strip $(RUST_3DS_SYSROOT)),)
	RUST_3DS_SYSROOT := sysroot
endif

.PHONY: all clean $(CRATE_NAME) dist sysroot target/3ds/release/$(CRATE_NAME).elf

all: $(CRATE_NAME)

target/3ds/release/$(CRATE_NAME).elf:
	$(CARGO_BUILD) --release --sysroot $(RUST_3DS_SYSROOT) --target "3ds.json" $(CARGO_FLAGS) -- -Z no-landing-pads

target/3ds/release/$(CRATE_NAME).smdh:
	$(SMDHTOOL) --create "${PROG_NAME}" "${PROG_DESC}" "${PROG_AUTHOR}" "${PROG_ICON}" target/3ds/release/$(CRATE_NAME).smdh

target/3ds/release/$(CRATE_NAME).3dsx: target/3ds/release/$(CRATE_NAME).elf target/3ds/release/$(CRATE_NAME).smdh
	$(3DSXTOOL) target/3ds/release/$(CRATE_NAME).elf target/3ds/release/$(CRATE_NAME).3dsx --smdh=target/3ds/release/$(CRATE_NAME).smdh

$(CRATE_NAME): target/3ds/release/$(CRATE_NAME).3dsx

dist: $(CRATE_NAME)
	mkdir -p dist/$(CRATE_NAME)
	cp target/3ds/release/$(CRATE_NAME).elf dist/$(CRATE_NAME)
	cp target/3ds/release/$(CRATE_NAME).3dsx dist/$(CRATE_NAME)
	cp target/3ds/release/$(CRATE_NAME).smdh dist/$(CRATE_NAME)
	cp $(PROG_ICON) dist/$(CRATE_NAME)/$(CRATE_NAME).png

clean:
	rm -rf target
	rm -rf dist

# Useful targets

sysroot: sysroot/lib/rustlib/3ds.json/lib/libcore.rlib \
		sysroot/lib/rustlib/3ds.json/lib/liballoc.rlib \
		sysroot/lib/rustlib/3ds.json/lib/liblibc.rlib \
		sysroot/lib/rustlib/3ds.json/lib/librustc_unicode.rlib \
		sysroot/lib/rustlib/3ds.json/lib/libcollections.rlib

sysroot/lib/rustlib/3ds.json/lib:
	@mkdir -p sysroot/lib/rustlib/3ds.json/lib

sysroot/lib/rustlib/3ds.json/lib/%.rlib: sysroot/lib/rustlib/3ds.json/lib
	@echo Building $*
	@rustc --target 3ds.json -o sysroot/lib/rustlib/3ds.json/lib/$*.rlib --sysroot $(RUST_3DS_SYSROOT) --cfg target_os,linux $(RUST_SRC_PATH)/$*/lib.rs -Z no-landing-pads

sysroot/lib/rustlib/3ds.json/lib/liblibc.rlib: 
	@echo Building liblibc
	@rustc --target 3ds.json -o sysroot/lib/rustlib/3ds.json/lib/liblibc.rlib --sysroot $(RUST_3DS_SYSROOT) --cfg     stdbuild --cfg target_os,linux $(RUST_SRC_PATH)/liblibc/src/lib.rs -Z no-landing-pads

