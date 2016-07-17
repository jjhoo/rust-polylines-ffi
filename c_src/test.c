#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "polyline.h"

#define BUFSIZE 256

int main(void)
{
   points_t data;
   coord_t * points;
   // char * res;
   int rc;

   data.data = malloc(sizeof(coord_t) * 2);
   data.count = 2;
   points = (coord_t *)data.data;

   points[0].latitude = 10.0;
   points[0].longitude = 20.0;
   points[1].latitude = 3.0;
   points[1].longitude = 4.0;

   char buf[BUFSIZE];
   memset(buf, 0, BUFSIZE);
   // res = encode_coordinates_ffi2(data.data, 2);
   rc = encode_coordinates_ffi2(buf, BUFSIZE, data.data, 2);
   printf("%d %s\n", rc, buf);
   free(data.data);
   // drop_cstring(res);

   return 0;
}
