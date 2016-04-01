SOURCES=$(wildcard src/*.rs)
BIN=csv_to_json

all: release

release: target/release/$(BIN)

debug: target/debug/$(BIN)

test: debug
	(./target/debug/csv_to_json < testdata/input.csv && tput bold && tput setaf 2 && echo PASS && tput sgr0) || (tput bold && tput setaf 1 && echo FAIL && tput sgr0)

clean:
	cargo clean

install: release
	install target/release/csv_to_json /usr/local/bin/

uninstall:
	$(RM) /usr/local/bin/csv_to_json

.PHONY: debug release clean install uninstall test all

target/debug/$(BIN): $(SOURCES)
	@echo Sources: $<
	cargo build

target/release/$(BIN): $(SOURCES)
	cargo build --release

