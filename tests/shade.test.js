const { expect } = require('chai')
const SegfaultHandler = require('segfault-handler')
const lib = require('../index')

SegfaultHandler.registerHandler('shade_crash')

describe('Creating shades from RGB', () => {
  it('Expect to create a shade from an RGB', async () => {
    const rgb = {
      r: 50,
      g: 100,
      b: 100
    }

    const shade = await lib.fromRGB({
      input: rgb,
      output: 'shade'
    })

    expect(shade.data).to.not.be.empty
  })

  it('Expect to create a shade from a bright RGB', async () => {
    const rgb = {
      r: 255,
      g: 255,
      b: 255
    }

    const shade = await lib.fromRGB({
      input: rgb,
      output: 'shade'
    })

    expect(shade.data).to.not.be.empty
    expect(shade.data[1].l).to.be.equal(90)
  })

  it('Expect to create a shade from a dark RGB', async () => {
    const rgb = {
      r: 0,
      g: 0,
      b: 0
    }

    const shade = await lib.fromRGB({
      input: rgb,
      output: 'shade'
    })

    expect(shade.data).to.not.be.empty
    expect(shade.data[1].l).to.be.equal(0)
  })

  it('Expect to throw when no value is passed', async () => {
    try {
      await lib.fromRGB({
        output: 'shade'
      })
    } catch (e) {
      expect(e.err).to.be.equal('Missing arguments')
    }
  })
})