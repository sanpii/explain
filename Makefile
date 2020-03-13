CARGO=cargo
CARGO_FLAGS=
PREFIX?=/usr

ifneq ($(MODE),debug)
	TARGET=target/release/explain
	CARGO_FLAGS+=--release
else
	TARGET=target/debug/explain
endif

all: build

build:
	$(CARGO) build $(CARGO_FLAGS)

install:
	install --directory $(PREFIX)/bin
	install $(TARGET) $(PREFIX)/bin/

.PHONY: all build install
