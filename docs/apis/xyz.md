## XYZ APIs

The library is providing an easy way to convert an XYZ back and forth. You can find the APIs below.

## fromXYZ API

The API allow you to convert an XYZ color to a supported target color.

### Signature

- input: an object representing an XYZ Object
- output: a string which represent the desired [color format](../index.md)
- clamp: an **optional** number (integer) value that clamp the output values

### Returned values

- Return a promise
* * data: contain the datas
* * error: contain a String which is representing an error

### Example of usage

```js
const lib = require('lymui-node')

async function getLAB() {
  const lab = await lib.fromXYZ({
    input: {
      x: 0.9505,
      y: 1,
      z: 1.0888
    },
    output: 'lab',
    clamp: 1000
  })

  return lab
}
```

## toXYZ API

The API allow you to convert a supported color to an XYZ color.

### Signature

- input: an object representing a color
- type: a string representing the desired [color format](../index.md)
- clamp: an **optional** number (integer) value that clamp the output values

### Returned values

- Return a promise
* * data: contain the datas
* * error: contain a String which is representing an error

### Example of usage

```js
const lib = require('lymui-node')

async function getXYZ() {
  const xyz = await lib.toXYZ({
    input: {
      l: 13.951,
      a: 37.071,
      b: -41.431
    },
    type: 'lab'
  })

  return xyz
}
```
