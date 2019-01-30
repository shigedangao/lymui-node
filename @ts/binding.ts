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
  convertRegular<T>(config: any): Promise<T>;
  /**
   * Convert Space
   * 
   * @param config
   */
  convertSpace<T>(config: any): Promise<T>;
  /**
   * To RGB
   * 
   * @param config
   */
  toRGB<T>(config: any): Promise<T>;
  /**
   * To Xyz
   * 
   * @param config 
   */
  toXYZ<T>(config: any): Promise<T>;
}

const liblymui: Lymui = require('./build/Release/lymuilib')

export default liblymui
