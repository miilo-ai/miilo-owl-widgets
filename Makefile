.PHONY: assemble test check-owl check-desktop check-keyboard check-typography build build-linux package
assemble:
	cargo xtask assemble

test:
	node tools/check-no-python.cjs
	node --test tools/no-python.test.cjs
	node tools/check-architecture.cjs
	node --test tools/architecture.test.cjs .github/release-tools/sdk-access.test.cjs
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
	node tools/test-portrait.cjs
	node tools/test-session-visibility.cjs

check-keyboard:
	tools/test-desktop-keyboard.sh

check-typography:
	bash tools/test-typography.sh

.PHONY: render-design
render-design:
	node tools/render-orbit.cjs
