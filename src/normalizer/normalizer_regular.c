#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <node_api.h>
#include "normalizer_regular.h"
#include "binding_util.h"
#include "binding_error.h"
#include "deserializer_opts.h"

// Global varaible referencing the napi_env
napi_env envglobal;

/**
 * @brief Set Bridge Opt Field
 * @param obj napi_value obj
 * @param br * BridgeObj
 */
static void setBridgeOptField(napi_value obj, BridgeObj *br) {
    OptField *profile = getOptField(envglobal, obj, "profile");
    if (profile == NULL) {
        return;
    }
    
    if (profile->has) {
        char *value = getStringValue(envglobal, profile->field, MAX_LEN_TYPE);
        br->matrix = value;
    }

    free(profile);
}

/**
 * @brief Is Hex
 * @param type char array
 * @return uint8_t
 */
static uint8_t isHex(char *type) {
    if (type == NULL) {
        return 0;
    }
    
    if (!strcmp("hex", type)) {
        return 1;
    }
    
    return 0;
}

BridgeObj *normalize(napi_env env, napi_value obj) {
    envglobal = env;
    uint8_t hex = 0;
    BridgeObj *br = malloc(sizeof(BridgeObj));
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
    hex = isHex(type);

    Validation *validator = getValidationProps(type);
    if (validator == NULL) {
        br->error = OTYPE_TYPE_ERR;
        return br;
    }

    br->color  = params[0];
    br->output = validator->output;
    br->error  = NULL;
    br->matrix = NULL;
    
    // Special case for the HEX format as it doesn't have any object
    // We test if the type is Hex and if correct we then forward to the converter
    if (hex) {
        return br;
    }

    if (!hasPropInJSObj(env, params[0], validator->schema, MIN_LEN_TYPE)) {
        br->error = ARG_TYPE_ERR;
        return br;
    }

    setBridgeOptField(obj, br);
    free(validator);

    return br;
}
