const { expect } = require('chai')
const SegfaultHandler = require('segfault-handler')
const lib = require('../index')

SegfaultHandler.registerHandler('argb_crash.log')

describe('Creating ARGB from Xyz', () => {
  it('Expect to create ARGB from Adobe profile XYZ Object', async () => {
    const xyz = await lib.fromRGB({
      input: {
        r: 50,
        g: 10,
        b: 95
      },
      output: 'xyz',
      profile: 'adobe'
    })

    const { data } = await lib.fromXYZ({
      input: xyz.data,
      output: 'argb',
      clamp: 100
    })

    expect(data).to.be.deep.equal({
      r: 0.2,
      g: 0.04,
      b: 0.37
    })
  })

  it('Expect to create dark ARGB from XYZ Object', async () => {
    const xyz = await lib.fromRGB({
      input: {
        r: 0,
        g: 0,
        b: 0
      },
      output: 'xyz'
    })

    const { data } = await lib.fromXYZ({
      input: xyz.data,
      output: 'argb',
      clamp: 100
    })

    expect(data).to.be.deep.equal({
      r: 0,
      g: 0,
      b: 0
    })
  })

  it('Expect to create white ARGB from XYZ Object', async () => {
    const xyz = await lib.fromRGB({
      input: {
        r: 255,
        g: 255,
        b: 255
      },
      output: 'xyz'
    })

    const { data } = await lib.fromXYZ({
      input: xyz.data,
      output: 'argb',
      clamp: 100
    })

    expect(data).to.be.deep.equal({
      r: 1.0,
      g: 1.0,
      b: 1.0
    })
  })

  it('Expect to create white XYZ from Argb', async () => {
    const xyz = await lib.toXYZ({
      input: {
        r: 1.0,
        g: 1.0,
        b: 1.0
      },
      type: 'argb',
      clamp: 10000
    })

    expect(xyz.data).to.be.deep.equal({
      x: 0.9505,
      y: 1.0000,
      z: 1.0888
    })
  })

  it('Expect to create black XYZ from Argb', async () => {
    const xyz = await lib.toXYZ({
      input: {
        r: 0.0,
        g: 0.0,
        b: 0.0
      },
      type: 'argb',
      clamp: 10000
    })

    expect(xyz.data).to.be.deep.equal({
      x: 0.0,
      y: 0.0,
      z: 0.0
    })
  })

  it('Expect to create a color XYZ from Argb', async () => {
    const xyz = await lib.toXYZ({
      input: {
        r: 0.196089,
        g: 0.039087,
        b: 0.372496
      },
      type: 'argb',
      clamp: 10000
    })

    expect(xyz.data).to.be.deep.equal({
      x: 0.0376,
      y: 0.0173,
      z: 0.1138
    })
  })
})
