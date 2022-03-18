.PHONY: kmeans mnist tar

all: build-kmeans build-mnist build-tar

build-%:
	wasm-pack build -d ../target/wasm/$* --target web --release $* --scope wasm
