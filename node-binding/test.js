const lymui = require('./node-binding.node')

const rgb = {
    r: 5,
    g: 10,
    b: 95
};

let res = lymui.getAnyColor(rgb, lymui.Mapping.Rgb, lymui.Mapping.Hex);
console.log(res);
