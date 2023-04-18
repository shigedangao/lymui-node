#include "../bindings.h"
#include <stdio.h>

int main(void) {
    double rgb[3] = {5.0, 10.0, 95.0};

    AnyColor *xyz = convert_color_from_xyz_compatible_color(rgb, RgbCompat, Xyz, D65);

    // You can also loop through the array with the provided length
    for (size_t i = 0; i < xyz->slice->len; i++) {
        printf("xyz %f\n", xyz->slice->ptr[i]);
    }

    // let's try to drop a value
    drop_any_color(xyz);
}