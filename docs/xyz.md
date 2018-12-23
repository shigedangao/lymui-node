# toXYZ

This api allow you to convert any type listed below to an XYZ:

- argb
- srgb
- lab
- lch
- lchlab
- luv
- xyy

# Usage

Usage is pretty simple. The method take 4 parameters, 2 which are optionals

- **input**: An Object representing an type <T> listed above
- **output**: a string representing the input color format
- **clamp**: An **optional** integer value which allow you to limit the precision of final value

# Eample

Converting an **Xyz to an Lab**

```js
const { data } = await lib.toXYZ({
  input: {
    l: 13.951,
    a: 37.071,
    b: -41.431
  },
  type: 'lab',
  clamp: 10000
})

// output: { data: { xyz: { x: 0.0349, y: 0.0172, z: 0.1097}}}
```