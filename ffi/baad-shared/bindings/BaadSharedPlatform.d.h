#ifndef BaadSharedPlatform_D_H
#define BaadSharedPlatform_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadSharedPlatform {
  BaadSharedPlatform_Android = 0,
  BaadSharedPlatform_Ios = 1,
  BaadSharedPlatform_Windows = 2,
} BaadSharedPlatform;

typedef struct BaadSharedPlatform_option {union { BaadSharedPlatform ok; }; bool is_ok; } BaadSharedPlatform_option;



#endif // BaadSharedPlatform_D_H
