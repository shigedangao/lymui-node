import lib from './@ts/binding'
import { Config, InputConfig, OutputConfig } from './@ts/config'

// Errors enum
enum Errors {
  MISSING_ARGS = 'Missing arguments'
}

/**
 * Check Config Emptyness
 * 
 * @param config
 */
function checkConfigEmptyness<T>(config: Config<T>) {
  if (
    typeof config === 'undefined' ||
    config === null
  ) {
    throw new Error(Errors.MISSING_ARGS)
  }
}

export default {
  /**
   * Convert Regular
   * 
   * @param {InputConfig} config
   * @return {Promise}
   */
  convertRegular<T>(config: InputConfig<T>): Promise<T> {
    checkConfigEmptyness(config)

    if (config.scale) {
      config.scale = config.scale.toLowerCase()
    }

    return lib.convertRegular(config)
  },
  /**
   * Convert Space
   * 
   * @param {InputConfig} config
   * @return {Promise}
   */
  convertSpace<T>(config: InputConfig<T>): Promise<T> {
    checkConfigEmptyness(config)
    return lib.convertSpace(config)
  },
  /**
   * To RGB
   * 
   * @param {OutputConfig} config
   * @return {Promise}
   */
  toRGB<T>(config: OutputConfig<T>): Promise<T> {
    checkConfigEmptyness(config)
    return lib.toRGB(config)
  },
  /**
   * To XYZ
   * 
   * @param {OutputConfig} config 
   * @return {Promise}
   */
  toXYZ<T>(config: OutputConfig<T>): Promise<T> {
    checkConfigEmptyness(config)
    return lib.toXYZ(config)
  }
}