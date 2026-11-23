#ifndef BaadSharedHashKind_D_H
#define BaadSharedHashKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadSharedHashKind {
  BaadSharedHashKind_Crc = 0,
  BaadSharedHashKind_Md5 = 1,
} BaadSharedHashKind;

typedef struct BaadSharedHashKind_option {union { BaadSharedHashKind ok; }; bool is_ok; } BaadSharedHashKind_option;



#endif // BaadSharedHashKind_D_H
