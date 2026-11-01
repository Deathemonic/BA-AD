#ifndef BaadResourceFilter_H
#define BaadResourceFilter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "BaadFilterMethod.d.h"

#include "BaadResourceFilter.d.h"






typedef struct BaadResourceFilter_new_result {union {BaadResourceFilter* ok; BaadError* err;}; bool is_ok;} BaadResourceFilter_new_result;
BaadResourceFilter_new_result BaadResourceFilter_new(DiplomatStringView pattern, BaadFilterMethod method);

bool BaadResourceFilter_matches(const BaadResourceFilter* self, DiplomatStringView path);

void BaadResourceFilter_destroy(BaadResourceFilter* self);





#endif // BaadResourceFilter_H
