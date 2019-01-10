//
//  normalizer_space.c
//  lymui
//
//  Created by Marc Intha on 10/01/2019.
//  Copyright © 2019 Marc. All rights reserved.
//

#include <stdio.h>
#include <stdlib.h>
#include "normalizer_space.h"
#include "format_props.h"
#include "binding_util.h"
#include "binding_error.h"
#include "deserializer_opts.h"

// Global varaible referencing the napi_env
napi_env envglobal;

/**
 * @brief Set BridgeSpace Opt Field
 * @param obj napi_value
 * @param br * BridgeObj
 * @void
 */
static void setBridgeOptField(napi_value obj, BridgeSpaceObj *br) {
    OptField *clamp = getOptField(envglobal, obj, "clamp");
    if (clamp == NULL) {
        return;
    }
    
    if (clamp->has) {
        double clampValue = getDoubleValue(envglobal, clamp->field);
        br->clamp = clampValue;
    }

    free(clamp);
}

BridgeSpaceObj *normalizeSpace(napi_env env, napi_value obj) {
    envglobal = env;
    BridgeSpaceObj *br = malloc(sizeof(BridgeSpaceObj));
    if (br == NULL) {
        return NULL;
    }
    
    char *inputProps = "input:type";
    napi_value params[2];
    
    if (!hasPropInJSObj(env, obj, inputProps, CONVERT_BASIC_LEN)) {
        br->error = ARG_NB_ERR;
        return br;
    }
    
    getNamedPropArray(env, inputProps, obj, CONVERT_BASIC_LEN, params);
    char *type = getStringValue(env, params[1], MAX_LEN_TYPE);
    if (type == NULL) {
        br->error = CONVERT_ERR;
        return br;
    }
    
    Validation *validator = getValidationProps(type);
    if (validator == NULL) {
      br->error = OTYPE_TYPE_ERR;
      return br;
    }
    
    // Get a validation character in order to check if the data is present
    if (!hasPropInJSObj(env, params[0], validator->schema, MIN_LEN_TYPE)) {
        br->error = ARG_TYPE_ERR;
        return br;
    }
    
    br->color  = params[0];
    br->output = validator->output;
    br->error  = NULL;
    br->clamp  = 0.0;
    
    free(validator);
    
    return br;
}