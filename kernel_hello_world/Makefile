.PHONY: build run clean

build:
	cargo build --target x86_64-unknown-none

image: build
	bootimage build --target x86_64-unknown-none

run: image
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/boot-image.bin -nographic -serial stdio

clean:
	cargo clean
	rm -f target/x86_64-unknown-none/debug/boot-image.bin
