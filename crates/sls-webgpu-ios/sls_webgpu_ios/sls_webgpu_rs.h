#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct sls_Point {
  int32_t x;
  int32_t y;
} sls_Point;

struct sls_Point make_point(int32_t x, int32_t y);
