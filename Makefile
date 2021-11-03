prepare:
	@git submodule update --depth 1 --init

test: build-macos-x86
test: install
test: prepare
	@nvim --headless --noplugin -u spec/spec.vim -c "PlenaryBustedDirectory spec/ { minimal_init = 'spec/spec.vim' }"

.PHONY: watch
watch: prepare
	@echo -e '\nRunning tests on "spec/**/*_spec.lua" when any Lua file on "lua/" and "spec/" changes\n'
	@find spec/ lua/ -name '*.lua' | entr make test


# Run nvim with DASH_NVIM_DEV env var, this adds a reload command to reload the plugin
export DASH_NVIM_DEV=1
.PHONY: dev
dev:
	nvim

clean:
	@cargo clean

build-macos-arm:
	cargo build --release --target aarch64-apple-darwin
	rm -rf ./bin/arm/
	mkdir -p ./bin/arm/deps/
	@cp ./target/aarch64-apple-darwin/release/libdash_nvim.dylib ./bin/arm/libdash_nvim.so
	@cp ./target/aarch64-apple-darwin/release/deps/*.rlib ./bin/arm/deps/

build-macos-x86:
	cargo build --release --target x86_64-apple-darwin
	rm -rf ./bin/x86/
	mkdir -p ./bin/x86/deps/
	@cp ./target/x86_64-apple-darwin/release/libdash_nvim.dylib ./bin/x86/libdash_nvim.so
	@cp ./target/x86_64-apple-darwin/release/deps/*.rlib ./bin/x86/deps/

build: build-macos-arm
build: build-macos-x86

.PHONY: install
install:
	./scripts/install-for-architecture.bash

.PHONY: install-hooks
install-hooks:
	@git config core.hooksPath .githooks
	@echo "Git hooks installed."
