#ifndef BaadSharedDownloads_H
#define BaadSharedDownloads_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"
#include "BaadSharedDownloadAsset.d.h"
#include "BaadSharedDownloadMedia.d.h"
#include "BaadSharedDownloadTable.d.h"
#include "BaadSharedHashKind.d.h"

#include "BaadSharedDownloads.d.h"






BaadSharedDownloads* BaadSharedDownloads_new(void);

void BaadSharedDownloads_append(BaadSharedDownloads* self, BaadSharedDownloads* other);

typedef struct BaadSharedDownloads_push_asset_result {union { BaadError* err;}; bool is_ok;} BaadSharedDownloads_push_asset_result;
BaadSharedDownloads_push_asset_result BaadSharedDownloads_push_asset(BaadSharedDownloads* self, DiplomatStringView url, DiplomatStringView path, BaadSharedHashKind hash_kind, DiplomatStringView hash, int64_t size, DiplomatStringsView files);

typedef struct BaadSharedDownloads_push_table_result {union { BaadError* err;}; bool is_ok;} BaadSharedDownloads_push_table_result;
BaadSharedDownloads_push_table_result BaadSharedDownloads_push_table(BaadSharedDownloads* self, DiplomatStringView url, DiplomatStringView path, BaadSharedHashKind hash_kind, DiplomatStringView hash, int64_t size, DiplomatStringsView files);

typedef struct BaadSharedDownloads_push_media_result {union { BaadError* err;}; bool is_ok;} BaadSharedDownloads_push_media_result;
BaadSharedDownloads_push_media_result BaadSharedDownloads_push_media(BaadSharedDownloads* self, DiplomatStringView url, DiplomatStringView path, BaadSharedHashKind hash_kind, DiplomatStringView hash, int64_t size);

size_t BaadSharedDownloads_assets_len(const BaadSharedDownloads* self);

const BaadSharedDownloadAsset* BaadSharedDownloads_assets_at(const BaadSharedDownloads* self, size_t index);

size_t BaadSharedDownloads_tables_len(const BaadSharedDownloads* self);

const BaadSharedDownloadTable* BaadSharedDownloads_tables_at(const BaadSharedDownloads* self, size_t index);

size_t BaadSharedDownloads_media_len(const BaadSharedDownloads* self);

const BaadSharedDownloadMedia* BaadSharedDownloads_media_at(const BaadSharedDownloads* self, size_t index);

void BaadSharedDownloads_destroy(BaadSharedDownloads* self);





#endif // BaadSharedDownloads_H
