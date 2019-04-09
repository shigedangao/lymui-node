#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <node_api.h>
#include "normalizer_regular.h"
#include "binding_util.h"
#include "binding_error.h"
#include "deserializer_opts.h"

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
    uint8_t hex = 0;
    BridgeObj *br = malloc(sizeof(BridgeObj));
    if (br == NULL) {
        return NULL;
    }
    
    char *inputProps = "input:type";
    napi_value params[2];
    
    if (!hasPropInJSObj(env, obj, inputProps)) {
        br->error = ARG_NB_ERR;
        return br;
    }
    
    char *err = getNamedPropArray(env, inputProps, obj, CONVERT_BASIC_LEN, params);
    if (err != NULL) {
        br->error = err;
        return br;
    }

    char *type = getStringValue(env, params[1], MAX_LEN_TYPE);
    hex = isHex(type);

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

    br->color  = params[0];
    br->output = validator->output;
    br->error  = NULL;
    br->matrix = NULL;
    
    // Special case for the HEX format as it doesn't have any object
    // We test if the type is Hex and if correct we then forward to the converter
    if (hex) {
        return br;
    }

    // Unlike the deserializer. The normalizer is able
    // to take any type of color input
    // Therefore we pass a char 'schema' representing the type
    // of color we want to check against
    // e.g validator->schema = 'x:y:z'
    if (!hasPropInJSObj(env, params[0], validator->schema)) {
        br->error = ARG_TYPE_ERR;
        return br;
    }

    free(validator);
    getProfileOpt(env, obj, br);

    return br;
}
