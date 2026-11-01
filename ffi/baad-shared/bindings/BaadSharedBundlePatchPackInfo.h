#ifndef BaadSharedBundlePatchPackInfo_H
#define BaadSharedBundlePatchPackInfo_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"
#include "BaadSharedBundlePatchPack.d.h"

#include "BaadSharedBundlePatchPackInfo.d.h"






typedef struct BaadSharedBundlePatchPackInfo_parse_json_result {union {BaadSharedBundlePatchPackInfo* ok; BaadError* err;}; bool is_ok;} BaadSharedBundlePatchPackInfo_parse_json_result;
BaadSharedBundlePatchPackInfo_parse_json_result BaadSharedBundlePatchPackInfo_parse_json(DiplomatStringView json);

DiplomatStringView BaadSharedBundlePatchPackInfo_milestone(const BaadSharedBundlePatchPackInfo* self);

int32_t BaadSharedBundlePatchPackInfo_patch_version(const BaadSharedBundlePatchPackInfo* self);

size_t BaadSharedBundlePatchPackInfo_full_patch_packs_len(const BaadSharedBundlePatchPackInfo* self);

const BaadSharedBundlePatchPack* BaadSharedBundlePatchPackInfo_full_patch_packs_at(const BaadSharedBundlePatchPackInfo* self, size_t index);

size_t BaadSharedBundlePatchPackInfo_update_packs_len(const BaadSharedBundlePatchPackInfo* self);

const BaadSharedBundlePatchPack* BaadSharedBundlePatchPackInfo_update_packs_at(const BaadSharedBundlePatchPackInfo* self, size_t index);

void BaadSharedBundlePatchPackInfo_destroy(BaadSharedBundlePatchPackInfo* self);





#endif // BaadSharedBundlePatchPackInfo_H
