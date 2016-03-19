all: test

csv_to_json: csv_to_json.rs
	rustc $<

test: csv_to_json
	(./csv_to_json < input.csv && tput bold && tput setaf 2 && echo PASS && tput sgr0) || (tput bold && tput setaf 1 && echo FAIL && tput sgr0)

clean:
	$(RM) csv_to_json

install: csv_to_json
	install csv_to_json /usr/local/bin/

uninstall:
	$(RM) /usr/local/bin/csv_to_json
