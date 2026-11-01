#ifndef BaadSharedProgress_H
#define BaadSharedProgress_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedProgress.d.h"






BaadSharedProgress* BaadSharedProgress_start(DiplomatStringView id, DiplomatStringView label, bool count_unit, uint64_t total);

void BaadSharedProgress_advance(const BaadSharedProgress* self, uint64_t delta);

void BaadSharedProgress_set_total(const BaadSharedProgress* self, uint64_t total);

void BaadSharedProgress_finish(BaadSharedProgress* self);

void BaadSharedProgress_fail(BaadSharedProgress* self, DiplomatStringView reason);

void BaadSharedProgress_destroy(BaadSharedProgress* self);





#endif // BaadSharedProgress_H
