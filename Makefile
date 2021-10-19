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
	mv ./target/aarch64-apple-darwin/release/libdash_nvim.dylib ./bin/dash_lib_arm.dylib
	mv ./target/x86_64-apple-darwin/release/libdash_nvim.dylib ./bin/dash_lib_x86.dylib

install:
	if [[ "$(uname -m)" == "arm64" ]]; then mv ./bin/dash_lib_arm.dylib ./lua/dash_lib.so; else mv ./bin/dash_lib_x86.dylib ./lua/dash_lib.so; fi

install-hooks:
	@git config core.hooksPath .githooks
	@echo "Git hooks installed."
