ROOT := justfile_directory()

RUST_LIBS := ROOT + "/libs"
WASM := ROOT + "/wasm"
WEB := ROOT + "/web"

TARGET := ROOT + "/target"
WASM_TARGET := ROOT + "/target/wasm"
NODE_MODULES := ROOT + "/node_modules"

_default:
    @just --list

### Rust Crates


### WebAssembly
# builds selected wasm crate
build-wasm crate:
	wasm-pack build \
		--target web \
		--release \
		--scope wasm \
		-d {{WASM_TARGET}}/{{crate}} \
		{{WASM}}/{{crate}}

### Web
build-web:
    npx vite build ./web --outDir ../dist --emptyOutDir