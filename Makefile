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
.PHONY: all

$(TARGET):
	$(CARGO) build $(CARGO_FLAGS)

install:
	install -Dm755 $(TARGET) $(PREFIX)/bin/
	COMPLETE=bash $(TARGET) | install -Dm0644 /dev/stdin $(PREFIX)/share/bash-completion/completions/explain
	COMPLETE=fish $(TARGET) | install -Dm0644 /dev/stdin $(PREFIX)/share/fish/vendor_completions.d/rustic.fish/explain.fish
	COMPLETE=zsh $(TARGET) | install -Dm0644 /dev/stdin $(PREFIX)/share/zsh/site-functions/_explain
.PHONY: install

examples: $(EXAMPLES:.json=.png)
.PHONY: examples

%.dot: %.json $(TARGET)
	$(TARGET) --dry-run --file $< --output $@

%.png: %.dot
	dot -Tpng $^ > $@

test: examples
	git diff examples/*.json
	git diff-index --quiet HEAD examples/*.json
.PHONY: test

.PRECIOUS: %.dot
