#!/bin/sh
.PHONY: wasm web

wasm:
	cd wasm; make all -j 4

web: wasm
	cd web; npm install; npm run build
