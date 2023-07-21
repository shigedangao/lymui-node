## Lymui-node 🍭 (WIP)

A small wrapper over the [lymui](https://github.com/shigedangao/lymui) library. The wrapper provide binding for the specified language below

- C
- NodeJS

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
gcc rgb.c -o test -llymui_c -L../../target/debug
```

4. Run the example

```sh
LD_LIBRARY_PATH=../../target/debug ./test
```

## NodeJS

You'll need to have NodeJS installed on your machine.

1. Go to the `node-binding` folder and run the following command `npm run build`. This is going to output a `.node` file which you can use in your npm project

2. In a JS file file you can copy paste the following example.

```js
const binding = require('../node-binding.node')

const res = binding.getAnyRgbCompatibleColor(
    { r: 5, g: 10, b: 100 },
    binding.RgbMapping.Rgb,
    binding.RgbMapping.Hex
)
```
