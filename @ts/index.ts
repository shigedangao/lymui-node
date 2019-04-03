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
   * From RGB <Type, Output>
   * 
   *   Types are available in inputs.ts
   *   Output refer to the output.js exported interface
   * @param {InputConfig} config
   * @return {Promise}
   */
  fromRGB<T, O>(config: InputConfig<T>): Promise<O> {
    checkConfigEmptyness(config)

    if (config.scale) {
      config.scale = config.scale.toLowerCase()
    }

    return lib.fromRGB(config)
  },
  /**
   * From XYZ
   * 
   * @param {InputConfig} config
   * @return {Promise}
   */
  fromXYZ<T, O>(config: InputConfig<T>): Promise<O> {
    checkConfigEmptyness(config)
    return lib.fromXYZ(config)
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