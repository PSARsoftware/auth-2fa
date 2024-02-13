build:
	cargo build

start:
	cargo run

dev:
	cargo watch --env-file .env -x  run

#start_docker:
#	docker-compose -f docker-compose.no_api.yml up -d
#
#stop_docker:
#	docker-compose -f docker-compose.no_api.yml down

install_sqlx:
	cargo install sqlx-cli --no-default-features --features postgres

db_migrate:
	sqlx migrate run

db_revert:
	sqlx migrate revert

run_prepared_sqlx:
	cargo sqlx prepare