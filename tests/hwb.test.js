const { expect } = require('chai')
const SegfaultHandler = require('segfault-handler')
const lib = require('../build/Release/lymuilib')

SegfaultHandler.registerHandler('hwb_crash.log')

describe('Creating HWB from RGB', () => {
  it('Expect to create HWB object from RGB object', async () => {
    const rgb = {
      r: 17,
      g: 12,
      b: 93
    };

    const hwb = await lib.convertRegular({
      input: rgb,
      output: 'hwb',
      clamp: 10
    })

    expect(hwb.data).to.be.deep.equal({
      h: 244,
      w: 4.7,
      b: 63.5
    })
  })

  it('Expect to create a bright HWB object from white RGB object', async () => {
    const rgb = {
      r: 255,
      g: 255,
      b: 255
    };

    const hwb = await lib.convertRegular({
      input: rgb,
      output: 'hwb',
      clamp: 10
    })

    expect(hwb.data).to.be.deep.equal({
      h: 0.0,
      w: 100.0,
      b: 0.0
    })
  })

  it('Expect to create a dark HWB object from black RGB object', async () => {
    const rgb = {
      r: 0,
      g: 0,
      b: 0
    };

    const hwb = await lib.convertRegular({
      input: rgb,
      output: 'hwb',
    })

    expect(hwb.data).to.be.deep.equal({
      h: 0.0,
      w: 0.0,
      b: 100.0
    })
  })

  it('Expect to throw an error if the object has one missing params', async () => {
    const rgb = {
      r: 17,
      g: 12,
    };

    try {
      await lib.convertRegular({
        input: rgb,
        output: 'hwb',
        clamp: 10
      })
    } catch (e) {
      expect(e).to.be.deep.equal({
        err: 'Wrong argument(s) type'
      })
    }
  })

  it('Expect to throw an error if the object is malformated', async () => {
    const rgb = {
      r: 17,
      g: 12,
      b: 'Hello'
    };

    try {
      await lib.convertRegular({
        input: rgb,
        output: 'hwb',
        clamp: 10
      })
    } catch (e) {
      expect(e.message).to.be.equal('An error occured when converting JS type to C type')
    }
  })
})

describe('Creating RGB from HWB', () => {
  it('Expect to create RGB from HWB', async () => {
    const hwb = {
      h: 244,
      w: 4.7,
      b: 63.5 
    }

    const rgb = await lib.toRGB({
      input: hwb,
      type: 'hwb'
    })

    expect(rgb.data).to.be.deep.equal({
      r: 17,
      g: 12,
      b: 93
    })
  })

  it('Expect to create a white RGB from bright HWB', async () => {
    const hwb = {
      h: 0,
      w: 100,
      b: 0 
    }

    const rgb = await lib.toRGB({
      input: hwb,
      type: 'hwb'
    })

    expect(rgb.data).to.be.deep.equal({
      r: 255,
      g: 255,
      b: 255
    })
  })

  it('Expect to create a black RGB from a dark HWB', async () => {
    const hwb = {
      h: 0,
      w: 0,
      b: 100 
    }

    const rgb = await lib.toRGB({
      input: hwb,
      type: 'hwb'
    })

    expect(rgb.data).to.be.deep.equal({
      r: 0,
      g: 0,
      b: 0
    })
  })

  it('Expect to throw when the object is malformated', async () => {
    const hwb = {
      h: 244,
      w: 4.7,
      c: 63.5 
    }

    try {
      await lib.toRGB({
        input: hwb,
        type: 'hwb'
      })  
    } catch (e) {
      expect(e).to.be.deep.equal({
        err: 'Wrong argument(s) type'
      })
    }
  })

  it('Expect to throw when the object is empty', async () => {
    try {
      await lib.toRGB({
        input: {},
        type: 'hwb'
      })  
    } catch (e) {
      expect(e).to.be.deep.equal({
        err: 'Wrong argument(s) type'
      })
    }
  })
})