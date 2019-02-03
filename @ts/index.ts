import lib from './binding'
import { Config, InputConfig, OutputConfig } from './config'

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
   * Convert Regular <Type, Output>
   * 
   *   Types are available in inputs.ts
   *   Output refer to the output.js exported interface
   * @param {InputConfig} config
   * @return {Promise}
   */
  convertRegular<T, O>(config: InputConfig<T>): Promise<O> {
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
  convertSpace<T, O>(config: InputConfig<T>): Promise<O> {
    checkConfigEmptyness(config)
    return lib.convertSpace(config)
  },
  /**
   * To RGB
   * 
   * @param {OutputConfig} config
   * @return {Promise}
   */
  toRGB<T, O>(config: OutputConfig<T>): Promise<O> {
    checkConfigEmptyness(config)
    return lib.toRGB(config)
  },
  /**
   * To XYZ
   * 
   * @param {OutputConfig} config 
   * @return {Promise}
   */
  toXYZ<T, O>(config: OutputConfig<T>): Promise<O> {
    checkConfigEmptyness(config)
    return lib.toXYZ(config)
  }
}