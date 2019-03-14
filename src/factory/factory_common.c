//
//  factory_common.c
//  lymui
//
//  Created by Marc Intha on 13/03/2019.
//  Copyright © 2019 Marc. All rights reserved.
//

#include "factory_common.h"
#include "binding_error.h"
#include "binding_util.h"

napi_value BuildPromiseError(napi_env env, char *error) {
    napi_status status;
    napi_value object;
    
    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }
    
    assignPropToJSObj(&object, env, string, "err", error);
    
    return object;
}