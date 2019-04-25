## RGB APIs

The library is providing an easy way to convert an rgb back and forth. You can find the APIs below.

## fromRGB API

The API allow you to convert an RGB color to a supported target color.

### Signature

- input: an object representing an RGB Object
- output: a string which represent the desired [color format](../index.md)
- clamp: an **optional** number (integer) value that clamp the output values
- profile: **parameter only use for the output of type xyz** it can either take ```srgb``` or ```argb```. By default the value is set to ```srgb```

### Returned values

- Return a promise
* * data: contain the datas
* * error: contain a String which is representing an error

### Example of usage

```js
const lib = require('lymui-node')

async function getHSL() {
  const hsl = await lib.fromRGB({
    input: {
      r: 9,
      g: 01,
      b: 19
    },
    output: 'hsl',
    clamp: 1000
  })

  return hsl
}
```

## toRGB API

The API allow you to convert a generated color to an RGB color.

### Signature

- input: an object representing a color
- type: a string representing the desired [color format](../index.md)
- profile: **parameter only use for the output of type xyz** it can either take ```srgb``` or ```argb```. By default the value is set to ```srgb```

### Returned values

- Return a promise
* * data: contain the datas
* * error: contain a String which is representing an error

### Example of usage

```js
const lib = require('lymui-node')

async function getRGB() {
  const rgb = await lib.toRGB({
    input: {
      x: 0.0348,
      y: 0.0172,
      z: 0.1097
    },
    type: 'xyz',
    profile: 'adobe'
  })

  return rgb
}
```

## Grayscale

Generating a grayscale use the same API with one more parameter in the signature of the method which is:

scale: an optional string which represent a type of grayscale (by default scale is set to lightness)

Below is an example

```js
const lib = require('lymui-node')

async function generateGrayscale() {
  const gray = await lib.fromRGB({
   input: {
      r: 255,
      g: 0,
      b: 0
   },
   output: 'grayscale',
   scale: 'bt2100'
 })

 return gray
}


// gray is equal to
{ data: { gray: 128 } }
```

The API support multiple type of grayscale. Below is the list of supported type of grayscale

- Lightness
- Average
- Luminosity
- bt709
- bt2100