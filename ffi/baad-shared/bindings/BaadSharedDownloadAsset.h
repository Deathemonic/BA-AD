#ifndef BaadSharedDownloadAsset_H
#define BaadSharedDownloadAsset_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedHashKind.d.h"

#include "BaadSharedDownloadAsset.d.h"






DiplomatStringView BaadSharedDownloadAsset_url(const BaadSharedDownloadAsset* self);

DiplomatStringView BaadSharedDownloadAsset_path(const BaadSharedDownloadAsset* self);

BaadSharedHashKind BaadSharedDownloadAsset_hash_kind(const BaadSharedDownloadAsset* self);

void BaadSharedDownloadAsset_hash(const BaadSharedDownloadAsset* self, DiplomatWrite* write);

int64_t BaadSharedDownloadAsset_size(const BaadSharedDownloadAsset* self);

size_t BaadSharedDownloadAsset_bundle_files_len(const BaadSharedDownloadAsset* self);

typedef struct BaadSharedDownloadAsset_bundle_files_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedDownloadAsset_bundle_files_at_result;
BaadSharedDownloadAsset_bundle_files_at_result BaadSharedDownloadAsset_bundle_files_at(const BaadSharedDownloadAsset* self, size_t index);

void BaadSharedDownloadAsset_destroy(BaadSharedDownloadAsset* self);





#endif // BaadSharedDownloadAsset_H
