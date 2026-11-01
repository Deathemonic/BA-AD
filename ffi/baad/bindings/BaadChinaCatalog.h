#ifndef BaadChinaCatalog_H
#define BaadChinaCatalog_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedDownloads.d.h"
#include "../../baad-shared/bindings/BaadSharedPlatform.d.h"

#include "BaadChinaCatalog.d.h"






typedef struct BaadChinaCatalog_new_result {union {BaadChinaCatalog* ok; BaadError* err;}; bool is_ok;} BaadChinaCatalog_new_result;
BaadChinaCatalog_new_result BaadChinaCatalog_new(uint8_t category, BaadSharedPlatform platform);

typedef struct BaadChinaCatalog_prepare_downloads_result {union {BaadSharedDownloads* ok; BaadError* err;}; bool is_ok;} BaadChinaCatalog_prepare_downloads_result;
BaadChinaCatalog_prepare_downloads_result BaadChinaCatalog_prepare_downloads(const BaadChinaCatalog* self);

void BaadChinaCatalog_destroy(BaadChinaCatalog* self);





#endif // BaadChinaCatalog_H
