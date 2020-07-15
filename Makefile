all: target/debug/app target/example.wasm
release: target/release/app target/example.wasm

target/debug/app:
	cargo build

target/example.wasm:
	mkdir target || true
	wasm-pack build plugin
	mv plugin/pkg/plugin_bg.wasm target/example.wasm

target/release/app:
	cargo build --release

clean:
	cargo clean
	rm example.wasm

test: target/example.wasm target/debug/app
	@echo "Example wasm filters any word with two or more"
	@echo "letter 'b's in a row"
	@echo ""
	@echo "Testing with abba"
	@echo ""
	./target/debug/app ./target/example.wasm abba
	@echo ""
	@echo "Testing with baab"
	@echo ""
	./target/debug/app ./target/example.wasm baab

test-release: target/example.wasm target/release/app
	@echo "Example wasm filters any word with two or more"
	@echo "letter 'b's in a row"
	@echo ""
	@echo "Testing with abba"
	@echo ""
	./target/release/app target/example.wasm abba
	@echo ""
	@echo "Testing with baab"
	@echo ""
	./target/release/app target/example.wasm baab
