#ifndef BaadSharedVersion_H
#define BaadSharedVersion_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedVersion.d.h"






typedef struct BaadSharedVersion_extract_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedVersion_extract_result;
BaadSharedVersion_extract_result BaadSharedVersion_extract(DiplomatStringView text);

void BaadSharedVersion_destroy(BaadSharedVersion* self);





#endif // BaadSharedVersion_H
