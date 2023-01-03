clean:
	rm ./build/*

library:
	cargo build --release

build: library
	cp ../../target/release/liball.a ./build
	go build -a -ldflags '-s' -o build/all

all: library build

run: all
	./build/all
