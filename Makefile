all: build android

check: force
	cargo check --all

build: force
	cargo build --all

android: force
	cargo build -p edgin_around_android --target aarch64-linux-android --release
	cargo build -p edgin_around_android --target armv7-linux-androideabi --release
	cargo build -p edgin_around_android --target x86_64-linux-android --release
	cargo build -p edgin_around_android --target i686-linux-android --release

setup: force
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android

test: force
	cargo test --all

clean:
	rm -rf target

force:

