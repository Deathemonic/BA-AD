#ifndef BaadJapanCatalog_H
#define BaadJapanCatalog_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadCatalogUrl.d.h"
#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedDownloads.d.h"
#include "../../baad-shared/bindings/BaadSharedPlatform.d.h"

#include "BaadJapanCatalog.d.h"






typedef struct BaadJapanCatalog_new_result {union {BaadJapanCatalog* ok; BaadError* err;}; bool is_ok;} BaadJapanCatalog_new_result;
BaadJapanCatalog_new_result BaadJapanCatalog_new(uint8_t category, BaadSharedPlatform platform);

typedef struct BaadJapanCatalog_prepare_downloads_result {union {BaadSharedDownloads* ok; BaadError* err;}; bool is_ok;} BaadJapanCatalog_prepare_downloads_result;
BaadJapanCatalog_prepare_downloads_result BaadJapanCatalog_prepare_downloads(const BaadJapanCatalog* self);

typedef struct BaadJapanCatalog_get_catalog_url_result {union {BaadCatalogUrl* ok; BaadError* err;}; bool is_ok;} BaadJapanCatalog_get_catalog_url_result;
BaadJapanCatalog_get_catalog_url_result BaadJapanCatalog_get_catalog_url(const BaadJapanCatalog* self);

void BaadJapanCatalog_destroy(BaadJapanCatalog* self);





#endif // BaadJapanCatalog_H
