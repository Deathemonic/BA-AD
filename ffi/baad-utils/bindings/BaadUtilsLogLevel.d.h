#ifndef BaadUtilsLogLevel_D_H
#define BaadUtilsLogLevel_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadUtilsLogLevel {
  BaadUtilsLogLevel_Trace = 0,
  BaadUtilsLogLevel_Debug = 1,
  BaadUtilsLogLevel_Info = 2,
  BaadUtilsLogLevel_Warn = 3,
  BaadUtilsLogLevel_Error = 4,
} BaadUtilsLogLevel;

typedef struct BaadUtilsLogLevel_option {union { BaadUtilsLogLevel ok; }; bool is_ok; } BaadUtilsLogLevel_option;



#endif // BaadUtilsLogLevel_D_H
