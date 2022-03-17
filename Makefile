wasm:
	wasm-pack build -d web/pkg --target web --release

serve-dev: wasm
	python3 -m http.server -d web 8000

build-pages: wasm
	mkdir -p www
	cp -r web/* www
	rm www/pkg/.gitignore

push-pages:
	cd www; git add --all; git commit -m "deploy to gh-pages"; git push origin gh-pages;
