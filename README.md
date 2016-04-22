# csv_to_json
Takes in a CSV file and transforms it to JSON. Each line of the CSV file gets it's own line in the JSON file.

![travis-ci build status](https://travis-ci.org/divyekapoor/csv_to_json.svg?branch=master)

```sh
$ cat input.csv
Name,Age,Country
Divye,7,USA
Divya,6,India
```

```sh
$ csv_to_json < input.csv
[{"Name": "Divye","Age": "7","Country": "USA"},
{"Name": "Divya","Age": "6","Country": "India"}]
```

# Install

```sh
$ ./configure
$ make
$ sudo make install
```

for Debian and Debian-like systems (eg. Ubuntu). The only dependency is the Rust compiler.
Compiled with Rust 1.7.

