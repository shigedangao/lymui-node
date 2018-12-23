# convertSpace

This API allow you to convert an XYZ to the followings colors formats:

- argb
- srgb
- lab
- lch
- lchlab
- luv
- xyy

# Usage

Usage is pretty simple. The method take 3 parameters, 1 which is optional

- **input**: An Object representing an RGB object
- **output**: a string representing the supported colors (list above)
- **clamp**: An **optional** integer value which allow you to limit the precision of final value

# Example

Converting an **Xyz to an Lab**

```js
const lab = await lib.convertSpace({
  input: {
    x: 0.9505,
    y: 1,
    z: 1.0888
  },
  output: 'lab',
  clamp: 1
})

// output: { data: { lab: { l: 100,a: 0.0, b: 0.0}}}
```