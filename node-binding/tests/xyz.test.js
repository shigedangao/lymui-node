const binding = require('../node-binding.node')

test('Expect to create an Xyz from an RGB', () => {
    const res = binding.getAnyXyzCompatibleColor(
        { x: 0.950470, y: 1.0, z: 1.088830 },
        binding.XyzMapping.Xyz,
        binding.XyzMapping.Hcl,
    )

    expect(res.h).toBeDefined()
    expect(res.c).toBeDefined()
    expect(res.l).toBeDefined()
})

test('Expect to return an error if one param is missing', () => {
    expect(() => {
        binding.getAnyXyzCompatibleColor(
            { x: 0.950470, y: 1.0 },
            binding.XyzMapping.Xyz,
            binding.XyzMapping.Hcl,
        )
    }).toThrow('InvalidArg') 
})
