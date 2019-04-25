## Lymui-node

Lymui-node is a small size color converter based on my Lymui C library. The library allow you to convert asynchronously colors.

The library support the following platforms:

- Linux
- OSX
- Windows (through mingw)

[![Build Status](https://dev.azure.com/androidgs2/lymui/_apis/build/status/MarcInthaamnouay.lymui-node?branchName=master)](https://dev.azure.com/androidgs2/lymui/_build/latest?definitionId=2&branchName=master)
![N-API bade v3](https://img.shields.io/badge/N--API-v3-green.svg)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/45d85b05a49647d68cf5ea9a53e1ced4)](https://www.codacy.com/app/mintha/lymui-node?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=MarcInthaamnouay/lymui-node&amp;utm_campaign=Badge_Grade)

## Version

1.2.0

## Getting start

### Install

The library is a native addons. Firstable let's install node-gyp

```shell
npm install node-gyp
```

When you have install the node-gyp you can then install the library

```shell
npm install lymui-node
```

### Example of usage

The API is pretty simple below is an example

```js
const lib = require('lymui-node');

const hsl = lib.fromRGB({
  input: {
    r: 100,
    g: 200,
    b: 200
  },
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

The bindings to the lymui library is located on the ```src``` folder.

## Contribute

Feel free to contribute :) Fork it and do whatever you want

## Changelog

The changelog is available over [here](docs/changelog.md)
