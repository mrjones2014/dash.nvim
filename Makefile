.PHONY: prepare
prepare:
	@git submodule update --depth 1 --init

.PHONY: test-lua
test-lua: build-macos-x86
test-lua: install
test-lua: prepare
	@echo "Running Lua tests..."
	@nvim --headless --noplugin -u spec/spec.vim -c "PlenaryBustedDirectory spec/ { minimal_init = 'spec/spec.vim' }"

.PHONY: test-rust
test-rust:
	@echo "Running Rust tests..."
	@cargo test

.PHONY: test
test: test-lua
test: test-rust

# Run nvim with DASH_NVIM_DEV env var, this adds a reload command to reload the plugin
export DASH_NVIM_DEV=1
.PHONY: dev
dev:
	nvim

.PHONY: clean
clean:
	@cargo clean

.PHONY: lint-rust
lint-rust:
	cargo clippy -- -D warnings

.PHONY: build-macos-arm
build-macos-arm:
	./scripts/build-for-architecture.bash macos-arm

.PHONY: build-macos-x86
build-macos-x86:
	./scripts/build-for-architecture.bash macos-x86

.PHONY: build-local
buld-local:
	./scripts/build-for-architecture.bash host

.PHONY: build
build: clean
build: build-macos-arm
build: build-macos-x86

.PHONY: install
install:
	@./scripts/install-for-architecture.bash

.PHONY: install-hooks
install-hooks:
	@git config core.hooksPath .githooks
	@echo "Git hooks installed."
