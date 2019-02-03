# Typescript (WIP)

## Installation

Typescript types are provided out of the box from the library. In order to use the typescript types first run this command.

```shell
cd node_modules/lymui-node/ && npm run build-ts
```

This will build the required necessary typing files under the namespace ```lymui-node/lib_ts```

You can then use the library like so

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

WIP...