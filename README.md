# octolog
Count bytes from a JSON log file

## Usage

`cargo run --release -- --help`

```
octolog 0.1.0
Aitor Ruano <codearm@pm.me>
Count bytes from a JSON log file

USAGE:
    octolog <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>    input log file to parse
```

## Build

`cargo build --release`

## Run example

`target/release/octolog examples/log.txt`

```
type       | count      | total bytes
A          | 1          | 8         
B          | 2          | 13
```
