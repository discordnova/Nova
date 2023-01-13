clean:
	rm ./build/*

library:
	cargo build --release

build: library
	cp ../../target/release/liball_in_one.a ./build
	go build -a -ldflags '-s' -o build/all-in-one

all: library build

run: all-in-one
	./build/all-in-one

