#!/bin/sh
.PHONY: wasm web

wasm:
	cd wasm; $(MAKE) all

web: wasm
	cd web; npm install; npm run build
