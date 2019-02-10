//
//  deserializer.c
//  lymui
//
//  Created by Marc Intha on 24/11/2018.
//  Copyright © 2018 Marc. All rights reserved.
//

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <node_api.h>
#include "deserializer.h"
#include "binding_util.h"
#include "binding_error.h"
#include "deserializer_opts.h"

BridgeObj *deserialize(napi_env env, napi_value obj) {
    // BridgeObject is use to transfer based information to the converter
    BridgeObj *br = malloc(sizeof(BridgeObj));
    if (br == NULL) {
        return NULL;
    }
    
    // Define the required param that the JS method take
    // See API on the converter_regular
    char *inputProps = "input:output";
    napi_value params[2];
    
    // We check if the JS Object has the required param
    if (!hasPropInJSObj(env, obj, inputProps)) {
        br->error = ARG_NB_ERR;
        return br;
    }
    
    // Base on the declared napi_value params[2]
    // We retrieve these required parameters
    getNamedPropArray(env, inputProps, obj, CONVERT_BASIC_LEN, params);
    
    // As these deserializer is only treating 'RGB' based color input
    // We check that the first object (input a JS Object)
    // contain the rgb object like so: {input: {r: 5, g: 10, b: 98}}
    if (!hasPropInJSObj(env, params[0], RGB_PROPS)) {
        br->error = ARG_TYPE_ERR;
        return br;
    }
    
    // We retrieve the type value
    char *type = getStringValue(env, params[1], MAX_LEN_TYPE);
    if (type == NULL) {
        br->error = CONVERT_ERR;
        return br;
    }

    // The validator will return the schema and the OType output
    // The OType output enum will be used by the converter
    // to trigger the proper ColorFormat<T> Factory method
    Validation *validator = getValidationProps(type);
    if (validator == NULL) {
        br->error = OTYPE_TYPE_ERR;
        return br;
    }

    if (validator->output == nil) {
        br->error = INVALID_TYPE;
        return br;
    }

    free(type);

    // set the struct
    br->color  = params[0];
    br->output = validator->output;
    br->error  = NULL;
    br->matrix = NULL;
    br->clamp  = 0.0;

    free(validator);

    // We retrieve the optionals values
    getProfileOpt(env, obj, br);
    getClampOpt(env, obj, br);
    getScaleOpt(env, obj, br);
    
    return br;
}
