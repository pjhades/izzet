release:
	cargo build --release

debug:
	cargo build

test:
	cargo test -- --nocapture

install:
	cargo install

clean:
	cargo clean

.PHONY: release, debug, test, clean, install
