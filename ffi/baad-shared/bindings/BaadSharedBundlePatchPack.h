#ifndef BaadSharedBundlePatchPack_H
#define BaadSharedBundlePatchPack_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedBundleFile.d.h"

#include "BaadSharedBundlePatchPack.d.h"






DiplomatStringView BaadSharedBundlePatchPack_pack_name(const BaadSharedBundlePatchPack* self);

int64_t BaadSharedBundlePatchPack_pack_size(const BaadSharedBundlePatchPack* self);

int64_t BaadSharedBundlePatchPack_crc(const BaadSharedBundlePatchPack* self);

bool BaadSharedBundlePatchPack_is_prologue(const BaadSharedBundlePatchPack* self);

bool BaadSharedBundlePatchPack_is_split_download(const BaadSharedBundlePatchPack* self);

size_t BaadSharedBundlePatchPack_bundle_files_len(const BaadSharedBundlePatchPack* self);

const BaadSharedBundleFile* BaadSharedBundlePatchPack_bundle_files_at(const BaadSharedBundlePatchPack* self, size_t index);

void BaadSharedBundlePatchPack_destroy(BaadSharedBundlePatchPack* self);





#endif // BaadSharedBundlePatchPack_H
