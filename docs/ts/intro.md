## Typescript support

The library is supporting Typescript. A set of types is defined in the generated ```lib_ts``` folder.

### Getting start

In order to get typescript running with the library you'll need to run the command below

```shell
# This command is building the typing necessary for the typescript compiler
cd node_modules/lymui-node/ && npm run build-ts 
```

### Usage

Once you have run the command above you'll need to import the lymui library throught this typing. Below is an example

```ts
import lib from 'lymui-node/lib_ts';
import { Rgb, Hex } from 'lymui-node/lib_ts/inputs';
import { Output } from 'lymui-node/lib_ts/output';

lib.fromRGB<Rgb, Output>({
  input: {
    r: 50,
    g: 10,
    b: 98
  },
  output: 'hsl'
})
.then(res => {
  const hex = <Hex> res.data
})
.catch(err => console.error(err))
```

For further information regarding the typing take a look a the [typing documentation](typing.md)