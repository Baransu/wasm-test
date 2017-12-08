release:
	@cargo web build --release
	@rm -rf dist
	@mkdir dist
	@cp target/asmjs-unknown-emscripten/release/wasm-test.js dist/app.js
	@cp public/index.html dist/index.html