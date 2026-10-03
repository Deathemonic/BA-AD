#ifndef BaadSharedTablePatchPack_H
#define BaadSharedTablePatchPack_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedTableBundle.d.h"

#include "BaadSharedTablePatchPack.d.h"






DiplomatStringView BaadSharedTablePatchPack_name(const BaadSharedTablePatchPack* self);

int64_t BaadSharedTablePatchPack_size(const BaadSharedTablePatchPack* self);

int64_t BaadSharedTablePatchPack_crc(const BaadSharedTablePatchPack* self);

bool BaadSharedTablePatchPack_is_prologue(const BaadSharedTablePatchPack* self);

size_t BaadSharedTablePatchPack_bundle_files_len(const BaadSharedTablePatchPack* self);

const BaadSharedTableBundle* BaadSharedTablePatchPack_bundle_files_at(const BaadSharedTablePatchPack* self, size_t index);

void BaadSharedTablePatchPack_destroy(BaadSharedTablePatchPack* self);





#endif // BaadSharedTablePatchPack_H
