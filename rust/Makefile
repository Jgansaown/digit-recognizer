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
		--scope wasm \
		-d ../../target/pkg/mnist-rs \
		./wasm/mnist-rs

build-unpack:
	wasm-pack build \
		--target web \
		--release \
		--scope wasm \
		-d ../../target/pkg/unpack \
		./wasm/unpack


build-mnist-rs-rayon:
	RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
		rustup run nightly-2022-04-07 \
		wasm-pack build \
		--target web \
		-d ../../target/mnist-rs/pkg \
		./wasm/mnist-rs \
		-- \
		--features multithread \
		-Z build-std=panic_abort,std 
