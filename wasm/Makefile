.PHONY: kmeans mnist tar

all: build-kmeans build-gz

build-%:
	wasm-pack build --target web --release --scope wasm -d ../target/pkg/$* $*
