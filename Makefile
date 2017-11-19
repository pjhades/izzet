all:
	cargo build --release

test:
	bash tests/integration.sh

clean:
	cargo clean

.PHONY: all, test, clean
