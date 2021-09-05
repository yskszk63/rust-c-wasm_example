# What's this

Example of a WASM implementation calling C from Rust.

- [Simple](simple)
    - Simple WASM Library.
- [WASI](wasi)
    - Rust calling C with stdio.h.

## How to build.

simple

```
$ cd simple
$ AR=llvm-ar cargo build --target=wasm32-unknown-unknown
```

wasi

```
$ cd wasi
$ AR=llvm-ar CFLAGS='--sysroot /opt/wasi-sdk/wasi-sysroot' cargo wasi run
```
