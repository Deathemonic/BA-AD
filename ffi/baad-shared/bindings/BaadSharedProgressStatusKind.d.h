#ifndef BaadSharedProgressStatusKind_D_H
#define BaadSharedProgressStatusKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadSharedProgressStatusKind {
  BaadSharedProgressStatusKind_NotStarted = 0,
  BaadSharedProgressStatusKind_Success = 1,
  BaadSharedProgressStatusKind_Skipped = 2,
  BaadSharedProgressStatusKind_Failed = 3,
  BaadSharedProgressStatusKind_HashMismatch = 4,
} BaadSharedProgressStatusKind;

typedef struct BaadSharedProgressStatusKind_option {union { BaadSharedProgressStatusKind ok; }; bool is_ok; } BaadSharedProgressStatusKind_option;



#endif // BaadSharedProgressStatusKind_D_H
