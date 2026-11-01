#ifndef BaadUtilsTerminalSize_D_H
#define BaadUtilsTerminalSize_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BaadUtilsTerminalSize {
  uint64_t width;
  uint64_t height;
} BaadUtilsTerminalSize;

typedef struct BaadUtilsTerminalSize_option {union { BaadUtilsTerminalSize ok; }; bool is_ok; } BaadUtilsTerminalSize_option;



#endif // BaadUtilsTerminalSize_D_H
