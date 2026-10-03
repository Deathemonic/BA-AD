#ifndef BaadUtilsAlignedLine_D_H
#define BaadUtilsAlignedLine_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadUtilsLogLevel.d.h"




typedef struct BaadUtilsAlignedLine {
  BaadUtilsLogLevel level;
  bool is_success;
  DiplomatStringView message;
  DiplomatStringView value;
  DiplomatStringView right;
  size_t width;
} BaadUtilsAlignedLine;

typedef struct BaadUtilsAlignedLine_option {union { BaadUtilsAlignedLine ok; }; bool is_ok; } BaadUtilsAlignedLine_option;



#endif // BaadUtilsAlignedLine_D_H
