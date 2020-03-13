CARGO=cargo
CARGO_FLAGS=
PREFIX?=/usr
EXAMPLES:=$(wildcard examples/*.json)

ifneq ($(MODE),debug)
	TARGET=target/release/explain
	CARGO_FLAGS+=--release
else
	TARGET=target/debug/explain
endif

all: $(TARGET)

$(TARGET):
	$(CARGO) build $(CARGO_FLAGS)

install:
	install --directory $(PREFIX)/bin
	install $(TARGET) $(PREFIX)/bin/

examples: $(EXAMPLES:.json=.png)

%.dot: %.json $(TARGET)
	$(TARGET) --dry-run --file $< --output $@ _

%.png: %.dot
	dot -Tpng $^ > $@

.PHONY: all build install
.PRECIOUS: %.dot
