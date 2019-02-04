# Typescript (WIP)

## Installation

Typescript types are provided out of the box by the library. In order to use the typescript types first run this command.

```shell
cd node_modules/lymui-node/ && npm run build-ts
```

This will build the required necessary typing files under the namespace ```lymui-node/lib_ts```


## Usage

```ts
import lib from 'lymui-node/lib_ts';
import { Rgb } from 'lymui-node/lib_ts/inputs';
import { Output } from 'lymui-node/lib_ts/output';

lib.convertRegular<Rgb, Output>({
  input: {
    r: 50,
    g: 10,
    b: 98
  },
  output: 'hsl'
})
.then(res => console.warn(res))
.catch(err => console.error(err))
```

## APIs

The same JS APIs is exposed. Below is how you could use them

```ts
convertRegular<T, O>({}): Promise<O>
convertSpace<T, O>({}): Promise<O>
toRGB<T, O>({}): Promise<O>
toXYZ<T, O>({}): Promise<O>
```

- The ```T``` generic keyword represent the Color type that is given in input. This will make sure that the ```input``` object respect the color format ```T```

- The ```O``` generic keyword represent the Output of the input

### convertRegular

```ts
import lib from 'lymui-node/lib_ts';
import { Rgb } from 'lymui-node/lib_ts/inputs';
import { Output } from 'lymui-node/lib_ts/output';

const { data } = await lib.convertRegular<Rgb, Output>({
  input: {
    r: 50,
    g: 10,
    b: 98
  },
  output: 'hsl'
})
.then(res => console.warn(res))
.catch(err => console.error(err))
```

### convertSpace

```ts
const { data } = await lib.convertSpace<Xyz, Output>({
  input: {
    x: 0.9505,
    y: 1,
    z: 1.0888
  },
  output: 'lab',
  clamp: 1
})
```

### toRGB

```ts
const { data } = await lib.toRGB<Cymk, Output>({
  input: {
    c: 0.973,
    y: 0,
    m: 0.949,
    k: 0.223
  },
  type: 'cymk'
})
```

### toXYZ

```ts
const { data } = await lib.toXYZ<Lab, Output>({
  input: {
    l: 100,
    a: 0,
    b: 0
  },
  type: 'lab',
  clamp: 10000
})
```
