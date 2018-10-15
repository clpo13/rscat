# rscat

Simple clone of the GNU coreutils program cat(1) implemented in Rust. Give it
a list of files and it will print their contents to stdout.

## Building

Building from source requires Rust and Cargo (install them via rustup). In the
top-level source directory, run `cargo build`. To install it on your computer,
run `cargo install`. The default install path is in your `~/.cargo/bin` directory.

## Running

```text
Usage: rscat file1 file2 ...
Prints the contents of one or more files to stdout.

  --help     print a brief help message
  --version  output the program version number and license information
```

## License

This program is released under the terms of the [GNU GPL version 3](LICENSE)
or any later version. You may modify it or redistribute it freely according
to the conditions of the license.
