#ifndef BaadFilterMethod_D_H
#define BaadFilterMethod_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadFilterMethod {
  BaadFilterMethod_Exact = 0,
  BaadFilterMethod_Contains = 1,
  BaadFilterMethod_Regex = 2,
  BaadFilterMethod_Fuzzy = 3,
  BaadFilterMethod_Glob = 4,
  BaadFilterMethod_ContainsIgnoreCase = 5,
  BaadFilterMethod_StartsWith = 6,
  BaadFilterMethod_EndsWith = 7,
} BaadFilterMethod;

typedef struct BaadFilterMethod_option {union { BaadFilterMethod ok; }; bool is_ok; } BaadFilterMethod_option;



#endif // BaadFilterMethod_D_H
