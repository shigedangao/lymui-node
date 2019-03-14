//
//  factory_space.c
//  lymui
//
//  Created by Marc Intha on 13/03/2019.
//  Copyright © 2019 Marc. All rights reserved.
//

#include "factory_space.h"
#include <stdlib.h>
#include <node_api.h>
#include "binding_error.h"
#include "binding_util.h"
#include "factory_common.h"

napi_value XyzJSObjFactory(napi_env env, Rgb *rgb, char *matrix, double clamp) {
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
    
    Matrix m = getEnumFromStr(matrix);
    Xyz *xyz = getXyzFromRgb(rgb, m);
    
    if (xyz == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (xyz->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", xyz->error);
        return object;
    }
    
    double x = clampValue(xyz->x, clamp);
    double y = clampValue(xyz->y, clamp);
    double z = clampValue(xyz->z, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "x", &x);
    assignPropToJSObj(&data, env, numberDouble, "y", &y);
    assignPropToJSObj(&data, env, numberDouble, "z", &z);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(xyz);
    free(rgb);
    
    return object;
}

napi_value XyzJSObjFactoryNoInst(napi_env env, Xyz *xyz, double clamp) {
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
    
    if (xyz == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    double x = clampValue(xyz->x, clamp);
    double y = clampValue(xyz->y, clamp);
    double z = clampValue(xyz->z, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "x", &x);
    assignPropToJSObj(&data, env, numberDouble, "y", &y);
    assignPropToJSObj(&data, env, numberDouble, "z", &z);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(xyz);
    
    return object;
}

napi_value LabJSObjFactory(napi_env env, Xyz *xyz, double clamp) {
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
    
    Lab *lab = getLabFromXyz(xyz);
    if (lab == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (lab->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", lab->error);
        return object;
    }
    
    double l = clampValue(lab->l, clamp);
    double a = clampValue(lab->a, clamp);
    double b = clampValue(lab->b, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "l", &l);
    assignPropToJSObj(&data, env, numberDouble, "a", &a);
    assignPropToJSObj(&data, env, numberDouble, "b", &b);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(lab);
    free(xyz);
    
    return object;
}

napi_value LchJSObjFactory(napi_env env, Xyz *xyz, double clamp) {
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
    
    Lch *lch = getLchFromXyz(xyz);
    if (lch == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (lch->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", lch->error);
        return object;
    }
    
    double l = clampValue(lch->l, clamp);
    double c = clampValue(lch->c, clamp);
    double h = clampValue(lch->h, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "l", &l);
    assignPropToJSObj(&data, env, numberDouble, "c", &c);
    assignPropToJSObj(&data, env, numberDouble, "h", &h);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(lch);
    free(xyz);
    
    return object;
}

napi_value LchLabJSObjFactory(napi_env env, Xyz *xyz, double clamp) {
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
    
    LchLab *lchlab = getLchFromLab(xyz);
    if (lchlab == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (lchlab->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", lchlab->error);
        return object;
    }
    
    double l = clampValue(lchlab->l, clamp);
    double c = clampValue(lchlab->c, clamp);
    double h = clampValue(lchlab->h, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "l", &l);
    assignPropToJSObj(&data, env, numberDouble, "c", &c);
    assignPropToJSObj(&data, env, numberDouble, "h", &h);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(lchlab);
    free(xyz);
    
    return object;
}

napi_value LuvJSObjFactory(napi_env env, Xyz *xyz, double clamp) {
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
    
    Luv *luv = getLuvFromXyz(xyz);
    if (luv == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (luv->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", luv->error);
        return object;
    }
    
    double l = clampValue(luv->l, clamp);
    double u = clampValue(luv->u, clamp);
    double v = clampValue(luv->v, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "l", &l);
    assignPropToJSObj(&data, env, numberDouble, "u", &u);
    assignPropToJSObj(&data, env, numberDouble, "v", &v);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(luv);
    free(xyz);
    
    return object;
}

napi_value HclJSObjFactory(napi_env env, Xyz *xyz, double clamp) {
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

    Luv *luv = getLuvFromXyz(xyz);
    if (luv == NULL) {
        assignPropToJSObj(&object, env, string, "error", CONVERSION_ERR);
        return object;
    }

    if (luv->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", luv->error);
        free(luv);
        return object;
    }

    Hcl *hcl = getHclFromLuv(luv);
    if (hcl == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        free(luv);
        return object;
    }

    if (hcl->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", hcl->error);
        free(hcl);
        return object;
    }

    double h = clampValue(hcl->h, clamp);
    double c = clampValue(hcl->c, clamp);
    double l = clampValue(hcl->l, clamp);

    assignPropToJSObj(&data, env, numberDouble, "h", &h);
    assignPropToJSObj(&data, env, numberDouble, "c", &c);
    assignPropToJSObj(&data, env, numberDouble, "l", &l);
    
    assignJSObjtoJSObj(env, &object, data, "data");  

    free(xyz);
    free(hcl);

    return object;  
}

napi_value ArgbJSObjFactory(napi_env env, Xyz *xyz, double clamp) {
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
    
    Argb *argb = getARgbFromXyz(xyz);
    if (argb == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (argb->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", argb->error);
        return object;
    }
    
    double r = clampValue(argb->r, clamp);
    double g = clampValue(argb->g, clamp);
    double b = clampValue(argb->b, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "r", &r);
    assignPropToJSObj(&data, env, numberDouble, "g", &g);
    assignPropToJSObj(&data, env, numberDouble, "b", &b);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(argb);
    free(xyz);
    
    return object;
}

napi_value SrgbJSObjFactory(napi_env env, Xyz *xyz, double clamp) {
    napi_status status;
    napi_value object, data;
    
    status = napi_create_object(env, &object);
    if (status != napi_ok) {
        napi_throw_error(env, NULL, OBJ_MAKE_ERR);
        return NULL;
    }
    
    status = napi_create_object(env, &data);
    if (status != napi_ok) {
        return NULL;
    }
    
    SRgb *srgb = getSrgbFromXyz(xyz);
    if (srgb == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (srgb->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", srgb->error);
        return object;
    }
    
    double r = clampValue(srgb->r, clamp);
    double g = clampValue(srgb->g, clamp);
    double b = clampValue(srgb->b, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "r", &r);
    assignPropToJSObj(&data, env, numberDouble, "g", &g);
    assignPropToJSObj(&data, env, numberDouble, "b", &b);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(srgb);
    free(xyz);
    
    return object;
}

napi_value XyyJSObjFactory(napi_env env, Xyz *xyz, double clamp) {
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
    
    Xyy *xyy = getXyyFromXyz(xyz);
    if (xyy == NULL) {
        assignPropToJSObj(&object, env, string, "error", OBJ_MAKE_ERR);
        return object;
    }
    
    if (xyy->error != NULL) {
        assignPropToJSObj(&object, env, string, "error", xyy->error);
        return object;
    }
    
    double x  = clampValue(xyy->x, clamp);
    double yx = clampValue(xyy->y, clamp);
    double yy = clampValue(xyy->Y, clamp);
    
    assignPropToJSObj(&data, env, numberDouble, "x", &x);
    assignPropToJSObj(&data, env, numberDouble, "y", &yx);
    assignPropToJSObj(&data, env, numberDouble, "Y", &yy);
    
    assignJSObjtoJSObj(env, &object, data, "data");
    
    free(xyy);
    free(xyz);
    
    return object;
}
