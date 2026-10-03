#ifndef BaadSharedBuildType_D_H
#define BaadSharedBuildType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadSharedBuildType {
  BaadSharedBuildType_Standard = 0,
  BaadSharedBuildType_Teen = 1,
} BaadSharedBuildType;

typedef struct BaadSharedBuildType_option {union { BaadSharedBuildType ok; }; bool is_ok; } BaadSharedBuildType_option;



#endif // BaadSharedBuildType_D_H
