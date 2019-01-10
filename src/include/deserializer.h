//
//  deserializer.h
//  lymui
//
//  Created by Marc Intha on 24/11/2018.
//  Copyright © 2018 Marc. All rights reserved.
//

#ifndef deserializer_h
#define deserializer_h

#include <stdio.h>
#include <node_api.h>
#include "format_props.h"

extern napi_env envglobal;

/**
 * @brief structure which will be used to calculate other value
 */
typedef struct BridgeObj {
    napi_value color;
    OType output;
    char *matrix;
    char *error;
    double clamp;
} BridgeObj;

/**
 * @brief deserialize
 * @param env napi_env
 * @param obj napi_value
 * @return BridgeObj pointer
 */
BridgeObj *deserialize(napi_env env, napi_value obj);

#endif /* deserializer_h */
