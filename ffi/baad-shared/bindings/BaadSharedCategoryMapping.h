#ifndef BaadSharedCategoryMapping_H
#define BaadSharedCategoryMapping_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedCategoryMapping.d.h"






DiplomatStringView BaadSharedCategoryMapping_group(const BaadSharedCategoryMapping* self);

size_t BaadSharedCategoryMapping_paths_len(const BaadSharedCategoryMapping* self);

typedef struct BaadSharedCategoryMapping_paths_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedCategoryMapping_paths_at_result;
BaadSharedCategoryMapping_paths_at_result BaadSharedCategoryMapping_paths_at(const BaadSharedCategoryMapping* self, size_t index);

void BaadSharedCategoryMapping_destroy(BaadSharedCategoryMapping* self);





#endif // BaadSharedCategoryMapping_H
