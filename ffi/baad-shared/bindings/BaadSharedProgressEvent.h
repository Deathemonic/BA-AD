#ifndef BaadSharedProgressEvent_H
#define BaadSharedProgressEvent_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedProgressEventKind.d.h"
#include "BaadSharedProgressStatusKind.d.h"
#include "BaadSharedProgressUnit.d.h"

#include "BaadSharedProgressEvent.d.h"






BaadSharedProgressEventKind BaadSharedProgressEvent_kind(const BaadSharedProgressEvent* self);

DiplomatStringView BaadSharedProgressEvent_id(const BaadSharedProgressEvent* self);

DiplomatStringView BaadSharedProgressEvent_label(const BaadSharedProgressEvent* self);

uint64_t BaadSharedProgressEvent_current(const BaadSharedProgressEvent* self);

uint64_t BaadSharedProgressEvent_total(const BaadSharedProgressEvent* self);

BaadSharedProgressUnit BaadSharedProgressEvent_unit(const BaadSharedProgressEvent* self);

BaadSharedProgressStatusKind BaadSharedProgressEvent_status(const BaadSharedProgressEvent* self);

DiplomatStringView BaadSharedProgressEvent_reason(const BaadSharedProgressEvent* self);

void BaadSharedProgressEvent_destroy(BaadSharedProgressEvent* self);





#endif // BaadSharedProgressEvent_H
