#ifndef BaadUtilsLoggingConfig_D_H
#define BaadUtilsLoggingConfig_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BaadUtilsLoggingConfig {
  bool enable_console;
  bool enable_json;
  bool enable_debug;
  bool enable_trace;
  bool include_timestamps;
  bool enable_async_writer;
  bool enable_error_handler;
  bool enable_progress;
} BaadUtilsLoggingConfig;

typedef struct BaadUtilsLoggingConfig_option {union { BaadUtilsLoggingConfig ok; }; bool is_ok; } BaadUtilsLoggingConfig_option;



#endif // BaadUtilsLoggingConfig_D_H
