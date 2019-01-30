/**
 * Config interface base
 */
export interface Config<T> {
  <T>(input: T): T;
  clamp?: Number;
}

/**
 * OutputConfig
 *  Config use by the methods to* which extends to the Config
 */
export interface OutputConfig<T> extends Config<T> {
  type: String;
  profile?: String;
}

/**
 * InputConfig
 *  Config use by the methods convert* which extends to the Config base
 */
export interface InputConfig<T> extends Config<T> {
  output: String;
  profile?: String;
  scale?: String;
}