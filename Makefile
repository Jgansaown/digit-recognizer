wasm:
	wasm-pack build -d web/pkg --target web

serve-dev: wasm
	python3 -m http.server -d web 8000

build-pages:
	mkdir -p www
	wasm-pack build -d www/pkg --target web
	rm www/pkg/.gitignore
	cp web/* www

push-pages:
	cd www; git add --all; git commit -m "deploy to gh-pages"; git push origin gh-pages;
