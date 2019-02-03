//
//  normalizer_rgb.c
//  lymui
//
//  Created by Marc Intha on 24/11/2018.
//  Copyright © 2018 Marc. All rights reserved.
//

#include "normalizer_rgb.h"
#include <node_api.h>
#include <string.h>
#include <stdlib.h>
#include "hex.h"
#include "binding_util.h"
#include "bridge.h"
#include "factory.h"

napi_value normalizeHex(napi_env env, napi_value color) {
    char *hex = getHEXFromJSObj(env, color);
    if (hex == NULL) {
        return NULL;
    }

    if (hex[3]) {
        Rgb *rgb = getRGBFromHex(hex);
        napi_value object = RgbJSObjFactory(env, rgb);
        return object;
    }

    char *smallHex = malloc(sizeof(char) * 4);
    if (smallHex == NULL) {
        return NULL;
    }

    snprintf(smallHex, 4, "%c%c%c", hex[0], hex[1], hex[2]);
    Rgb *tinyRgb = getRGBFromHex(smallHex);
    napi_value object = RgbJSObjFactory(env, tinyRgb);

    return object;
}

napi_value normalizeHsl(napi_env env, napi_value color) {
    Hsl *hsl = getHslFromJSObj(env, color);
    if (hsl == NULL) {
        return NULL;
    }
    
    Rgb *rgb = getRgbFromHsl(hsl);
    napi_value object = RgbJSObjFactory(env, rgb);
    return object;
}

napi_value normalizeHsv(napi_env env, napi_value color) {
    Hsv *hsv = getHsvFromJSObj(env, color);
    if (hsv == NULL) {
        return NULL;
    }
    
    Rgb *rgb = getRgbFromHsv(hsv);
    napi_value object = RgbJSObjFactory(env, rgb);
    return object;
}

napi_value normalizeCymk(napi_env env, napi_value color) {
    Cymk *cymk = getCymkFromJSObj(env, color);
    if (cymk == NULL) {
        return NULL;
    }
    
    Rgb *rgb = getRgbFromCymk(cymk);
    napi_value object = RgbJSObjFactory(env, rgb);
    
    return object;
}

napi_value normalizeYcbcr(napi_env env, napi_value color) {
    Ycbcr *ycbcr = getYcbcrFromJSObj(env, color);
    if (ycbcr == NULL) {
        return NULL;
    }
    
    Rgb *rgb = getRgbFromYcbcr(ycbcr);
    napi_value object = RgbJSObjFactory(env, rgb);
    
    return object;
}

napi_value normalizeYuv(napi_env env, napi_value color) {
    Yuv *yuv = getYuvFromJSObj(env, color);
    if (yuv == NULL) {
        return NULL;
    }
    
    Rgb *rgb = getRgbFromYuv(yuv);
    napi_value object = RgbJSObjFactory(env, rgb);
    
    return object;
}

napi_value normalizeHwb(napi_env env, napi_value color) {
    Hwb *hwb = getHwbFromJSObj(env, color);
    if (hwb == NULL) {
        return NULL;
    }
  
    Rgb *rgb = getRgbFromHwb(hwb);
    napi_value object = RgbJSObjFactory(env, rgb);

    return object;
}

napi_value normalizeTsl(napi_env env, napi_value color) {
    Tsl *tsl = getTslFromJSObj(env, color);
    if (tsl == NULL) {
        return NULL;
    }

    Rgb *rgb = getRgbFromTsl(tsl);
    napi_value object = RgbJSObjFactory(env, rgb);

    return object;
}

napi_value normalizeXyz(napi_env env, napi_value color, char *m) {
    Matrix mx = getEnumFromStr(m);
    Xyz *xyz = getXyzFromJSObj(env, color);
    if (xyz == NULL) {
        return NULL;
    }
    
    Rgb *rgb = getRgbFromXyz(xyz, mx);
    napi_value object = RgbJSObjFactory(env, rgb);
    
    return object;
}
