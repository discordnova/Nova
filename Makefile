EXTENSION := 
ifeq ($(OS),Windows_NT)
	EXTENSION += .exe
endif
dir_guard=@mkdir -p $(@D)
PROJECTS = $(shell find exes/ -mindepth 1 -maxdepth 1 -type d  -printf '%f\n')
BINS=$(PROJECTS:%=build/bin/%$(EXTENSION))

# Static libraries
target/release/lib%.a:
	cargo build --release -p $*

# Executables
target/release/%$(EXTENSION):
	cargo build --release -p $*

# Copy static libraries
build/lib/%: target/release/%
	$(dir_guard)
	cp target/release/$* build/lib

# Copy executables
build/bin/%$(EXTENSION): target/release/%$(EXTENSION)
	$(dir_guard)
	cp target/release/$*$(EXTENSION) build/lib/

# All in one binary
build/bin/nova$(EXTENSION): build/lib/liball_in_one.a
	$(dir_guard)
	go build -a -ldflags '-s' -o build/bin/nova cmd/nova/nova.go

all: $(BINS) build/bin/nova$(EXTENSION)

clean:
	rm -rf build
	rm -rf $(PROJECTS:%=target/release/%$(EXTENSION))
	rm -rf target/release/liball_in_one.a

test:
	cargo test
	go test

.PHONY: clean all test
