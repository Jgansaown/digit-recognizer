.PHONY: kmeans mnist tar

all: build-kmeans build-mnist build-wasm-tar

build-%:
	wasm-pack build -d ../target/wasm/$* --target web --release $* --scope wasm
