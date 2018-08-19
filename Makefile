# Project info
CRATE_NAME := rust3ds-template
PROG_NAME := Rust 3DS Template
PROG_DESC := Rust, a modern, safe systems language.
PROG_AUTHOR := You
PROG_ICON := $(DEVKITPRO)/libctru/default_icon.png

3DSXTOOL := $(DEVKITPRO)/tools/bin/3dsxtool
SMDHTOOL := $(DEVKITPRO)/tools/bin/smdhtool

export CC_3ds := $(DEVKITARM)/bin/arm-none-eabi-gcc
export TARGET_CFLAGS := -specs=3dsx.specs -mfloat-abi=hard -march=armv6k -mtune=mpcore \
						-mfpu=vfp -mtp=soft

.PHONY: all clean $(CRATE_NAME) dist test send target/3ds/release/$(CRATE_NAME).elf

all: $(CRATE_NAME) 

target/3ds/release/$(CRATE_NAME).elf:
	RUST_TARGET_PATH=$(shell pwd) xargo build --release

target/3ds/release/$(CRATE_NAME).smdh:
	$(SMDHTOOL) --create "${PROG_NAME}" "${PROG_DESC}" "${PROG_AUTHOR}" "${PROG_ICON}" target/3ds/release/$(CRATE_NAME).smdh

target/3ds/release/$(CRATE_NAME).3dsx: target/3ds/release/$(CRATE_NAME).elf target/3ds/release/$(CRATE_NAME).smdh
	$(3DSXTOOL) target/3ds/release/$(CRATE_NAME).elf target/3ds/release/$(CRATE_NAME).3dsx --smdh=target/3ds/release/$(CRATE_NAME).smdh

$(CRATE_NAME): target/3ds/release/$(CRATE_NAME).3dsx

dist: $(CRATE_NAME)
	mkdir -p dist/$(CRATE_NAME)
	cp target/3ds/release/$(CRATE_NAME).elf dist/$(CRATE_NAME)
	cp target/3ds/release/$(CRATE_NAME).3dsx dist/$(CRATE_NAME)
	cp $(PROG_ICON) dist/$(CRATE_NAME)/$(CRATE_NAME).png

test: $(CRATE_NAME)
	citra target/3ds/release/$(CRATE_NAME).elf

send: $(CRATE_NAME)
	3dslink target/3ds/release/$(CRATE_NAME).3dsx

clean:
	rm -rf target
	rm -rf dist

nuke: clean
	rm -rf ~/.xargo
