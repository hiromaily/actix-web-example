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
# sea-orm
# Refer to
# - https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/
#------------------------------------------------------------------------------
.PHONY: setup-sea-orm
setup-sea-orm:
	cargo install sea-orm-cli
	sea-orm-cli migrate init
	rm -rf ./migration/src/m20220101_000001_create_table.rs
	@echo create table
	sea-orm-cli migrate generate create_table_users

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
	curl http://127.0.0.1:8080/api/v1/health
	@echo ""
	curl -X POST -d '{"user":"Jecy", "password":"xxxxx"}' http://127.0.0.1:8080/api/v1/admin/login
	@echo ""
	curl http://127.0.0.1:8080/api/v1/admin/users
	@echo ""
	curl -X POST -H "Content-Type: application/json" -d '{"name":"Jecy", "password":"xxxxx"}' http://127.0.0.1:8080/api/v1/admin/users
	@echo ""
	curl http://127.0.0.1:8080/api/v1/admin/users/1
	@echo ""
	curl -X PUT -H "Content-Type: application/json" -d '{"name":"Jecy", "password":"xxxxx"}' http://127.0.0.1:8080/api/v1/admin/users/1
	@echo ""
	curl -X DELETE http://127.0.0.1:8080/api/v1/admin/users/1
	@echo ""
	curl -X POST -d '{"user":"Jecy", "password":"xxxxx"}' http://127.0.0.1:8080/api/v1/app/login
	@echo ""
	curl http://127.0.0.1:8080/api/v1/app/users/1/todos
	@echo ""
	curl -X POST -d '{"user":"Jecy", "password":"xxxxx"}' http://127.0.0.1:8080/api/v1/app/users/1/todos
	@echo ""
	curl http://127.0.0.1:8080/api/v1/app/users/1/todos/1
	@echo ""
	curl -X PUT -H "Content-Type: application/json" -d '{"name":"Jecy", "password":"xxxxx"}' http://127.0.0.1:8080/api/v1/app/users/1/todos/1
	@echo ""
	curl -X DELETE http://127.0.0.1:8080/api/v1/app/users/1/todos/1
	@echo ""

# curl http://127.0.0.1:8080/hello
# @echo ""
# curl http://127.0.0.1:8080/hello/5/Bob
# @echo ""
# curl http://127.0.0.1:8080/health
# @echo ""
# curl -X POST -d '{"name":"Jecy", "age":"30"}' http://127.0.0.1:8080/echo
# @echo ""
# curl -X POST -H "Content-Type: application/json" -d '{"name":"Jecy", "age":30}' http://127.0.0.1:8080/echojson
# @echo ""
# curl http://127.0.0.1:8080/api/v1/info
# @echo ""
# curl http://127.0.0.1:8080/app/index.html
# @echo ""
# curl http://127.0.0.1:8080/api/v1/user

.PHONY: tcpdump
tcpdump:
	sudo tcpdump -i lo0 port 8080 -vv
