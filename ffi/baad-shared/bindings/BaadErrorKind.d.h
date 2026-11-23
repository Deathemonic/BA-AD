#ifndef BaadErrorKind_D_H
#define BaadErrorKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BaadErrorKind {
  BaadErrorKind_InvalidArgument = 0,
  BaadErrorKind_Filter = 1,
  BaadErrorKind_Catalog = 2,
  BaadErrorKind_Download = 3,
  BaadErrorKind_File = 4,
  BaadErrorKind_Json = 5,
  BaadErrorKind_Network = 6,
  BaadErrorKind_Configuration = 7,
  BaadErrorKind_Runtime = 8,
} BaadErrorKind;

typedef struct BaadErrorKind_option {union { BaadErrorKind ok; }; bool is_ok; } BaadErrorKind_option;



#endif // BaadErrorKind_D_H
