#ifndef factory_regular_h
#define factory_regular_h

#include <stdio.h>
#include <node_api.h>
#include "rgb.h"
#include "hex.h"
#include "cymk.h"
#include "ycbcr.h"
#include "hsl.h"
#include "hsv.h"
#include "yuv.h"
#include "hwb.h"
#include "tsl.h"
#include "grayscale.h"

/**
 * @brief Factory creating Rgb Object on the fly
 * @param env napi_env
 * @param rgb Rgb struct pointer
 * @return napi_value
 */
napi_value RgbJSObjFactory(napi_env env, Rgb *rgb);

/**
 * @brief Factory creating a Hex JS Object
 * @param env napi_env
 * @param rgb Rgb struct pointer
 * @return napi_env
 */
napi_value HexJSObjFactory(napi_env env, Rgb *rgb);

/**
 * @brief Factory creating Cymk JS Object
 * @param env napi_env
 * @param rgb Rgb struct pointer
 * @param clamp double
 * @return napi_value
 */
napi_value CymkJSObjFactory(napi_env env, Rgb *rgb, double clamp);

/**
 * @brief Factory creating Ycbcr Object
 * @param env napi_env
 * @param rgb Rgbs struct pointer
 * @return napi_value
 */
napi_value YcbcrJSObjFactory(napi_env env, Rgb *rgb);

/**
 * @brief Factory creating Hsl Object
 * @param env napi_env
 * @param rgb Rgb struct pointer
 * @param clamp double
 * @return napi_value
 */
napi_value HslJSObjFactory(napi_env env, Rgb *rgb, double clamp);

/**
 * @brief Creating HSV JS Object
 * @param env napi_env
 * @param rgb struct Rgb pointer
 * @param clamp double
 * @return napi_value
 */
napi_value HsvJSObjFactory(napi_env env, Rgb *rgb, double clamp);

/**
 * @brief Creating YUV JS Object
 * @param env napi_env
 * @param rgb *Rgb
 * @param clamp double
 * @return napi_value
 */
napi_value YuvJSObjFactory(napi_env env, Rgb *rgb, double clamp);

/**
 * @brief Creating HSV JS Object
 * @param env napi_env
 * @param rgb Rgb struct
 * @param clamp double
 * @return napi_value
 */
napi_value HwbJSObjFactory(napi_env env, Rgb *rgb, double clamp);

/**
 * @brief Creating TSL JS Object
 * @param env napi_env
 * @param rgb Rgb
 * @param clamp double
 * @return napi_value
 */ 
napi_value TslJSObjFactory(napi_env env, Rgb *rgb, double clamp);

/**
 * @brief Creating a Grayscale JS Object
 * @param env napi_env
 * @param rgb Rgb
 * @param matrix char*
 * @return napi_value
 */
napi_value GrayScaleJSObjFactory(napi_env env, Rgb *rgb, char *matrix);


#endif