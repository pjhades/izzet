release:
	cargo build --release
debug:
	cargo build

test: unit integration
unit:
	cargo test -- --nocapture
integration: debug
	BUILD=debug bash tests/integration.sh

clean:
	cargo clean

.PHONY: release, debug, test, unit, integration, clean
