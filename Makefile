SOURCES=$(wildcard src/*.rs)

all: test

debug: $(SOURCES)
	@echo Sources: $<
	cargo build

release: $(SOURCES)
	cargo build --release

test: debug
	(./target/debug/csv_to_json < input.csv && tput bold && tput setaf 2 && echo PASS && tput sgr0) || (tput bold && tput setaf 1 && echo FAIL && tput sgr0)

clean:
	cargo clean

install: release
	install target/release/csv_to_json /usr/local/bin/

uninstall:
	$(RM) /usr/local/bin/csv_to_json
