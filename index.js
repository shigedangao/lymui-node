const lib = require('./build/Release/lymuilib')

const ERRORS = {
  MISSING_ARGS: 'Missing arguments'
}

/**
 * Check Config Emptyness
 * 
 * @param {Object} config 
 * @throw
 */
const checkConfigEmptyness = config => {
  if (
    typeof config === 'undefined' ||
    config === null
  ) {
    throw { err: ERRORS.MISSING_ARGS }
  }
}

module.exports = {
  /**
   * Convert Regular
   *    Proxy of the Convert Regular liblymui method
   * 
   * @param {Object} config
   * @return {Promise}
   */
  convertRegular: config => {
    checkConfigEmptyness(config)
    if (config.hasOwnProperty('scale')) {
      config.scale = config.scale.toLowerCase()
    }

    return lib.convertRegular(config)
  },
  /**
   * Convert Space
   *    Smart reference to the lib.convertSpace method
   * 
   * @return {Promise}
   */
  convertSpace: config => {
    checkConfigEmptyness(config)
    return lib.convertSpace(config)
  },
  /**
   * To RGB
   *    Smart reference to the lib.toRGB method
   * 
   * @return {Promise}
   */
  toRGB: config => {
    checkConfigEmptyness(config)
    return lib.toRGB(config)
  },
  /**
   * To XYZ
   *    Smart Reference to the lib.toXYZ method
   * 
   * @return {Promise}
   */
  toXYZ: config => {
    checkConfigEmptyness(config)
    return lib.toXYZ(config)
  }
}