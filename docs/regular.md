# convertRegular

This api allow you to convert an RGB to the followings colors foramts:

- hex
- hsl
- hsv
- ycbcr
- cymk
- xyz
- yuv
- grayscale
- hwb
- tsl

# Usage

Usage is pretty simple. The method take 4 parameters, 2 which are optionals

- **input**: An Object representing an RGB object
- **output**: a string representing the supported colors (list above)
- **clamp**: An **optional** integer value which allow you to limit the precision of final value
- **profile**: An **optional** string that is **only used for converting Rgb to Xyz** it can either take the value 'srgb' or 'adobe'. By default it is set to 'srgb'
- **scale**: An **optional** string that is **only used for converting Rgb to Grayscale**. It can contain the following values (lightness|luminosity|average|bt709|bt2100). By default scale is set to 'lightness'

# Examples

Converting an **Rgb to an Hex**

```js
const hex = await lib.convertRegular({
  input: {
    r: 5,
    g: 10,
    b: 98
  },
  output: 'hex'
})

// output: { data: { hex: '050a62' } }
```

Converting an **Rgb to an Xyz**

```js
const xyz = await lib.convertRegular({
  input: {
    r: 50,
    g: 10,
    b: 95
  },
  output: 'xyz',
  profile: 'srgb',
  clamp: 10000
})

// output { data: { x: 0.0376, y: 0.0174, z: 0.1138 }}
```

Converting an **Rgb to a Grayscale**

```js
const rgb = {
  r: 255,
  g: 0,
  b: 0
}

const gray = await lib.convertRegular({
  input: rgb,
  output: 'grayscale',
  scale: 'average'
})

// output { data: { gray: 85 }}
```