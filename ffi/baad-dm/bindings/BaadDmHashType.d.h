#ifndef BaadDmHashType_D_H
#define BaadDmHashType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadDmHashType {
  BaadDmHashType_Md5 = 0,
  BaadDmHashType_Sha1 = 1,
  BaadDmHashType_Crc32 = 2,
  BaadDmHashType_Crc64 = 3,
} BaadDmHashType;

typedef struct BaadDmHashType_option {union { BaadDmHashType ok; }; bool is_ok; } BaadDmHashType_option;



#endif // BaadDmHashType_D_H
