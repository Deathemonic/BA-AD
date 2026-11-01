#ifndef BaadSharedProgressUnit_D_H
#define BaadSharedProgressUnit_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadSharedProgressUnit {
  BaadSharedProgressUnit_Bytes = 0,
  BaadSharedProgressUnit_Count = 1,
} BaadSharedProgressUnit;

typedef struct BaadSharedProgressUnit_option {union { BaadSharedProgressUnit ok; }; bool is_ok; } BaadSharedProgressUnit_option;



#endif // BaadSharedProgressUnit_D_H
