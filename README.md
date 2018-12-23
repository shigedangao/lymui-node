## Lymui-node

Lymui-node is a node module based on my Lymui library color converter. This node module allow you to convert asynchronously colors. The library has been tested on latest osx and ubuntu 18.04 & 16.04

![Travis logo](https://travis-ci.org/MarcInthaamnouay/lymui-node.svg?branch=master)

## Requirements

This node module is only available on linux & osx platform.
Window is currently not supported by the main library.

In order to run & install the module you need to have

- nodejs
- node-gyp
- python
- build-essential (linux)

## Example of usage

Below is an example of how to use the library

```js
const lib = require('lymui-node');

const rgb = {
  r: 100,
  g: 200,
  b: 200
};

const hsl = lib.convertRegular({
  input: rgb,
  output: 'hsl',
  clamp: 100
})
.then(res => console.log(res))
.catch(err => console.log(err))
```

For more examples please take a look at the docs: [Lymui-node docs](https://marcinthaamnouay.github.io/lymui-node/docs/)

## Test it locally

Test it locally should be easy. Please follow the steps below:

- Run npm install (This will import lymui lib for the target os)
- Run npm test (this run the JS test files)

## Bindings

If you want to debug the bindings files are located on the **src** folder.
Most of the binding is done in C. The entrypoint is *bootstrap.c*

## Support of a new color

If you want me to add a new color type or if you want to add a new color type please open an issue in this repo.