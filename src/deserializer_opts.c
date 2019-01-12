//
//  deserializer_opts.c
//  lymui
//
//  Created by Marc Intha on 25/11/2018.
//  Copyright © 2018 Marc. All rights reserved.
//

#include "deserializer_opts.h"
#include <stdlib.h>
#include "binding_util.h"

OptField *getOptField(napi_env env, napi_value obj, char *field) {
    napi_status status;
    bool hasOptProp;
    napi_value optfield;
    
    OptField *opt = malloc(sizeof(OptField));
    if (opt == NULL) {
        return NULL;
    }
    
    status = napi_has_named_property(env, obj, field, &hasOptProp);
    if (status != napi_ok) {
        opt->has = false;
        return opt;
    }
    
    if (!hasOptProp) {
        opt->has = false;
        return opt;
    }
    
    status = napi_get_named_property(env, obj, field, &optfield);
    if (status != napi_ok) {
        opt->has = false;
        return opt;
    }
    
    opt->field = optfield;
    opt->has = true;
    
    return opt;
}

void getProfileOpt(napi_env env, napi_value obj, BridgeObj *br) {
    OptField *opt = getOptField(env, obj, "profile");
    if (opt == NULL) {
        return;
    }
    
    if (opt->has) {
        char *value = getStringValue(env, opt->field, MAX_LEN_TYPE);
        br->matrix = value;
    }

    free(opt);
}

void getClampOpt(napi_env env, napi_value obj, BridgeObj *br) {
    OptField *clamp = getOptField(env, obj, "clamp");
    if (clamp == NULL) {
        return;
    }
    
    if (clamp->has) {
        double clampValue = getDoubleValue(env, clamp->field);
        br->clamp = clampValue;
    }

    free(clamp);
}

void getScaleOpt(napi_env env, napi_value obj, BridgeObj *br) {
    OptField *scale = getOptField(env, obj, "scale");
    if (scale == NULL) {
        return;
    }

    if (scale->has) {
        char *value = getStringValue(env, scale->field, MAX_LEN_TYPE);
        br->matrix = value;
    }

    free(scale);
}

