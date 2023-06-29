const binding = require('../node-binding.node')

test('Expect to create an Hex from an RGB', () => {
    const res = binding.getAnyRgbCompatibleColor(
        { r: 5, g: 10, b: 100 },
        binding.RgbMapping.Rgb,
        binding.RgbMapping.Hex
    )

    expect(res['0']).toBe('#050a64')
})

test('Expect to create an Ansi color from RGB', () => {
    const res = binding.getAnyRgbCompatibleColor(
        { r: 12, g: 17, b: 200 },
        binding.RgbMapping.Rgb,
        binding.RgbMapping.Ansi256
    )

    expect(res['0']).toBe(20)
})

test('Expect to create a Hsl from an RGB', () => {
    const res = binding.getAnyRgbCompatibleColor(
        { r: 12, g: 17, b: 200 },
        binding.RgbMapping.Rgb,
        binding.RgbMapping.Hsl
    )
    
    expect(res.h).toBeDefined()
    expect(res.s).toBeDefined()
    expect(res.l).toBeDefined()
})

test('Expect to return an error if color could not be parsed', () => {
    expect(() => {
        binding.getAnyRgbCompatibleColor(
            { r: 10, g: 10 },
            binding.RgbMapping.Rgb,
            binding.RgbMapping.Hsl
        )
    }).toThrow('InvalidArg')
})
