PROGRAM := jlsort
SHELL   := /bin/bash
VERSION := v$(shell cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "$(PROGRAM)").version')
TARGET  := x86_64-apple-darwin
#TARGET := x86_64-unknown-linux-gnu

.PHONY: all
all: test build

.PHONY: build
build:
	cross build --target $(TARGET) --release

.PHONY: package
package: clean build
	gzip target/$(TARGET)/release/$(PROGRAM) -c > $(PROGRAM)_$(VERSION)_$(TARGET).gz
	sha1sum $(PROGRAM)_$(VERSION)_$(TARGET).gz > $(PROGRAM)_$(VERSION)_$(TARGET).gz.sha1sum

.PHONY: clean
clean:
	rm -rf target

.PHONY: test
test:
	cargo test
