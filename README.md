# bits.rs

x86 BMI-style bit manipulation routines

## Synopsis

This library provides portable implementation of x86 BMI (Bit
Manipulation Instructions) 1, 2, TBM (Trailing Bit Manipulation
Instructions), and ABM (Advanced Bit Manipulation Instructions). It
will (eventually) use machine instructions if available on the native
architecture.

The following instructions are implemented:

#### ABM

* POPCNT (via Rust intrinsics)
* LZCNT (via Rust intrinsics)

#### BMI1

* ANDN
* BEXTR
* BLSI
* BLSMSK
* BLSR
* TZCNT (via Rust intrinsics)

#### BMI2

* BZHI
* PDEP
* PEXT

#### TBM

* BLCFILL
* BLCI
* BLCIC
* BLCMASK
* BLCS
* BLSFILL
* BLSIC
* T1MSKC
* TZMSK

## Documentation

See the API documentation [here](http://freebroccolo.github.io/bmi.rs/doc/bmi/).

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
