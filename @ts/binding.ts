import { InputConfig, OutputConfig } from './config' 

/**
 * Lymui
 *    Interface to the Lymuilib
 */
export interface Lymui {
  /**
   * From RGB
   * 
   * @param config
   */
  fromRGB<T, O>(config: InputConfig<T>): Promise<O>;
  /**
   * From XYZ
   * 
   * @param config
   */
  fromXYZ<T, O>(config: InputConfig<T>): Promise<O>;
  /**
   * To RGB
   * 
   * @param config
   */
  toRGB<T, O>(config: OutputConfig<T>): Promise<O>;
  /**
   * To Xyz
   * 
   * @param config 
   */
  toXYZ<T, O>(config: OutputConfig<T>): Promise<O>;
}

const liblymui: Lymui = require('../build/Release/lymuilib')

export default liblymui
