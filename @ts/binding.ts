import { InputConfig, OutputConfig } from './config' 

/**
 * Lymui
 *    Interface to the Lymuilib
 */
export interface Lymui {
  /**
   * Convert Regular
   * 
   * @param config
   */
  convertRegular<T, U>(config: InputConfig<T>): Promise<U>;
  /**
   * Convert Space
   * 
   * @param config
   */
  convertSpace<T>(config: InputConfig<T>): Promise<T>;
  /**
   * To RGB
   * 
   * @param config
   */
  toRGB<T>(config: OutputConfig<T>): Promise<T>;
  /**
   * To Xyz
   * 
   * @param config 
   */
  toXYZ<T>(config: OutputConfig<T>): Promise<T>;
}

const liblymui: Lymui = require('../build/Release/lymuilib')

export default liblymui
