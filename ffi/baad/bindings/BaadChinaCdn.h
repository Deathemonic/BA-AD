#ifndef BaadChinaCdn_H
#define BaadChinaCdn_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedBundleCatalogCN.d.h"
#include "../../baad-shared/bindings/BaadSharedMediaCatalogCN.d.h"
#include "../../baad-shared/bindings/BaadSharedPlatform.d.h"
#include "../../baad-shared/bindings/BaadSharedTableCatalogCN.d.h"

#include "BaadChinaCdn.d.h"






BaadChinaCdn* BaadChinaCdn_new(DiplomatStringView url, BaadSharedPlatform platform, DiplomatStringView resource_version, DiplomatStringView table_version, DiplomatStringView media_version);

typedef struct BaadChinaCdn_fetch_assets_result {union {BaadSharedBundleCatalogCN* ok; BaadError* err;}; bool is_ok;} BaadChinaCdn_fetch_assets_result;
BaadChinaCdn_fetch_assets_result BaadChinaCdn_fetch_assets(const BaadChinaCdn* self);

typedef struct BaadChinaCdn_fetch_table_result {union {BaadSharedTableCatalogCN* ok; BaadError* err;}; bool is_ok;} BaadChinaCdn_fetch_table_result;
BaadChinaCdn_fetch_table_result BaadChinaCdn_fetch_table(const BaadChinaCdn* self);

typedef struct BaadChinaCdn_fetch_media_result {union {BaadSharedMediaCatalogCN* ok; BaadError* err;}; bool is_ok;} BaadChinaCdn_fetch_media_result;
BaadChinaCdn_fetch_media_result BaadChinaCdn_fetch_media(const BaadChinaCdn* self);

void BaadChinaCdn_destroy(BaadChinaCdn* self);





#endif // BaadChinaCdn_H
