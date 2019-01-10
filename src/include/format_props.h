//
//  format_props.h
//  lymui
//
//  Created by Marc Intha on 08/01/2019.
//  Copyright © 2018 Marc. All rights reserved.
//

#ifndef format_props_h
#define format_props_h

#define RGB_PROPS "r:g:b"
#define CMYK_PROPS "c:y:m:k"
#define HSL_PROPS "h:s:l"
#define HSV_PROPS "h:s:v"
#define YCBCR_PROPS "y:cb:cr"
#define YUV_PROPS "y:u:v"
#define HWB_PROPS "h:w:b"
#define TSL_PROPS "t:s:l"
#define GSCALE_PROPS "g"
#define XYZ_PROPS "x:y:z"
#define LAB_PROPS "l:a:b"
#define LCH_PROPS "l:c:h"
#define LUV_PROPS "l:u:v"
#define ARGB_PROPS "r:g:b"
#define SRGB_PROPS "r:g:b"
#define LCHLAB_PROPS "l:c:h"
#define XYY_PROPS "x:y:Y"

#define SUPPORTED_FORMAT_LEN 15

extern const char* validator_props[15];
extern const char* supported_type[15];

typedef enum Ouput {
  hex,
  hsl,
  hsv,
  cymk,
  ycbcr,
  yuv,
  hwb,
  tsl,
  grasycale,
  xyz,
  lab,
  lch,
  llab,
  luv,
  argb,
  Srgb,
  xyy
} OType;

typedef struct Validation {
  OType output;
  char *schema;
} Validation;

/**
 * @brief Get Validation Props
 * @param o OType
 * @return validation Validation
 */
Validation *getValidationProps(OType o);


#endif /* format_props_h */
