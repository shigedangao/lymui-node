#ifndef factory_operation_h
#define factory_operation_h

#include <stdio.h>
#include <node_api.h>
#include <rgb.h>
#include <tint.h>
#include <shade.h>

/**
 * @brief Factory creating a set of Tint
 * @param env napi_env
 * @param rgb Rgb ptr
 * @return napi_value
 */
napi_value TintFactory(napi_env env, Rgb *rgb);

/**
 * @brief Factory creating a set of Shade
 * @param env napi_env
 * @param rgb Rgb ptr
 * @return napi_value
 */
napi_value ShadeFactory(napi_env env, Rgb *rgb);

#endif