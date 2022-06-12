.PHONY: kmeans mnist tar

all: build-kmeans build-gz

build-%:
	wasm-pack build \
		--target web \
		--release \
		--scope wasm \
		-d ../target/pkg/$* \
		$*

build-mnist-rs:
	wasm-pack build \
		--target web \
		--release \
		-d ../../target/pkg/mnist-rs \
		./wasm/mnist-rs
