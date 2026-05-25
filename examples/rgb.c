#include "../bindings.h"
#include <stdint.h>
#include <stdio.h>

int main(void) {
    uint8_t rgb[3] = {255, 0, 0};
    Lumens lumen = None;

    // convert any color that can be convert from an rgb compatible type to an rgb compatible type
    Color *cymk = get_color(rgb, Rgb, Cymk, lumen);
    double c = cymk->data->ptr[0];
    double y = cymk->data->ptr[1];
    double m = cymk->data->ptr[2];
    double k = cymk->data->ptr[3];

    printf("c: %f \n", c);
    printf("y: %f \n", y);
    printf("m: %f \n", m);
    printf("k: %f \n", k);

    Lumens D65 = D65;
    Color *xyz = get_color(rgb, Rgb, Xyz, D65);

    double _x = xyz->data->ptr[0];
    double _y = xyz->data->ptr[1];
    double _z = xyz->data->ptr[2];

    printf("x: %f \n", _x);
    printf("y: %f \n", _y);
    printf("z: %f \n", _z);

    Color *hex = get_color(rgb, Rgb, Hex, None);
    printf("hex: %s \n", hex->hex);

    Grayscale sc = Lightness;
    uint8_t *grayscale = get_grayscale(rgb, Rgb, sc);
    printf("grayscale: %u \n", *grayscale);

    GeneratorKind kind = Shade;
    Generator *shade = get_generator(rgb, Rgb, 0.1, kind);
    printf("shade for r: %d \n", shade->generated[1].r);
    printf("shade for g: %d \n", shade->generated[1].g);
    printf("shade for b: %d \n", shade->generated[1].b);

    char wronghex[] = "#&";
    Color *wrong = get_color(wronghex, Hex, Rgb, None);
    printf("wrong rgb: %s \n", wrong == NULL ? "true" : "false");
    Error err = get_error();
    printf("error: %d \n", err);

    // drop the color when you're done with it -> free the memory
    drop_color(*hex);
    drop_generator(shade);
}
