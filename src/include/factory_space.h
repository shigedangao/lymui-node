#ifndef factory_space_h
#define factory_space_h

#include <stdio.h>
#include <node_api.h>
#include <xyz.h>
#include <lab.h>
#include <lch.h>
#include <lchlab.h>
#include <luv.h>
#include <argb.h>
#include <srgb.h>
#include <xyy.h>

/**
 * @brief Creating Xyz JS Object
 * @param env napi_env
 * @param rgb *Rgb
 * @param clamp double
 * @return napi_value
 */
napi_value XyzJSObjFactory(napi_env env, Rgb *rgb, char *matrix, double clamp);

/**
 * @brief Create Xyz JS Object
 * @param env napi_env
 * @param xyz Xyz
 * @param clamp double
 * @return napi_value
 */
napi_value XyzJSObjFactoryNoInst(napi_env env, Xyz *xyz, double clamp);

/**
 * @brief Creating Lab JS Object
 * @param env napi_env
 * @param xyz * Xyz
 * @param clamp double
 * @return napi_value
 */
napi_value LabJSObjFactory(napi_env env, Xyz * xyz, double clamp);

/**
 * @brief Creating Lch JS Object
 * @param env napi_env
 * @param xyz * Xyz
 * @param clamp double
 * @return napi_value
 */
napi_value LchJSObjFactory(napi_env env, Xyz * xyz, double clamp);

/**
 * @brief Creating Lch Lab JS Object
 * @param env napi_env
 * @param xyz * Xyz
 * @param clamp double
 * @return napi_value
 */
napi_value LchLabJSObjFactory(napi_env env, Xyz * xyz, double clamp);

/**
 * @brief Creating Luv JS Object
 * @param env napi_env
 * @param xyz * Xyz
 * @param clamp double
 * @return napi_value
 */
napi_value LuvJSObjFactory(napi_env env, Xyz * xyz, double clamp);

/**
 * @brief Creating ARGB JS Object
 * @param env napi_env
 * @param xyz * Xyz
 * @param clamp double
 * @return napi_value
 */
napi_value ArgbJSObjFactory(napi_env env, Xyz *xyz, double clamp);

/**
 * @brief Creating Srgb JS Object
 * @param env napi_env
 * @param xyz * Xyz
 * @param clamp double
 * @return napi_value
 */
napi_value SrgbJSObjFactory(napi_env env, Xyz *xyz, double clamp);

/**
 * @brief Creating XYY JS Object
 * @param env napi_env
 * @param xyz Xyz
 * @param clamp double
 * @return napi_value
 */
napi_value XyyJSObjFactory(napi_env env, Xyz *xyz, double clamp);


#endif