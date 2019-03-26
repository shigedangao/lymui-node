//
//  factory_operation.c
//  lymui
//
//  Created by Marc Intha on 25/03/2019.
//  Copyright © 2019 Marc. All rights reserved.
//

#include "factory_operation.h"
#include "factory_common.h"
#include "binding_error.h"
#include "binding_util.h"

napi_value TintFactory(napi_env env, Rgb *rgb) {
    napi_status status;
    napi_value object;

    status = napi_create_object(env, &object);
    if (status != napi_ok) {
      return NULL;
    }

    Tint *tint = getTint(rgb);
    if (tint == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }

    if (tint->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", tint->error);
        return object;
    }

    napi_value array = BuildArrayObject(env, tint->tint);
    if (array == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }

    status = napi_set_named_property(env, object, "data", array);
    if (status != napi_ok) {
        assignPropToJSObj(&object, env, string, "error", ASSIGN_ERR);
        return object; 
    }

    return object;
}

napi_value ShadeFactory(napi_env env, Rgb *rgb) {
    napi_status status;
    napi_value object;

    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }

    Shade *shade = getShade(rgb);
    if (shade == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }

    if (shade->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", shade->error);
        return object;
    }

    napi_value array = BuildArrayObject(env, shade->shade);
    if (array == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }

    status = napi_set_named_property(env, object, "data", array);
    if (status != napi_ok) {
        assignPropToJSObj(&object, env, string, "error", ASSIGN_ERR);
        return object; 
    }

    return object;
}
