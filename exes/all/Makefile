clean:
	rm ./build/libhello.so

library:
	cargo build --release

build: library
	cp ../../target/release/liball.so ./build
	go build -ldflags="-r build" -o build/all

all: library build

run: all
	./build/all
