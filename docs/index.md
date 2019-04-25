# Lymui-node

This library is a NodeJS library which allow you to convert color to a panel of other formats. This library is based on my personal color conversion library Lymui. Below you can find a table which show you the current format supported.

| Base Color                  | Format                                                                  |
|-----------------------------|-------------------------------------------------------------------------|
| RGB                         | - Hex - Hsl - Hsv - Hwb - Tsl - Cymk - YCbCr - Yuv                      |
| Xyz                         | - Lab - Luv  - Lch(uv) - Lch(Lab) - Xyy - Lab(Hunter lab) - Argb - Srgb |
| Rgb operations (output Hsl) | - Tint - Shade                                                          |

# Getting started

The library is a native addons which is using ```N-API``` bindings to NodeJS. The library has been tested on NodeJS 10.x.x and above. It currently support the following OS

- Linux
- OSX
- Windows

## Installation

In order to install the library you'll need to install ```node-gyp```. Install it by running the command

```shell
npm install -g node-gyp
```

Secondly you can then install the library with this command

```shell
npm install lymui-node
```

## Example of usage

The library is simple to use as an example we're going to convert an RGB to an Hexadecimal

```js
const lib = require('lymui-node')

const hex = await lib.fromRGB({
  input: {
    r: 100,
    g: 200,
    b: 200
  },
  output: 'hex'
})

// Hex is equal to
{ data: { hex: '64c8c8' } }
```

## Going further

You can consult the example of usage of the API here

- [Converting from & to RGB](apis/rgb.md)
- [Converting from & to XYZ](apis/xyz.md)

## Typescript

The library support TypeScript. Please check the following 2 links below.

- [Using typesript](ts/intro.md)
- [Available typings](ts/typing.md)

## Contributions

Contributions are always welcome. Feel free to fork and do whatever you want with it :D
