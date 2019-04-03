import lib from '../../@ts/index'
import { expect } from 'chai'
import { Output } from '../../@ts/output'
import { Rgb, Xyz, Cymk, Lab, Hex } from '../../@ts/inputs'
import 'mocha'

describe('Creating Regular type (fromRGB)', () => {
  it('Expect to create an hsl', async () => {
    const hsl = await lib.fromRGB<Rgb, Output>({
      input: {
        r: 5,
        g: 10,
        b: 95
      },
      output: 'hsl',
      clamp: 10
    })

    expect(hsl.data).to.be.deep.equal({
      h: 237,
      s: 90,
      l: 19.6
    })
  })

  it('Expect to throw an error if the input is not correct', async () => {
    try {
      await lib.fromRGB<Rgb, Output>({
        input: {
          r: 5,
          g: 10,
          b: 95
        },
        output: 'ddddd',
        clamp: 10
      })
    } catch(e) {
      expect(e).to.be.deep.equal({
        err: 'This color format is not supported by the library'
      })
    }
  })
})

describe('Creating Space type (fromXYZ)', () => {
  it('Expect to create a space type', async () => {
    const lab = await lib.fromXYZ<Xyz, Output>({
      input: {
        x: 0.9505,
        y: 1,
        z: 1.0888
      },
      output: 'lab',
      clamp: 1
    })

    expect(lab.data).to.be.deep.equal({
      l: 100,
      a: 0.0,
      b: 0.0
    })
  })

  it('Expect to throw an error when a type is not supported', async () => {
    try {
      await lib.fromXYZ<Xyz, Output>({
        input: {
          x: 0.9505,
          y: 1,
          z: 1.0888
        },
        output: 'lol',
        clamp: 1
      })
    } catch(e) {
      expect(e).to.be.deep.equal({
        err: 'This color format is not supported by the library'
      })
    }
  })
})

describe('Creating RGB from a Cymk (toRGB)', () => {
  it('Expect to create RGB value from Cymk', async () => {
    const rgb = await lib.toRGB<Cymk, Output>({
      input: {
        c: 0.973,
        y: 0,
        m: 0.949,
        k: 0.223
      },
      type: 'cymk'
    })

    expect(rgb.data).to.be.deep.equal({
      r: 5,
      g: 10,
      b: 198
    })
  })

  it('Expect to throw an error when the type is not supported', async () => {
    try {
      await lib.toRGB<Cymk, Output>({
        input: {
          c: 0.973,
          y: 0,
          m: 0.949,
          k: 0.223
        },
        type: 'ly'
      })
    } catch(e) {
      expect(e).to.be.deep.equal({
        err: 'This color format is not supported by the library'
      })
    }
  })
})

describe('Creating an XYZ from a Lab type (toXYZ)', () => {
  it('Expect to create an XYZ from a Lab', async () => {
    const { data } = await lib.toXYZ<Lab, Output>({
      input: {
        l: 100,
        a: 0,
        b: 0
      },
      type: 'lab',
      clamp: 10000
    })

    expect(data).to.be.deep.equal({
      x: 0.9505,
      y: 1,
      z: 1.0888
    })
  })

  it('Expect to throw when the type is not supported', async () => {
    try {
      await lib.toXYZ<Lab, Output>({
        input: {
          l: 100,
          a: 0,
          b: 0
        },
        type: 'mn',
        clamp: 10000
      })
    } catch (e) {
      expect(e).to.be.deep.equal({
        err: 'This color format is not supported by the library'
      })
    }
  })
})

describe('Creating RGB from Hex', () => {
  it('Expect to create an HEX from an RGB', async () => {
    const { data } = await lib.toRGB<Hex, Output>({
      input: 'FFFFFF',
      type: 'hex'
    })

    expect(data).to.be.deep.equal({
      r: 255,
      g: 255,
      b: 255
    })
  })
})
