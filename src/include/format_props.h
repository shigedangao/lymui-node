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
#define HCL_PROPS "h:c:l"

#define SUPPORTED_FORMAT_LEN 18

extern char* validator_props[SUPPORTED_FORMAT_LEN];
extern char* supported_type[SUPPORTED_FORMAT_LEN];

typedef enum Ouput {
  hex,
  cymk,
  hsl,
  hsv,
  ycbcr,
  yuv,
  hwb,
  tsl,
  grasycale,
  hcl,
  xyz,
  lab,
  lch,
  luv,
  argb,
  Srgb,
  llab,
  xyy,
  nil
} OType;

typedef struct Validation {
  OType output;
  char *schema;
} Validation;

/**
 * @brief Get Validation Props
 * @param c char
 * @return validation Validation
 */
Validation *getValidationProps(char *c);


#endif /* format_props_h */
