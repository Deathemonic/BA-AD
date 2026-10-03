#ifndef BaadSharedCategory_H
#define BaadSharedCategory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedCategory.d.h"






DiplomatStringView BaadSharedCategory_lang(const BaadSharedCategory* self);

bool BaadSharedCategory_has_lang(const BaadSharedCategory* self);

DiplomatStringView BaadSharedCategory_texture_encode_type(const BaadSharedCategory* self);

bool BaadSharedCategory_has_texture_encode_type(const BaadSharedCategory* self);

DiplomatStringView BaadSharedCategory_texture_quality_level(const BaadSharedCategory* self);

bool BaadSharedCategory_has_texture_quality_level(const BaadSharedCategory* self);

size_t BaadSharedCategory_group_len(const BaadSharedCategory* self);

typedef struct BaadSharedCategory_group_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedCategory_group_at_result;
BaadSharedCategory_group_at_result BaadSharedCategory_group_at(const BaadSharedCategory* self, size_t index);

void BaadSharedCategory_destroy(BaadSharedCategory* self);





#endif // BaadSharedCategory_H
