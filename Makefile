# Build nova all-in-one bin
all:
	# Creates bin folder for artifacts
	@mkdir -p build/{bin,lib}
	
	# Builds rust
	@echo "Building rust project"
	cargo build --release
	ls target/release/
	@cp target/release/liball_in_one.a build/lib
	@cp target/release/{cache,gateway,ratelimit,rest,webhook} build/bin

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
