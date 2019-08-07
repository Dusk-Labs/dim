test:
	cargo test

build-release:
	mkdir build
	mkdir build/web
	cargo build --release && mv ./target/release/OpenFlixServerRust ./build/dim && strip ./build/dim
	cd web_ui && npm run-script build
	mv web_ui/build/* build/web

clean:
	cargo clean
	rm -rf ./build
	rm -rf ./web_ui/build

build:
	cargo build
