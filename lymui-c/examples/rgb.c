#include "../bindings.h"
#include <stdio.h>

int main(void) {
    char hex[] = "#059178";

    // convert any color that can be convert from an rgb compatible type to an rgb compatible type
    AnyColor *ycbcr = convert_color_from_rgb_compatible_color(hex, Hex, YCbCr);
    double y = ycbcr->slice->ptr[0];
    double cb = ycbcr->slice->ptr[1];
    double cr = ycbcr->slice->ptr[2];

    printf("y: %f \n", y);
    printf("cb: %f \n", cb);
    printf("cr: %f \n", cr);

    drop_any_color(ycbcr);

    // create an rgb
    double rgb[3] = {10.0, 20.0, 194.0};

    AnyColor *h = convert_color_from_rgb_compatible_color(rgb, Rgb, Hex);
    printf("hex: %s \n", h->hex);
}