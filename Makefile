.PHONY: all
all:
	make -C wasm-code/
	make -C wasmer/
	cd wasmer && ./fhewasmer