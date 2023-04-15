## Lymui-node 🍭

A small wrapper over the [lymui](https://github.com/shigedangao/lymui) library. The wrapper provide binding for the specified language below

- C 🚧
- NodeJS (TBD)

## C Binding

Uses these commands

1. Compile the header for the C library

```sh
cbindgen --config cbindgen.toml --crate lymui-c --output my_header.h --lang c
```

2. Compile lymui-node

```sh
cargo build 
```

3. Compile the example with GCC

```sh
gcc test.c -o test -llymui_c -L../../target/debug
```

4. Run the example

```sh
LD_LIBRARY_PATH=../../target/debug ./test
```