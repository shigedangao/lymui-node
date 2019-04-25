## Typings

### Description

Below is the description of the typings available

The typings expose every APIs of the lymui-node library below is the list

```ts
fromRGB<T, O>({}): Promise<O>
fromXYZ<T, O>({}): Promise<O>
toRGB<T, O>({}): Promise<O>
toXYZ<T, O>({}): Promise<O>
```

Let's peel an example of those API. The explanation is applicable to all of those APIs

```ts
fromRGB<T, O>({}): Promise<O>
```

- T: T represent a type of Color
- O: O represent a desired type of Color
- Promise<O>: Return a promise of the type of the desired color

- T: lymui-node/lib_ts/inputs: A list of Color represent by a set of interface e.g Hsl interface
- O: lymui-node/lib_ts/output: A list of Output interface

### Examples

Example of converting an XYZ to a Lab

```ts
import lib from 'lymui-node/lib_ts';
import { Lab } from 'lymui-node/lib_ts/inputs'
import { Output } from 'lymui-node/lib_ts/output'; 

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

Example of converting for generating a Tint

```ts
import lib from 'lymui-node/lib_ts';
import { Rgb } from 'lymui-node/lib_ts/inputs'
import { Output } from 'lymui-node/lib_ts/output'; 

const tint = await lib.fromRGB<Rgb, OutputOperation>({
  input: {
    r: 100,
    g: 150,
    b: 255
  },
  output: 'tint'
})
```