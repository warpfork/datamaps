(export CFLAGS_wasm32_unknown_unknown="-I$(pwd)/wasm-sysroot -Wbad-function-cast -Wcast-function-type -fno-builtin" ; ~/.cargo/bin/wasm-pack build  --target=web)
