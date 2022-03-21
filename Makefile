#!/bin/sh
.PHONY: wasm web

wasm:
	cd wasm; make all

web: wasm
	cd web; npm install; npm run build
