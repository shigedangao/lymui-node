//
//  deserializer_space.h
//  lymui
//
//  Created by Marc Intha on 24/11/2018.
//  Copyright © 2018 Marc. All rights reserved.
//

#ifndef deserializer_space_h
#define deserializer_space_h

#include <stdio.h>
#include <node_api.h>
#include "deserializer.h"
#include "format_props.h"

extern napi_env envglobal;

/**
 * @brief struct that will be used to calculate value
 */
// typedef struct BridgeSpaceObj {
//     napi_value color;
//     OType output;
//     char *error;
//     double clamp;
// } BridgeSpaceObj;

/**
 * @brief deserialize space color
 * @param env napi_env
 * @param obj napi_value
 * @return BridgeSpaceObj pointer
 */
BridgeObj *deserializeSpace(napi_env env, napi_value obj);

#endif /* deserializer_space_h */
