#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include "format_props.h"

// Not the best idea though
// As the type should be in the right order
// for OType enum, validator_props & supported_type
// /!\ Might consider a list ?
char* validator_props[SUPPORTED_FORMAT_LEN] = {
    "",
    CMYK_PROPS,
    HSL_PROPS,
    HSV_PROPS,
    YCBCR_PROPS,
    YUV_PROPS,
    HWB_PROPS,
    TSL_PROPS,
    GSCALE_PROPS,
    XYZ_PROPS,
    LAB_PROPS,
    LCH_PROPS,
    LUV_PROPS,
    ARGB_PROPS,
    SRGB_PROPS,
    LCHLAB_PROPS,
    XYY_PROPS
};

char *supported_type[SUPPORTED_FORMAT_LEN] = {
    "hex",
    "cymk",
    "hsl",
    "hsv",
    "ycbcr",
    "yuv",
    "hwb",
    "tsl",
    "grayscale",
    "xyz",
    "lab",
    "lch",
    "luv",
    "argb",
    "Srgb",
    "llab",
    "xyy"
};

Validation *getValidationProps(char *str) {
    Validation *validate = malloc(sizeof(Validation));
    if (validate == NULL) {
        return NULL;
    }

    if (str == NULL) {
        free(validate);
        return NULL;
    }

    uint8_t idx = 0;
    size_t size = SUPPORTED_FORMAT_LEN;
    validate->schema = NULL;
    validate->output = nil;

    if (strcmp(str, "tint")) {
        validate->output = tint;
        return validate;
    }

    if (strcmp(str, "shade")) {
        validate->output = shade;
        return validate;
    }

    while(idx < size) {
        if (!strcmp(str, supported_type[idx])) {
            OType t = idx;
            validate->output = t;
            validate->schema = validator_props[idx];
            idx = size + 1;
        }

        idx++;
    }

    return validate;
}
