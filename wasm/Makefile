.PHONY: kmeans mnist tar

all: build-kmeans build-mnist build-gz

build-%:
	wasm-pack build -d ../target/wasm/$* --target web --release $* --scope wasm
