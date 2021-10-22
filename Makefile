prepare:
	@git submodule update --depth 1 --init

test: build-for-ci
test: install
test: prepare
	@nvim --headless --noplugin -u spec/spec.vim -c "PlenaryBustedDirectory spec/ { minimal_init = 'spec/spec.vim' }"

watch: prepare
	@echo -e '\nRunning tests on "spec/**/*_spec.lua" when any Lua file on "lua/" and "spec/" changes\n'
	@find spec/ lua/ -name '*.lua' | entr make test

clean:
	@cargo clean

build-macos-arm:
	cargo build --release --target aarch64-apple-darwin

build-macos-x86:
	cargo build --release --target x86_64-apple-darwin

build-for-ci:
	cargo build --release
	@rm -rf ./lua/libdash_nvim.so ./lua/deps/
	@cp ./target/release/libdash_nvim.so ./lua/libdash_nvim.so
	@mkdir -p ./lua/deps/
	@cp -r ./target/release/deps/*.rlib ./lua/deps/

install-linux:
	echo test

build: build-macos-arm
build: build-macos-x86
	rm -rf ./bin/
	mkdir -p ./bin/arm/deps/
	mkdir -p ./bin/x86/deps/
	@cp ./target/aarch64-apple-darwin/release/libdash_nvim.dylib ./bin/arm/libdash_nvim.so
	@cp ./target/aarch64-apple-darwin/release/deps/*.rlib ./bin/arm/deps/
	@cp ./target/x86_64-apple-darwin/release/libdash_nvim.dylib ./bin/x86/libdash_nvim.so
	@cp ./target/x86_64-apple-darwin/release/deps/*.rlib ./bin/x86/deps/

install:
	./scripts/install-for-architecture.bash

install-hooks:
	@git config core.hooksPath .githooks
	@echo "Git hooks installed."
