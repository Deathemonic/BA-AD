#ifndef BaadDmSummary_H
#define BaadDmSummary_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadSharedProgressStatusKind.d.h"

#include "BaadDmSummary.d.h"






DiplomatStringView BaadDmSummary_url(const BaadDmSummary* self);

DiplomatStringView BaadDmSummary_filename(const BaadDmSummary* self);

uint64_t BaadDmSummary_size(const BaadDmSummary* self);

uint16_t BaadDmSummary_status_code(const BaadDmSummary* self);

bool BaadDmSummary_resumable(const BaadDmSummary* self);

bool BaadDmSummary_is_success(const BaadDmSummary* self);

BaadSharedProgressStatusKind BaadDmSummary_status(const BaadDmSummary* self);

DiplomatStringView BaadDmSummary_reason(const BaadDmSummary* self);

void BaadDmSummary_destroy(BaadDmSummary* self);





#endif // BaadDmSummary_H
