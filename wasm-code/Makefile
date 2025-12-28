wasm-code/target/wasm32-unknown-unknown/release/exports.wasm:
	cargo build --release --target wasm32-unknown-unknown --lib
	cp ../target/wasm32-unknown-unknown/release/exports.wasm ../build/

.PHONY: keygen
keygen:
	cargo build --release --bin keygen # --features seeder
	cp ../target/release/keygen .
	./keygen
