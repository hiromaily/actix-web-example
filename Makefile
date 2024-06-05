#------------------------------------------------------------------------------
# main
#------------------------------------------------------------------------------

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
# docker
#------------------------------------------------------------------------------
.PHONY: build-image
build-image:
	docker compose build

.PHONY: up-db
up-db:
	docker compose up

#------------------------------------------------------------------------------
# Test Server
#------------------------------------------------------------------------------
.PHONY: request
request:
	curl http://127.0.0.1:8080/hello
	@echo ""
	curl http://127.0.0.1:8080/hello/5/Bob
	@echo ""
	curl http://127.0.0.1:8080/health
	@echo ""
	curl -X POST -d '{"name":"Jecy", "age":"30"}' http://127.0.0.1:8080/echo
	@echo ""
	curl -X POST -H "Content-Type: application/json" -d '{"name":"Jecy", "age":30}' http://127.0.0.1:8080/echojson
	@echo ""
	curl http://127.0.0.1:8080/api/v1/info
	@echo ""
	curl http://127.0.0.1:8080/app/index.html
	@echo ""
	curl http://127.0.0.1:8080/api/v1/user

.PHONY: tcpdump
tcpdump:
	sudo tcpdump -i lo0 port 8080 -vv
