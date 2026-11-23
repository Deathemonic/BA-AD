#ifndef BaadSharedChinaMediaType_D_H
#define BaadSharedChinaMediaType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadSharedChinaMediaType {
  BaadSharedChinaMediaType_None = 0,
  BaadSharedChinaMediaType_Ogg = 1,
  BaadSharedChinaMediaType_Mp4 = 2,
  BaadSharedChinaMediaType_Jpg = 3,
  BaadSharedChinaMediaType_Png = 4,
  BaadSharedChinaMediaType_Acb = 5,
  BaadSharedChinaMediaType_Awb = 6,
} BaadSharedChinaMediaType;

typedef struct BaadSharedChinaMediaType_option {union { BaadSharedChinaMediaType ok; }; bool is_ok; } BaadSharedChinaMediaType_option;



#endif // BaadSharedChinaMediaType_D_H
