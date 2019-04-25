const { expect } = require('chai')
const lib = require('../index')

describe('Creating a Grayscale from RGB', () => {
  it('Expect to create a Grayscale of type Lightness from RGB object', async () => {
    const rgb = {
      r: 255,
      g: 0,
      b: 0
    }

    const gray = await lib.fromRGB({
      input: rgb,
      output: 'grayscale'
    })

    expect(gray.data).to.be.deep.equal({
      gray: 128
    })
  })

  it('Expect to create a Grayscale of type Luminosity from RGB object', async () => {
    const rgb = {
      r: 255,
      g: 0,
      b: 0
    }

    const gray = await lib.fromRGB({
      input: rgb,
      output: 'grayscale',
      scale: 'luminosity'
    })

    expect(gray.data).to.be.deep.equal({
      gray: 54
    })
  })

  it('Expect to create a Grayscale of type Average from RGB object', async () => {
    const rgb = {
      r: 255,
      g: 0,
      b: 0
    }

    const gray = await lib.fromRGB({
      input: rgb,
      output: 'grayscale',
      scale: 'average'
    })

    expect(gray.data).to.be.deep.equal({
      gray: 85
    })
  })

  it('Expect to create a Grayscale of type bt709 from RGB object', async () => {
    const rgb = {
      r: 255,
      g: 0,
      b: 0
    }

    const gray = await lib.fromRGB({
      input: rgb,
      output: 'grayscale',
      scale: 'bt709'
    })

    expect(gray.data).to.be.deep.equal({
      gray: 54
    })
  })

  it('Expect to create a Grayscale of type bt2100 from RGB object', async () => {
    const rgb = {
      r: 255,
      g: 0,
      b: 0
    }

    const gray = await lib.fromRGB({
      input: rgb,
      output: 'grayscale',
      scale: 'bt2100'
    })

    expect(gray.data).to.be.deep.equal({
      gray: 67
    })
  })

  it('Expect to create a Grayscale of type BT2100 from RGB object', async () => {
    const rgb = {
      r: 255,
      g: 0,
      b: 0
    }

    const gray = await lib.fromRGB({
      input: rgb,
      output: 'grayscale',
      scale: 'BT2100'
    })

    expect(gray.data).to.be.deep.equal({
      gray: 67
    })
  })
})