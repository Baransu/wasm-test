release:
	@cargo web build --release
	@rm -rf dist
	@mkdir dist
	@mkdir dist/js
	@cp target/asmjs-unknown-emscripten/release/wasm-test.js dist/js/app.js
	@cp static/index.html dist/index.html