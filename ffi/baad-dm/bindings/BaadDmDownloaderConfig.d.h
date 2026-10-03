#ifndef BaadDmDownloaderConfig_D_H
#define BaadDmDownloaderConfig_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BaadDmDownloaderConfig {
  size_t concurrent_downloads;
  size_t max_chunks_per_file;
  size_t max_concurrent_chunks;
  uint64_t chunk_threshold;
  uint32_t retries;
  bool resumable;
  bool overwrite;
  bool http1_only;
} BaadDmDownloaderConfig;

typedef struct BaadDmDownloaderConfig_option {union { BaadDmDownloaderConfig ok; }; bool is_ok; } BaadDmDownloaderConfig_option;



#endif // BaadDmDownloaderConfig_D_H
