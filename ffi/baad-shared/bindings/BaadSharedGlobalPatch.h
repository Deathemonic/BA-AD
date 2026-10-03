#ifndef BaadSharedGlobalPatch_H
#define BaadSharedGlobalPatch_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedStrings.d.h"

#include "BaadSharedGlobalPatch.d.h"






int32_t BaadSharedGlobalPatch_patch_version(const BaadSharedGlobalPatch* self);

DiplomatStringView BaadSharedGlobalPatch_resource_path(const BaadSharedGlobalPatch* self);

size_t BaadSharedGlobalPatch_bdiff_path_len(const BaadSharedGlobalPatch* self);

BaadSharedStrings* BaadSharedGlobalPatch_bdiff_path_keys(const BaadSharedGlobalPatch* self, size_t index);

typedef struct BaadSharedGlobalPatch_bdiff_path_get_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedGlobalPatch_bdiff_path_get_result;
BaadSharedGlobalPatch_bdiff_path_get_result BaadSharedGlobalPatch_bdiff_path_get(const BaadSharedGlobalPatch* self, size_t index, DiplomatStringView key);

void BaadSharedGlobalPatch_destroy(BaadSharedGlobalPatch* self);





#endif // BaadSharedGlobalPatch_H
