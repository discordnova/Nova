# Build nova all-in-one bin
all:
	# Creates bin folder for artifacts
	@mkdir -p build/bin
	@mkdir -p build/lib
	
	@echo "Using cc, go, rust and ld versions"
	cc -v
	go version
	rustc --version
	ld -v

	# Builds rust
	@echo "Building rust project"
	cargo build --release
	@cp target/release/liball_in_one.a build/lib/
	@cp target/release/cache build/bin/
	@cp target/release/gateway build/bin/
	@cp target/release/ratelimit build/bin/
	@cp target/release/rest build/bin/
	@cp target/release/webhook build/bin/

	# Builds go
	go build -a -ldflags '-s' -o build/bin/nova cmd/nova/nova.go

docker-images:
	docker-compose build

docker-push:
	docker-compose push

rust-test:
	cargo test

test: rust-test

.PHONY: all docker-images docker-push test rust-test
