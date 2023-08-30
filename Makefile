# 

wasm-build:
	wasm-pack build \
		--target web \
		--scope wasm \
		--out-dir ../target/wasm/pkg \
		--release \
		./wasm

web-dev:
	npx vite ./web --host

web-build:
	npx vite build ./web --outDir ../docs --emptyOutDir --base=/digit-recognizer/

build:
	wasm-pack build \
		--target web \
		--scope wasm \
		--out-dir ../target/wasm/pkg \
		--release \
		./wasm
	npx vite build ./web --outDir ../docs --emptyOutDir --base=/digit-recognizer/

preview:
	npx vite preview ./web --outDir ../docs --host --base=/digit-recognizer/

distclean:
	rm -f -r ./node_modules ./target
