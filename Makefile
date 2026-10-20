.PHONY: assemble test check-owl check-desktop check-keyboard check-typography build build-linux package
assemble:
	cargo xtask assemble

test: check-preview-toolchain
	node tools/test-render-invalidation.cjs
	node --test tools/plain-text.test.cjs
	node --test tools/install-helper.test.cjs
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
	node tools/test-compact.cjs
	node tools/test-session-visibility.cjs
	node tools/test-mobile-settings.cjs
	node tools/test-helper-onboarding.cjs

check-keyboard:
	tools/test-desktop-keyboard.sh

check-typography: check-plain-text
	bash tools/test-typography.sh

.PHONY: render-design
render-design:
	node tools/render-orbit.cjs

.PHONY: check-plain-text
check-plain-text:
	node tools/test-plain-text.cjs tests/desktop/TestPlainText.qml

.PHONY: check-canvas-clock
check-canvas-clock:
	node tools/test-canvas-clock.cjs

check-typography: check-canvas-clock

.PHONY: check-preview-toolchain check-preview
check-preview-toolchain:
	node tools/check-preview-toolchain.cjs
	node --test tools/preview-toolchain.test.cjs
	cargo fmt --manifest-path tools/qt-preview/Cargo.toml --check
	cargo test --locked --manifest-path tools/qt-preview/Cargo.toml
	cargo clippy --locked --manifest-path tools/qt-preview/Cargo.toml --all-targets -- -D warnings

check-preview: check-preview-toolchain
	node --test tools/preview-capture.test.cjs
