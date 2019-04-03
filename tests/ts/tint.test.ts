import lib from '../../@ts/index'
import { expect } from 'chai'
import { OutputOperation } from '../../@ts/output'
import { Rgb } from '../../@ts/inputs'
import 'mocha'

describe('Creating a tint', () => {
  it('Expect to create a tint from a RGB value', async () => {
    const tint = await lib.fromRGB<Rgb, OutputOperation>({
      input: {
        r: 100,
        g: 150,
        b: 255
      },
      output: 'tint'
    })

    expect(tint.data[0]).to.not.be.empty
  })

  it('Expct to create a bright tint', async () => {
    const tint = await lib.fromRGB<Rgb, OutputOperation>({
      input: {
        r: 255,
        g: 255,
        b: 255
      },
      output: 'tint'
    })

    expect(tint.data).to.not.be.empty
    expect(tint.data[1].l).to.be.equal(100)
  })

  it('Expect to create a dark tint', async () => {
    const tint = await lib.fromRGB<Rgb, OutputOperation>({
      input: {
        r: 0,
        g: 0,
        b: 0
      },
      output: 'tint'
    })

    expect(tint.data).to.not.be.empty
    expect(tint.data[1].l).to.be.equal(10)
  })
})