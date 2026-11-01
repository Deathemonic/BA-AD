#ifndef BaadGlobalCatalog_H
#define BaadGlobalCatalog_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadCatalogUrl.d.h"
#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedBuildType.d.h"
#include "../../baad-shared/bindings/BaadSharedDownloads.d.h"
#include "../../baad-shared/bindings/BaadSharedGlobalCatalogData.d.h"
#include "../../baad-shared/bindings/BaadSharedPlatform.d.h"

#include "BaadGlobalCatalog.d.h"






typedef struct BaadGlobalCatalog_new_result {union {BaadGlobalCatalog* ok; BaadError* err;}; bool is_ok;} BaadGlobalCatalog_new_result;
BaadGlobalCatalog_new_result BaadGlobalCatalog_new(uint8_t category, BaadSharedPlatform platform, BaadSharedBuildType build_type);

typedef struct BaadGlobalCatalog_prepare_downloads_result {union {BaadSharedDownloads* ok; BaadError* err;}; bool is_ok;} BaadGlobalCatalog_prepare_downloads_result;
BaadGlobalCatalog_prepare_downloads_result BaadGlobalCatalog_prepare_downloads(const BaadGlobalCatalog* self);

typedef struct BaadGlobalCatalog_get_catalog_url_result {union {BaadCatalogUrl* ok; BaadError* err;}; bool is_ok;} BaadGlobalCatalog_get_catalog_url_result;
BaadGlobalCatalog_get_catalog_url_result BaadGlobalCatalog_get_catalog_url(const BaadGlobalCatalog* self, DiplomatStringView version);

typedef struct BaadGlobalCatalog_fetch_catalogs_result {union {BaadSharedGlobalCatalogData* ok; BaadError* err;}; bool is_ok;} BaadGlobalCatalog_fetch_catalogs_result;
BaadGlobalCatalog_fetch_catalogs_result BaadGlobalCatalog_fetch_catalogs(const BaadGlobalCatalog* self, DiplomatStringView url);

void BaadGlobalCatalog_destroy(BaadGlobalCatalog* self);





#endif // BaadGlobalCatalog_H
