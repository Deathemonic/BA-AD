#ifndef BaadSharedChinaMediaType_H
#define BaadSharedChinaMediaType_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedChinaMediaType.d.h"






typedef struct BaadSharedChinaMediaType_from_value_result {union {BaadSharedChinaMediaType ok; }; bool is_ok;} BaadSharedChinaMediaType_from_value_result;
BaadSharedChinaMediaType_from_value_result BaadSharedChinaMediaType_from_value(int32_t value);

void BaadSharedChinaMediaType_extension(BaadSharedChinaMediaType self, DiplomatWrite* write);





#endif // BaadSharedChinaMediaType_H
