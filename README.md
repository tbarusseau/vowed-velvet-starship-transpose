# How to compile and run

## Rust library

```sh
$ cd rust
$ ./build.sh
```

This will build the library and copy them to the `c_caller` directory.

## C caller (FFI)

```sh
$ cd c_caller
$ ./build-and-run.sh # Requires bazel
$ make && make run
```

This will use the FFI bindings to test the arithmetics on polynomials, and generate random polynomials.

## Symbolic execution

```sh
$ cd rust
$ cargo run --example symbolic_execution
```
