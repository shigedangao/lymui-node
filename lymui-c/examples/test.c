#include "../my_header.h"
#include <stdio.h>

int main(void) {
    char hex[] = "#ffffff";

    double hsl[3] = {237.0, 90.0, 19.0};
    double rgb[3] = {5.0, 10.0, 95.0};

    AnyColor *hx = convert_color_from_rgb_compatible_color(hex, Hex, Hex);
    AnyColor *h = convert_color_from_rgb_compatible_color(hsl, Hsl, Rgb);
    AnyColor *cymk = convert_color_from_rgb_compatible_color(hex, Hex, Cymk);
    AnyColor *xyz = convert_color_from_xyz_compatible_color(rgb, RgbCompat, Xyz, D65);

    printf("%s\n", hx->hex);
    printf("%zu\n", h->slice->len);

    // loop through the rgb
    for (size_t i = 0; i < h->slice->len; i++) {
        printf("rgb: %f\n", h->slice->ptr[i]);
    }

    // loop through the cymk
    for (size_t i = 0; i < cymk->slice->len; i++) {
        printf("cymk %f\n", cymk->slice->ptr[i]);
    }

    for (size_t i = 0; i < xyz->slice->len; i++) {
        printf("xyz %f\n", xyz->slice->ptr[i]);
    }

    // let's try to drop a value
    drop_any_color(hx);
    drop_any_color(h);
    drop_any_color(cymk);
    drop_any_color(xyz);

    printf("dropped hex\n");
}