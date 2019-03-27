//
//  convert_xyz.c
//  lymui
//
//  Created by Marc Intha on 24/11/2018.
//  Copyright © 2018 Marc. All rights reserved.
//

#include <node_api.h>
#include <stdlib.h>
#include "convert_xyz.h"
#include "binding_error.h"
#include "factory_regular.h"
#include "factory_space.h"
#include "factory_common.h"
#include "normalizer_space.h"
#include "normalizer_xyz.h"

/**
 * @brief generate xyz object
 * @param env napi_env
 * @param bridge BridgeObj pointer
 * @return napi_value
 */
static napi_value generateXYZ(napi_env env, BridgeObj *bridge) {
    switch (bridge->output) {
        case lab:
            return normalizeLab(env, bridge->color, bridge->clamp);
        case lch:
            return normalizeLch(env, bridge->color, bridge->clamp);
        case llab:
            return normalizeLchLab(env, bridge->color, bridge->clamp);
        case luv:
            return normalizeLuv(env, bridge->color, bridge->clamp);
        case xyy:
            return normalizeXyy(env, bridge->color, bridge->clamp);
        case argb:
            return normalizeARgb(env, bridge->color, bridge->clamp);
        case Srgb:
            return normalizeSRgb(env, bridge->color, bridge->clamp);
        default:
            return NULL;
    }
}

/**
 * JS API
 *  const rgb = async toXYZ({
 *    input: {
 *      l: 13.9492,
 *      c: 40.0716,
 *      h: 276.8616
 *    },
 *    type: 'lch',
 *    clamp: <optional> double
 *  })
 *
 */
napi_value toXYZ(napi_env env, napi_callback_info info) {
    napi_status status;
    size_t argc = 1;
    napi_value argv[1];
    napi_value promise;
    napi_deferred def;
    napi_value JSObject;
    
    status = napi_create_promise(env, &def, &promise);
    if (status != napi_ok) {
        napi_throw_error(env, NULL, PROMISE_ERR);
        return NULL;
    }
    
    status = napi_get_cb_info(env, info, &argc, argv, NULL, NULL);
    if (status != napi_ok) {
        napi_reject_deferred(env, def, BuildPromiseError(env, CB_INFO_ERR));
        return promise;
    }
    
    if (argc < 1) {
        napi_reject_deferred(env, def, BuildPromiseError(env, ARG_NB_ERR));
        return promise;
    }
    
    BridgeObj *bridge = normalizeSpace(env, argv[0]);
    if (bridge == NULL) {
        napi_reject_deferred(env, def, BuildPromiseError(env, ALLOCATION_ERR));
        return promise;
    }
    
    if (bridge->error != NULL) {
        napi_reject_deferred(env, def, BuildPromiseError(env, bridge->error));
        free(bridge);
        return promise;
    }
    
    JSObject = generateXYZ(env, bridge);
    if (JSObject == NULL) {
        napi_reject_deferred(env, def, BuildPromiseError(env, CREATE_VALUE_ERR));
        free(bridge);
        return promise;
    }
    
    free(bridge);
    napi_resolve_deferred(env, def, JSObject);
    
    return promise;
}
