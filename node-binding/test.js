const lymui = require('./node-binding.node')

const rgb = {
    r: 5,
    g: 10,
    b: 95
};

let res = lymui.getAnyRgbCompatibleColor(rgb, lymui.RgbMapping.Rgb, lymui.RgbMapping.Hex);
console.log(res)
