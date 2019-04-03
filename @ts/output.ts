import { Color, Hsl } from "./inputs"

/**
 * Output interface
 */
export interface Output {
  data: Color,
  error?: String  
}

/**
 * Output Operation Interface
 */
export interface OutputOperation {
  data: Hsl[],
  error?: String
}