//
//  normalizer_regular.h
//  lymui
//
//  Created by Marc Intha on 10/01/2019.
//  Copyright © 2019 Marc. All rights reserved.
//

#ifndef normalizer_regular_h
#define normalizer_regular_h

#include <stdio.h>
#include <node_api.h>
#include "deserializer.h"

/**
 * @brief normalize the value of regular color for being prepare into being convet into RGB (broken english am sick).
 * @param env napi_env
 * @param obj napi_value
 * @return BridgeObj pointer
 */
BridgeObj *normalize(napi_env env, napi_value obj);


#endif /* normalizer_regular_h */