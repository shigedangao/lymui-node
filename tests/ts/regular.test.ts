import lib from '../../index'
import { expect } from 'chai'
import { Output } from '../../@ts/output'
import 'mocha'

describe('Creating Regular type', () => {
  it('Should create an hsl', async () => {
    const hsl = await lib.convertRegular<Output>({
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
})
