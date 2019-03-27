//
//  factory_regular.c
//  lymui
//
//  Created by Marc Intha on 13/03/2019.
//  Copyright © 2019 Marc. All rights reserved.
//


#include "factory_regular.h"
#include <stdlib.h>
#include <node_api.h>
#include "binding_error.h"
#include "binding_util.h"
#include "factory_common.h"

napi_value RgbJSObjFactory(napi_env env, Rgb *rgb) {
    napi_status status;
    napi_value object, data;
    
    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }
    
    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }
    
    if (rgb == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (rgb->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", rgb->error);
        return object;
    }
    
    // assign the value
    assignPropToJSObj(&data, env, numberInt, "r", &rgb->r);
    assignPropToJSObj(&data, env, numberInt, "g", &rgb->g);
    assignPropToJSObj(&data, env, numberInt, "b", &rgb->b);
    
    // assign data to the object
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(rgb);
    
    return object;
}

napi_value HexJSObjFactory(napi_env env, Rgb *rgb) {
    napi_status status;
    napi_value object, data;
    
    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }
    
    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }
    
    char *hex = getHexFromRGB(rgb);
    if (hex == NULL) {
        assignPropToJSObj(&object, env, string, "error", BuildPromiseError(env, CONVERT_ERR));
        return object;
    }
    
    assignPropToJSObj(&data, env, string, "hex", hex);
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(hex);
    free(rgb);
    
    return object;
}

napi_value CymkJSObjFactory(napi_env env, Rgb *rgb, double clamp) {
    napi_status status;
    napi_value object, data;
    
    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }
    
    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }
    
    Cymk *cymk = getCymkFromRgb(rgb);
    if (cymk == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (cymk->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", cymk->error);
        return object;
    }

    double c = clampValue(cymk->c, clamp);
    double y = clampValue(cymk->y, clamp);
    double m = clampValue(cymk->m, clamp);
    double k = clampValue(cymk->k, clamp);
    
    // assign the cymk object
    assignPropToJSObj(&data, env, numberDouble, "c", &c);
    assignPropToJSObj(&data, env, numberDouble, "y", &y);
    assignPropToJSObj(&data, env, numberDouble, "m", &m);
    assignPropToJSObj(&data, env, numberDouble, "k", &k);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(cymk);
    free(rgb);
    
    return object;
}

napi_value YcbcrJSObjFactory(napi_env env, Rgb *rgb) {
    napi_status status;
    napi_value object, data;
    
    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }
    
    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }
    
    Ycbcr *ycb = getYcbcrFromRgb(rgb);
    if (ycb == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (ycb->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", ycb->error);
        return object;
    }
    
    assignPropToJSObj(&data, env, numberInt, "y", &ycb->y);
    assignPropToJSObj(&data, env, numberInt, "cb", &ycb->cb);
    assignPropToJSObj(&data, env, numberInt, "cr", &ycb->cr);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(ycb);
    free(rgb);
    
    return object;
}

napi_value HslJSObjFactory(napi_env env, Rgb *rgb, double clamp) {
    napi_status status;
    napi_value object, data;
    
    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }
    
    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }
    
    Hsl *hsl = getHslFromRgb(rgb);
    if (hsl == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (hsl->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", hsl->error);
        return object;
    }
    
    double h = clampValue(hsl->h, clamp);
    double s = clampValue(hsl->s, clamp);
    double l = clampValue(hsl->l, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "h", &h);
    assignPropToJSObj(&data, env, numberDouble, "s", &s);
    assignPropToJSObj(&data, env, numberDouble, "l", &l);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(hsl);
    free(rgb);
    
    return object;
}

napi_value HsvJSObjFactory(napi_env env, Rgb *rgb, double clamp) {
    napi_status status;
    napi_value object, data;
    
    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }
    
    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }
    
    Hsv *hsv = getHsvFromRgb(rgb);
    if (hsv == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (hsv->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", hsv->error);
        return object;
    }
    
    double h = clampValue(hsv->h, clamp);
    double s = clampValue(hsv->s, clamp);
    double v = clampValue(hsv->v, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "h", &h);
    assignPropToJSObj(&data, env, numberDouble, "s", &s);
    assignPropToJSObj(&data, env, numberDouble, "v", &v);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(hsv);
    free(rgb);
    
    return object;
}

napi_value YuvJSObjFactory(napi_env env, Rgb *rgb, double clamp) {
    napi_status status;
    napi_value object, data;
    
    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }
    
    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }
    
    Yuv *yuv = getYuvFromRgb(rgb);
    if (yuv == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (yuv->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", yuv->error);
        return object;
    }
    
    double y = clampValue(yuv->y, clamp);
    double u = clampValue(yuv->u, clamp);
    double v = clampValue(yuv->v, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "y", &y);
    assignPropToJSObj(&data, env, numberDouble, "u", &u);
    assignPropToJSObj(&data, env, numberDouble, "v", &v);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(yuv);
    free(rgb);
    
    return object;
}

napi_value HwbJSObjFactory(napi_env env, Rgb *rgb, double clamp) {
    napi_status status;
    napi_value object, data;

    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }
    
    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }

    Hwb *hwb = getHwbFromRgb(rgb);
    if (hwb == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }

    if (hwb->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", hwb->error);
        return object;
    }

    double h = clampValue(hwb->h, clamp);
    double w = clampValue(hwb->w, clamp);
    double b = clampValue(hwb->b, clamp);

    assignPropToJSObj(&data, env, numberDouble, "h", &h);
    assignPropToJSObj(&data, env, numberDouble, "w", &w);
    assignPropToJSObj(&data, env, numberDouble, "b", &b);
    
    assignJSObjtoJSObj(env, &object, data, "data");

    free(hwb);
    free(rgb);

    return object;
}

napi_value TslJSObjFactory(napi_env env, Rgb *rgb, double clamp) {
    napi_status status;
    napi_value object, data;

    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }

    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }

    Tsl *tsl = getTslFromRgb(rgb);
    if (tsl == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }

    if (tsl->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", tsl->error);
        return object;
    }

    double t = clampValue(tsl->t, clamp);
    double s = clampValue(tsl->s, clamp);
    double l = clampValue(tsl->l, clamp);

    assignPropToJSObj(&data, env, numberDouble, "t", &t);
    assignPropToJSObj(&data, env, numberDouble, "s", &s);
    assignPropToJSObj(&data, env, numberDouble, "l", &l);
    
    assignJSObjtoJSObj(env, &object, data, "data");

    free(tsl);
    free(rgb);

    return object;
}

napi_value GrayScaleJSObjFactory(napi_env env, Rgb *rgb, char *matrix) {
    napi_status status;
    napi_value object, data;

    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        return NULL;
    }

    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }

    Strategy strat = getScaleStrategyFromStr(matrix);
    Gray gray = getGrayScale(rgb, strat);

    assignPropToJSObj(&data, env, numberInt, "gray", &gray);
    assignJSObjtoJSObj(env, &object, data, "data");

    return object;
}