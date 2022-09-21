default:
	rustup target add wasm32-unknown-unknown
	cargo build --target=wasm32-unknown-unknown
clean:
	rm -rf target
