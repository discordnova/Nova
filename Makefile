EXTENSION := 
ifeq ($(OS),Windows_NT)
	EXTENSION += .exe
endif
PROJECTS = $(shell find exes/ -mindepth 1 -maxdepth 1 -type d -printf '%f\n')
PLATFORMS :=
ifdef BUILDX_PLATFORMS
	PLATFORMS += --platform ${BUILDX_PLATFORMS}
endif

# Static libraries
target/release/lib%.a: libs/%
	cargo build --release -p $*

# Executables
target/release/%$(EXTENSION):
	cargo build --release -p $*

# Copy static libraries
build/lib/%: target/release/%
	@mkdir -p build/lib
	cp target/release/$* build/lib/$*

# Copy executables
build/bin/%$(EXTENSION): target/release/%$(EXTENSION)
	@mkdir -p build/bin
	cp target/release/$*$(EXTENSION) build/bin/$*$(EXTENSION)

# All in one binary
build/bin/nova$(EXTENSION): build/lib/liball_in_one.a
	@mkdir -p build/bin
	go build -a -ldflags '-s' -o build/bin/nova cmd/nova/nova.go

docker-%:
	
	
docker: $(PROJECTS:%=docker-%)
BINS=$(PROJECTS:%=build/bin/%$(EXTENSION))
bins: $(BINS) build/bin/nova$(EXTENSION)

all: docker bins

clean:
	rm -rf build
	rm -rf $(PROJECTS:%=target/release/%$(EXTENSION))
	rm -rf target/release/liball_in_one.a 

test:
	cargo test
	go test

.PHONY: clean all test
