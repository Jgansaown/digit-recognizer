ROOT := justfile_directory()

RUST_LIBS := ROOT + "/libs"
WASM := ROOT + "/wasm"
WEB := ROOT + "/web"

TARGET := ROOT + "/target"
WASM_TARGET := ROOT + "/target/wasm"
NODE_MODULES := ROOT + "/node_modules"
WEB_DIST := ROOT + "/dist"

_default:
    @just --list

### All
#
build: wasm-build-all && web-build
    npm i
#
clean: wasm-clean-all web-clean

### Rust Crates


### WebAssembly
# build all wasm crate
wasm-build-all: (wasm-build "mnist-rs") (wasm-build "unpack")
# clean wasm output
wasm-clean-all:
    rm -r {{WASM_TARGET}}
# build selected wasm crate
wasm-build crate:
	wasm-pack build \
		--target web \
		--release \
		--scope wasm \
		-d {{WASM_TARGET}}/{{crate}} \
		{{WASM}}/{{crate}}

### Web
#
web-build:
    npx vite build {{WEB}} --outDir {{WEB_DIST}} --emptyOutDir
#
web-clean:
    rm -r {{WEB_DIST}}
#
web-dev:
    npx vite {{WEB}}
#
web-preview:
    npx vite preview
