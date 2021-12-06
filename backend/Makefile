.PHONY:\
	dep\
	run\
	build\
	stop\
	check\
	format\
	lint\
	test\
	medium-test\

dep:
	docker-compose --profile dep up -d

run:
	docker-compose --profile dep --profile app up -d

build:
	docker build -t book_record_backend:latest .

stop:
	docker-compose --profile dep --profile app down

check: format lint test
	cargo check

format:
	cargo fmt

lint:
	cargo clippy -- --no-deps

test:
	cargo test

medium-test: dep
	sqlx database create --database-url postgres://book_record:book_record@localhost/test_book_record
	sqlx migrate run --database-url postgres://book_record:book_record@localhost/test_book_record
	-DATABASE_URL=postgres://book_record:book_record@localhost/test_book_record \
		REFRESH_EXP=20 \
		cargo test -- --ignored
	sqlx database drop -y --database-url postgres://book_record:book_record@localhost/test_book_record
