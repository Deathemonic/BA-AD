#ifndef BaadSharedDownloadTable_H
#define BaadSharedDownloadTable_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedHashKind.d.h"

#include "BaadSharedDownloadTable.d.h"






DiplomatStringView BaadSharedDownloadTable_url(const BaadSharedDownloadTable* self);

DiplomatStringView BaadSharedDownloadTable_path(const BaadSharedDownloadTable* self);

BaadSharedHashKind BaadSharedDownloadTable_hash_kind(const BaadSharedDownloadTable* self);

void BaadSharedDownloadTable_hash(const BaadSharedDownloadTable* self, DiplomatWrite* write);

int64_t BaadSharedDownloadTable_size(const BaadSharedDownloadTable* self);

size_t BaadSharedDownloadTable_bundle_files_len(const BaadSharedDownloadTable* self);

typedef struct BaadSharedDownloadTable_bundle_files_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedDownloadTable_bundle_files_at_result;
BaadSharedDownloadTable_bundle_files_at_result BaadSharedDownloadTable_bundle_files_at(const BaadSharedDownloadTable* self, size_t index);

void BaadSharedDownloadTable_destroy(BaadSharedDownloadTable* self);





#endif // BaadSharedDownloadTable_H
