#ifndef BaadSharedStrings_H
#define BaadSharedStrings_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedStrings.d.h"






bool BaadSharedStrings_is_empty(const BaadSharedStrings* self);

size_t BaadSharedStrings_len(const BaadSharedStrings* self);

typedef struct BaadSharedStrings_get_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedStrings_get_result;
BaadSharedStrings_get_result BaadSharedStrings_get(const BaadSharedStrings* self, size_t index);

void BaadSharedStrings_destroy(BaadSharedStrings* self);





#endif // BaadSharedStrings_H
