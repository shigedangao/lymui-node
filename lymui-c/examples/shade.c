#include "../bindings.h"
#include <stdio.h>

int main(void) {
    double rgb[3] = {102, 170, 119};

    ColorMixture *shade = generate_shade_tint(rgb, Rgb, 0.1, true);
    for (size_t i = 0; i < shade->len; i++) {
        printf("shade rgb r: %hhu\n", shade->ptr[i].r);
        printf("shade rgb g: %hhu\n", shade->ptr[i].g);
        printf("shade rgb b: %hhu\n", shade->ptr[i].b);
    }

    drop_color_mixture(shade);

    ColorMixture *tint = generate_shade_tint(rgb, Rgb, 0.1, false);
    for (size_t i = 0; i < tint->len; i++) {
        printf("tint rgb r: %hhu\n", tint->ptr[i].r);
        printf("tint rgb g: %hhu\n", tint->ptr[i].g);
        printf("tint rgb b: %hhu\n", tint->ptr[i].b);
    }

    drop_color_mixture(tint);
}