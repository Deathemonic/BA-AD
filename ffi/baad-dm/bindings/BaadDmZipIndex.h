#ifndef BaadDmZipIndex_H
#define BaadDmZipIndex_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadDmZipFileInfo.d.h"
#include "../../baad-shared/bindings/BaadSharedStrings.d.h"

#include "BaadDmZipIndex.d.h"






BaadSharedStrings* BaadDmZipIndex_names(const BaadDmZipIndex* self);

typedef struct BaadDmZipIndex_get_result {union {BaadDmZipFileInfo ok; }; bool is_ok;} BaadDmZipIndex_get_result;
BaadDmZipIndex_get_result BaadDmZipIndex_get(const BaadDmZipIndex* self, DiplomatStringView name);

void BaadDmZipIndex_destroy(BaadDmZipIndex* self);





#endif // BaadDmZipIndex_H
