#ifndef factory_common_h
#define factory_common_h

#include <stdio.h>
#include <node_api.h>

/**
 * @brief Build Promise Error create an error
 * @param env napi_env
 * @param error char*
 * @return napi_value
 */
napi_value BuildPromiseError(napi_env env, char *error);

/**
 * @brief Build Array Object
 * @param env napi_env
 * @param arg void ptr
 * @return napi_value
 */
napi_value BuildArrayObject(napi_env env, void *arg);

#endif