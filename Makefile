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

# https://www.sea-ql.org/sea-orm-tutorial/ch01-02-migration-cli.html
#.PHONY: migrate-db
# migrate-db:
# 	sea-orm-cli migrate init
# 	rm -rf ./migration/src/m20220101_000001_create_table.rs
# 	@echo create table
# 	sea-orm-cli migrate generate create_table_users

# https://www.sea-ql.org/sea-orm-tutorial/ch01-04-entity-generation.html
# deeply nested directory as output doesn't work
.PHONY: generate-entity-from-db
generate-entity-from-db:
	rm -rf src/schemas
	sea-orm-cli generate entity -u postgresql://admin:admin@127.0.0.1:5432/example -o src/schemas --with-serde both

#------------------------------------------------------------------------------
# docker
#------------------------------------------------------------------------------
.PHONY: build-image
build-image:
	docker compose build

.PHONY: up-db
up-db:
	docker compose up

.PHONY: reset-db
reset-db:
	docker compose down -v
	docker compose up


# docker container exec -it {container_id} bash
#  or
# docker container exec -it actix-web-postgresql bash
# 
# > psql -U postgres example
# > \d users

#------------------------------------------------------------------------------
# Test Server
#------------------------------------------------------------------------------
.PHONY: req
req:
	#hurl --verbose ./scripts/admin.hurl
	hurl --very-verbose ./scripts/admin.hurl
	

.PHONY: req-sh
req-sh:
	./scripts/req.sh

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

#------------------------------------------------------------------------------
# Monitoring
#------------------------------------------------------------------------------

.PHONY: tcpdump
tcpdump:
	sudo tcpdump -i lo0 port 8080 -vv
