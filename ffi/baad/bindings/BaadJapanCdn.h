#ifndef BaadJapanCdn_H
#define BaadJapanCdn_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedBundlePatchPackInfo.d.h"
#include "../../baad-shared/bindings/BaadSharedJapanAddressable.d.h"
#include "../../baad-shared/bindings/BaadSharedMediaCatalog.d.h"
#include "../../baad-shared/bindings/BaadSharedPlatform.d.h"
#include "../../baad-shared/bindings/BaadSharedTableCatalog.d.h"

#include "BaadJapanCdn.d.h"






BaadJapanCdn* BaadJapanCdn_new(DiplomatStringView url, BaadSharedPlatform platform);

typedef struct BaadJapanCdn_fetch_assets_result {union {BaadSharedBundlePatchPackInfo* ok; BaadError* err;}; bool is_ok;} BaadJapanCdn_fetch_assets_result;
BaadJapanCdn_fetch_assets_result BaadJapanCdn_fetch_assets(const BaadJapanCdn* self);

typedef struct BaadJapanCdn_fetch_table_result {union {BaadSharedTableCatalog* ok; BaadError* err;}; bool is_ok;} BaadJapanCdn_fetch_table_result;
BaadJapanCdn_fetch_table_result BaadJapanCdn_fetch_table(const BaadJapanCdn* self);

typedef struct BaadJapanCdn_fetch_media_result {union {BaadSharedMediaCatalog* ok; BaadError* err;}; bool is_ok;} BaadJapanCdn_fetch_media_result;
BaadJapanCdn_fetch_media_result BaadJapanCdn_fetch_media(const BaadJapanCdn* self);

typedef struct BaadJapanCdn_fetch_addressable_result {union {BaadSharedJapanAddressable* ok; BaadError* err;}; bool is_ok;} BaadJapanCdn_fetch_addressable_result;
BaadJapanCdn_fetch_addressable_result BaadJapanCdn_fetch_addressable(DiplomatStringView url);

typedef struct BaadJapanCdn_extract_catalog_url_result {union {DiplomatStringView ok; BaadError* err;}; bool is_ok;} BaadJapanCdn_extract_catalog_url_result;
BaadJapanCdn_extract_catalog_url_result BaadJapanCdn_extract_catalog_url(const BaadSharedJapanAddressable* addressable);

void BaadJapanCdn_destroy(BaadJapanCdn* self);





#endif // BaadJapanCdn_H
