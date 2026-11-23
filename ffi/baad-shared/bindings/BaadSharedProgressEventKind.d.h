#ifndef BaadSharedProgressEventKind_D_H
#define BaadSharedProgressEventKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadSharedProgressEventKind {
  BaadSharedProgressEventKind_Started = 0,
  BaadSharedProgressEventKind_Advance = 1,
  BaadSharedProgressEventKind_Completed = 2,
} BaadSharedProgressEventKind;

typedef struct BaadSharedProgressEventKind_option {union { BaadSharedProgressEventKind ok; }; bool is_ok; } BaadSharedProgressEventKind_option;



#endif // BaadSharedProgressEventKind_D_H
