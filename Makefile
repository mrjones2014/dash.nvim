prepare:
	@git submodule update --depth 1 --init

test: prepare
	@nvim --headless --noplugin -u spec/spec.vim -c "PlenaryBustedDirectory spec/ { minimal_init = 'spec/spec.vim' }"

watch: prepare
	@echo -e '\nRunning tests on "spec/**/*_spec.lua" when any Lua file on "lua/" and "spec/" changes\n'
	@find spec/ lua/ -name '*.lua' | entr make test

clean:
	@cargo clean

build-rust:
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	rm -rf ./bin/
	mkdir -p ./bin/arm/deps/
	mkdir -p ./bin/x86/deps/
	cp ./target/aarch64-apple-darwin/release/libdash_nvim.dylib ./bin/arm/libdash_nvim.so
	cp -r ./target/aarch64-apple-darwin/release/deps/ ./bin/arm/deps/
	cp ./target/x86_64-apple-darwin/release/libdash_nvim.dylib ./bin/x86/libdash_nvim.so
	cp -r ./target/x86_64-apple-darwin/release/deps/ ./bin/x86/deps/

install:
	./scripts/install-for-architecture.bash

install-hooks:
	@git config core.hooksPath .githooks
	@echo "Git hooks installed."
