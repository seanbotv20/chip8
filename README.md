# Chip8

Chip8 is a [chip8](https://en.wikipedia.org/wiki/CHIP-8) emulator written in rust.

## Build

Chip8 requires cargo to be installed and can be built with:

```bash
cargo build
```

## Usage

Chip8 takes a path to a compiled chip8 program as its only argument

```bash
chip8 ./program.ch8
```

or

```bash
cargo run -- ./program.ch8
```

Chip8 currently does not support keyboard inputs
