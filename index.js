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
    return Promise.reject({ err: ERRORS.MISSING_ARGS })
  }

  return Promise.resolve()
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
    return checkConfigEmptyness(config)
      .catch(err => Promise.reject(err))
      .then(() => {
        if (config.hasOwnProperty('scale')) {
          config.scale = config.scale.toLowerCase()
        }
      })
      .then(() => lib.convertRegular(config))
  },
  /**
   * Convert Space
   *    Smart reference to the lib.convertSpace method
   * 
   * @param {Object} config
   * @return {Promise}
   */
  convertSpace: config => {
    return checkConfigEmptyness(config)
      .catch(err => Promise.reject(err))
      .then(() => lib.convertSpace(config))
  },
  /**
   * To RGB
   *    Smart reference to the lib.toRGB method
   * 
   * @param {Object} config
   * @return {Promise}
   */
  toRGB: config => {
    return checkConfigEmptyness(config)
      .catch(err => Promise.reject(err))
      .then(() => lib.toRGB(config))
  },
  /**
   * To XYZ
   *    Smart Reference to the lib.toXYZ method
   * 
   * @param {Object} config
   * @return {Promise}
   */
  toXYZ: config => {
    return checkConfigEmptyness(config)
      .catch(err => Promise.reject(err))
      .then(() => lib.toXYZ(config))
  }
}