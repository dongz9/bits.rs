# bits.rs

x86 BMI-style bit manipulation routines

## Synopsis

This library provides various bit manipulation routines not available
in Rust `std`. Currently it is just a portable implementation of the
x86 ABM, BMI 1/2, and TBM instruction sets but the API may expand in
the future. The implementation will (eventually) take advantage of
specialized hardware instructions available on the native platform.

## Documentation

See the API documentation [here](http://freebroccolo.github.io/bits.rs/doc/bits/).

## Requirements

1.   [Rust](http://www.rust-lang.org/)
2.   [Cargo](http://crates.io/)

You can install both with the following:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

See [Installing Rust](http://doc.rust-lang.org/guide.html#installing-rust) for further details.

## Usage

```
$ cargo build       ## build library/binary
$ cargo run         ## run examples
$ cargo test        ## run tests
```
