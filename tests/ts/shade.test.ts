import lib from '../../@ts/index'
import { expect } from 'chai'
import { OutputOperation } from '../../@ts/output'
import { Rgb } from '../../@ts/inputs'
import 'mocha'

describe('Creating a shade', () => {
  it('Expect to create a shade from a RGB value', async () => {
    const shade = await lib.fromRGB<Rgb, OutputOperation>({
      input: {
        r: 100,
        g: 150,
        b: 255
      },
      output: 'shade'
    })

    expect(shade.data).to.not.be.empty
  })

  it('Expect to create a lesser dark shade', async () => {
    const shade = await lib.fromRGB<Rgb, OutputOperation>({
      input: {
        r: 255,
        g: 255,
        b: 255
      },
      output: 'shade'
    })

    expect(shade.data).to.not.be.empty
    expect(shade.data[1].l).to.be.equal(90)
  })

  it('Expect to create a full dark shade', async () => {
    const shade = await lib.fromRGB<Rgb, OutputOperation>({
      input: {
        r: 0,
        g: 0,
        b: 0
      },
      output: 'shade'
    })

    expect(shade.data).to.not.be.empty
    expect(shade.data[1].l).to.be.equal(0)
  })
})