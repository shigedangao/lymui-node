const { expect } = require('chai')
const SegfaultHandler = require('segfault-handler')
const lib = require('../index')

SegfaultHandler.registerHandler('hcl_crash.log')

describe('Creating HCL from Xyz', () => {
  it('Expect to create an HCL from an XYZ Object', async () => {
    const xyz = await lib.convertRegular({
      input: {
        r: 50,
        g: 10,
        b: 95
      },
      output: 'xyz',
      profile: 'srgb'
    })
    
    const { data } = await lib.convertSpace({
      input: xyz.data,
      output: 'hcl',
      clamp: 1000
    })

    expect(data).to.be.deep.equal({
      h: 173.13,
      c: 40.075,
      l: 13.951
    })
  })
  
  it('Expect to create a bright HCL from an XYZ Object', async () => {
    const { data } = await lib.convertSpace({
      input: {
        x: 0.95047,
        y: 1.0,
        z: 1.08883
      },
      output: 'hcl',
      clamp: 1000
    })

    expect(data).to.be.deep.equal({
      h: 0,
      c: 0,
      l: 100
    })
  })

  it('Expect to create a dark HCL from an XYZ Object', async () => {
    const { data } = await lib.convertSpace({
      input: {
        x: 0.0,
        y: 0.0,
        z: 0.0
      },
      output: 'hcl',
      clamp: 1000
    })

    expect(data).to.be.deep.equal({
      h: 0,
      c: 0,
      l: 0
    })
  })
})