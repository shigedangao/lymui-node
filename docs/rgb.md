# toRGB

This api allow you to convert any type listed below to an RGB:

- hex
- hsl
- hsv
- ycbcr
- cymk
- xyz
- yuv

# Usage

Usage is pretty simple. The method take 4 parameters, 2 which are optionals

- **input**: An Object representing an type <T> listed above
- **output**: a string representing the input color format
- **profile**: An **optional** string that is **only used for converting Rgb to Xyz** it can either take the value 'srgb' or 'adobe'. By default it is set to 'srgb'

# Eample

Converting an **Hex to RGB**

```js
const rgb = await lib.toRGB({
  input: '000000',
  type: 'hex'
})

// output: { data: { rgb: { r: 0, g: 0, b: 0 }}}
```

Converting an **Xyz to RGB**

```js
const rgb = await lib.toRGB({
  input: {
    x: 0.0376,
    y: 0.0173,
    z: 0.1138
  },
  type: 'xyz',
  profile: 'adobe'
})

// output { data: { r: 50, g: 10, b: 95 }}
```