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
#include <hsl.h>

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

napi_value BuildArrayObject(napi_env env, void *arg) {
    Hsl **data = (Hsl **)arg;
    napi_status status;
    napi_value array;

    status = napi_create_array(env, &array);
    if (status != napi_ok) {
        return NULL;
    }

    for (int idx = 0; idx < 10; idx++) {
        napi_value object;
        status = napi_create_object(env, &object);
        if (status != napi_ok) {
            return NULL;
        }

        assignPropToJSObj(&object, env, numberDouble, "h", &data[idx]->h);
        assignPropToJSObj(&object, env, numberDouble, "s", &data[idx]->s);
        assignPropToJSObj(&object, env, numberDouble, "l", &data[idx]->l);
        
        status = napi_set_element(env, array, idx, object);
        if (status != napi_ok) {
            return NULL;
        }
    }

    return array;
}