import lib from '../../@ts/index'
import { expect } from 'chai'
import { Output } from '../../@ts/output'
import { Rgb } from '../../@ts/inputs'
import 'mocha'

describe('Creating Regular type', () => {
  it('Should create an hsl', async () => {
    const hsl = await lib.convertRegular<Rgb, Output>({
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

  it('Should throw an error if the input is not correct', async () => {
    try {
      await lib.convertRegular<Rgb, Output>({
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
