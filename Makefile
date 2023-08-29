# 

wasm-build:
	wasm-pack build \
		--target web \
		--scope wasm \
		--out-dir ../target/wasm/pkg \
		--dev \
		./wasm

web-dev:
	npx vite ./web --host

web-build:
	npx vite build ./web --outDir ../docs --emptyOutDir

build:
	wasm-pack build \
		--target web \
		--scope wasm \
		--out-dir ../target/wasm/pkg \
		--release \
		./wasm
	npx vite build ./web --outDir ../docs --emptyOutDir

preview:
	npx vite preview ./web --outDir ../docs --host

distclean:
	rm -f -r ./node_modules ./target
