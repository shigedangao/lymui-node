//
//  convert.c
//  lymui
//
//  Created by Marc Intha on 13/11/2018.
//  Copyright © 2018 Marc. All rights reserved.
//

#include <stdlib.h>
#include "convert_space.h"
#include <node_api.h>
#include "binding_error.h"
#include "bridge.h"
#include "deserializer_space.h"
#include "factory_space.h"
#include "factory_common.h"

static napi_value generateColorSpaceJSObj(napi_env env, BridgeObj *bridge) {
    Xyz *xyz = getXyzFromJSObj(env, bridge->color);
    if (xyz == NULL) {
        return NULL;
    }
    
    // Create the Object for each type
    switch(bridge->output) {
        case lab:
            return LabJSObjFactory(env, xyz, bridge->clamp);
        case lch:
            return LchJSObjFactory(env, xyz, bridge->clamp);
        case llab:
            return LchLabJSObjFactory(env, xyz, bridge->clamp);
        case argb:
            return ArgbJSObjFactory(env, xyz, bridge->clamp);
        case Srgb:
            return SrgbJSObjFactory(env, xyz, bridge->clamp);
        case luv:
            return LuvJSObjFactory(env, xyz, bridge->clamp);
        case xyy:
            return XyyJSObjFactory(env, xyz, bridge->clamp);
        default:
            return NULL;
    }
}

/**
 * JS API
 * const luv = async fromXYZ({
 *   input: {
 *     x: 0.50,
 *     y: 1.0,
 *     z: 0.98
 *   },
 *   output: 'lab',
 *   clamp: <optional> <double>
 * })
 *
 */
napi_value fromXYZ(napi_env env, napi_callback_info info) {
    napi_status status;
    size_t argc = 1;
    napi_value argv[1];
    napi_value promise;
    napi_value JSObject;
    napi_deferred def;
    
    status = napi_create_promise(env, &def, &promise);
    if (status != napi_ok) {
        napi_throw_error(env, NULL, PROMISE_ERR);
        return NULL;
    }
    
    status = napi_get_cb_info(env, info, &argc, argv, NULL, NULL);
    if (status != napi_ok) {
        napi_reject_deferred(env, def, BuildPromiseError(env, DESERIALIZE_ERR));
        return promise;
    }
    
    if (argc < 1) {
        napi_reject_deferred(env, def, BuildPromiseError(env, ARG_NB_ERR));
        return promise;
    }
    
    BridgeObj *bridge = deserializeSpace(env, argv[0]);
    if (bridge == NULL) {
        napi_reject_deferred(env, def, BuildPromiseError(env, CREATE_VALUE_ERR));
        return promise;
    }
    
    if (bridge->error != NULL) {
        napi_reject_deferred(env, def, BuildPromiseError(env, bridge->error));
        free(bridge);
        return promise;
    }
    
    JSObject = generateColorSpaceJSObj(env, bridge);
    if (JSObject == NULL) {
        napi_reject_deferred(env, def, BuildPromiseError(env, CREATE_VALUE_ERR));
        free(bridge);
        return promise;
    }

    free(bridge);
    napi_resolve_deferred(env, def, JSObject);
    
    return promise;
}
