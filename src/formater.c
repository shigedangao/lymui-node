#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "format_props.h"

const char* validator_props[SUPPORTED_FORMAT_LEN] = {
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

const char *supported_type[SUPPORTED_FORMAT_LEN] = {
    "hex",
    "hsl",
    "hsv",
    "cymk",
    "ycbcr",
    "yuv",
    "hwb",
    "tsl",
    "grayscale",
    "xyz",
    "lab",
    "lch",
    "llab",
    "luv",
    "argb",
    "Srgb",
    "xyy"
};

Validation *getValidationProps(char *str) {
    Validation *validate = malloc(sizeof(Validation));
    if (validate == NULL) {
        return NULL;
    }

    uint8_t idx = 0;
    size_t size = SUPPORTED_FORMAT_LEN;

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
