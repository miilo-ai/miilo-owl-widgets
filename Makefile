PYTHON ?= python3
.PHONY: assemble test check-owl check-desktop check-keyboard build build-linux package
assemble:
	cargo xtask assemble

test:
	cargo test --locked --manifest-path helper/Cargo.toml -j 2
	cargo test --locked --manifest-path xtask/Cargo.toml -j 2

build:
	cargo build --locked --release --manifest-path helper/Cargo.toml -j 2

build-linux:
	bash tools/build-linux.sh

package:
	bash tools/package.sh

check-owl:
	cargo xtask check-owl

check-desktop:
	cargo xtask check-desktop
	node tools/test-robot-motion.cjs
	node tools/test-orbit.cjs

check-keyboard:
	tools/test-desktop-keyboard.sh

.PHONY: render-design
render-design:
	$(PYTHON) tools/render-orbit.py
