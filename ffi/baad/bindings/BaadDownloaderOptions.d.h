#ifndef BaadDownloaderOptions_D_H
#define BaadDownloaderOptions_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BaadDownloaderOptions {
  size_t limit;
  uint32_t retries;
  bool http1_only;
  size_t max_chunks_per_file;
  size_t max_concurrent_chunks;
  uint64_t chunk_threshold;
} BaadDownloaderOptions;

typedef struct BaadDownloaderOptions_option {union { BaadDownloaderOptions ok; }; bool is_ok; } BaadDownloaderOptions_option;



#endif // BaadDownloaderOptions_D_H
