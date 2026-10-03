#ifndef BaadDmZipFileInfo_D_H
#define BaadDmZipFileInfo_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BaadDmZipFileInfo {
  uint16_t compression_method;
  uint64_t compressed_size;
  uint64_t uncompressed_size;
  uint64_t local_header_offset;
} BaadDmZipFileInfo;

typedef struct BaadDmZipFileInfo_option {union { BaadDmZipFileInfo ok; }; bool is_ok; } BaadDmZipFileInfo_option;



#endif // BaadDmZipFileInfo_D_H
