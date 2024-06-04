.PHONY: lint
lint:
	cargo fmt
	cargo clippy

.PHONY: fix
fix:
	cargo fix --allow-staged

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	RUST_LOG=debug cargo run -- ./config/settings.toml -d

.PHONY: compile
compile:
	rustc ./src/main.rs

.PHONY: test
test:
	cargo test

#------------------------------------------------------------------------------
# Test Server
#------------------------------------------------------------------------------

# curl http://127.0.0.1:8080
# curl http://127.0.0.1:8080/hey
# curl -X POST -d '{"name":"Jecy", "age":"30"}' http://127.0.0.1:8080/echo