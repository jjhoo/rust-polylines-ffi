#ifndef POLYLINE_H
#define POLYLINE_H

#include <stdint.h>

typedef struct {
    double longitude;
    double latitude;
} coord_t;

void drop_cstring(void * str);

char * encode_coordinates_ffi(coord_t * points, size_t npoints);
int encode_coordinates_ffi2(char * dst, size_t n, coord_t * points, size_t npoints);

#endif
