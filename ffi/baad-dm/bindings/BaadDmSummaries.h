#ifndef BaadDmSummaries_H
#define BaadDmSummaries_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadDmSummary.d.h"

#include "BaadDmSummaries.d.h"






bool BaadDmSummaries_is_empty(const BaadDmSummaries* self);

size_t BaadDmSummaries_len(const BaadDmSummaries* self);

const BaadDmSummary* BaadDmSummaries_get(const BaadDmSummaries* self, size_t index);

void BaadDmSummaries_destroy(BaadDmSummaries* self);





#endif // BaadDmSummaries_H
