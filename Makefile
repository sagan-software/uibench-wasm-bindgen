.PHONY: build-wasm
build-wasm:
	cargo +stable build -vv --release --target=wasm32-unknown-unknown
	wasm-gc \
		target/wasm32-unknown-unknown/release/uibench.wasm \
		target/wasm32-unknown-unknown/release/uibench_gc.wasm
	wasm-opt --output \
		target/wasm32-unknown-unknown/release/uibench_gc_opt.wasm -O \
		target/wasm32-unknown-unknown/release/uibench_gc.wasm
	wasm-bindgen \
		target/wasm32-unknown-unknown/release/uibench.wasm \
		--out-dir target

dist:
	git worktree remove --force ./dist || exit 0
	git worktree add ./dist gh-pages

.PHONY: build
build: dist build-wasm
	`yarn bin`/webpack

.PHONY: start
start: build
	`yarn bin`/concurrently --raw --kill-others \
		"`yarn bin`/webpack-dev-server" \
		"cargo watch -s \"make build-wasm\""