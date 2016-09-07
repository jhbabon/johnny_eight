# Johnny Eight: A Chip-8 emulator

## Overview

This project is an experiment to try to learn more about rust and the concept of emulators. I wanted to use Chip-8 because it has a small set of instructions and it's well documented.

### What is CHIP-8?

The webpage [chip8.com](http://www.chip8.com/?page=73) explains it like this:

> CHIP-8 is an interpreted programming language, developed by the late Joseph Weisbecker. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s. CHIP-8 programs are run on a CHIP-8 virtual machine. It was made to allow video games to be more easily programmed for said computers.

## Requeriments

* Rust, the last stable release.
* SDL2.0 development libraries. You can check the [documentation on the official project](https://github.com/AngryLawyer/rust-sdl2#sdl20--development-libraries) to see how to install them.

## Installation

Clone the repository and install all the dependencies:

```
$ git clone git@github.com:jhbabon/johnny_eight.git
$ cd johnny_eight
$ cargo build
```

You can check that (almost) everything works by running the tests:

```
$ cargo test
```

## Usage

The binary program accepts the path to a ROM file. You can find many in the [chip8.com](http://www.chip8.com/?page=109) webpage.

Example

```
# run directly
$ cargo run --release -- fixtures/chip_8_logo.rom

# or build and run
$ cargo build --release
$ target/release/johnny_eight fixtures/chip_8_logo.rom
```

## Resources

Some useful projects and webpages about Chip-8:

* [chip8.com](http://www.chip8.com)
* [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
* [Mastering Chip-8](http://mattmik.com/files/chip8/mastering/chip8.html)
* [How to write an emulator CHIP-8 interpreter](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
* [CHIP-8 virtual machine implementation in the Rust programming language](https://github.com/chip8-rust/chip8-vm)
