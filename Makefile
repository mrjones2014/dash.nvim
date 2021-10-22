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
	mkdir -p ./bin/
	rm ./bin/*.dylib
	cp ./target/aarch64-apple-darwin/release/libdash_nvim.dylib ./target/aarch64-apple-darwin/release/libdash_nvim.so
	cp ./target/x86_64-apple-darwin/release/libdash_nvim.dylib ./target/x86_64-apple-darwin/release/libdash_nvim.so

install:
	./bin/install-for-architecture.sh

install-hooks:
	@git config core.hooksPath .githooks
	@echo "Git hooks installed."
