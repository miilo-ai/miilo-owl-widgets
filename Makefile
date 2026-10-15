.PHONY: assemble test check-owl build build-linux package
assemble:
	python3 tools/assemble.py

test:
	cargo test --locked --manifest-path helper/Cargo.toml -j 2
	cargo build --locked --manifest-path helper/Cargo.toml -j 2
	MIILO_OWL_BINARY="$(CURDIR)/helper/target/debug/miilo-owl" python3 -m unittest discover -s tests -p 'test_*.py' -v

build:
	cargo build --locked --release --manifest-path helper/Cargo.toml -j 2

build-linux:
	bash tools/build-linux.sh

package:
	bash tools/package.sh

check-owl:
	python3 tools/validate.py
