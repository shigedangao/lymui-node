const { expect } = require('chai')
const lib = require('../index')

describe('Creating TSL from RGB', () => {
  it('Expect to create TSL object from RGB object', async () => {
    const rgb = {
      r: 50,
      g: 10,
      b: 128
    }

    const tsl = await lib.fromRGB({
      input: rgb,
      output: 'tsl',
      clamp: 1000
    })

    expect(tsl.data).to.be.deep.equal({
      t: 0.788,
      s: 0.387,
      l: 35.412
    })
  })

  it('Expect to create a bright TSL from white RGB', async () => {
    const rgb = {
      r: 255,
      g: 255,
      b: 255
    }

    const tsl = await lib.fromRGB({
      input: rgb,
      output: 'tsl',
      clamp: 1000
    })

    expect(tsl.data).to.be.deep.equal({
      t: 0.0,
      s: 0.0,
      l: 255
    })
  })

  it('Expect to create a dark TSL from black RGB', async () => {
    const rgb = {
      r: 0,
      g: 0,
      b: 0
    }

    const tsl = await lib.fromRGB({
      input: rgb,
      output: 'tsl',
      clamp: 1000
    })

    expect(tsl.data).to.be.deep.equal({
      t: 0.0,
      s: 0.0,
      l: 0.0
    })
  })

  it('Expect to throw an error if the object has a missing parameter', async () => {
    const rgb = {
      r: 0,
      g: 0
    }

    try {
      await lib.fromRGB({
        input: rgb,
        output: 'tsl',
        clamp: 1000
      })
    } catch (e) {
      expect(e).to.be.deep.equal({
        err: 'Wrong argument(s) type'
      })
    }
  })
})

describe('Creating RGB from TSL', () => {
  it('Expect to create a RGB from a TSL', async () => {
    const tsl = {
      t: 0.788,
      s: 0.387,
      l: 35.412
    }

    const rgb = await lib.toRGB({
      input: tsl,
      type: 'tsl'
    })

    expect(rgb.data).to.be.deep.equal({
      r: 50,
      g: 10,
      b: 128
    })
  })

  it('Expect to create a white RGB from a bright TSL', async () => {
    const tsl = {
      t: 0.0,
      s: 0.0,
      l: 255
    }

    const rgb = await lib.toRGB({
      input: tsl,
      type: 'tsl'
    })

    expect(rgb.data).to.be.deep.equal({
      r: 255,
      g: 255,
      b: 255
    })
  })

  it('Expect to create a black RGB from a dark TSL', async () => {
    const tsl = {
      t: 0.0,
      s: 0.0,
      l: 0.0
    }

    const rgb = await lib.toRGB({
      input: tsl,
      type: 'tsl'
    })

    expect(rgb.data).to.be.deep.equal({
      r: 0,
      g: 0,
      b: 0
    })
  })

  it('Expect to throw an error if the object is malformated', async () => {
    const tsl = {
      t: 0.0,
      s: 0.0,
      l: 'lol'
    }

    try {
      await lib.toRGB({
        input: tsl,
        type: 'tsl'
      })
    } catch (e) {
      expect(e.message).to.be.equal('An error occured when converting JS type to C type')
    }
  })
})
