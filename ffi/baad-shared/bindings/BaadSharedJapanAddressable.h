#ifndef BaadSharedJapanAddressable_H
#define BaadSharedJapanAddressable_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedConnectionGroup.d.h"

#include "BaadSharedJapanAddressable.d.h"






size_t BaadSharedJapanAddressable_connection_groups_len(const BaadSharedJapanAddressable* self);

const BaadSharedConnectionGroup* BaadSharedJapanAddressable_connection_groups_at(const BaadSharedJapanAddressable* self, size_t index);

void BaadSharedJapanAddressable_destroy(BaadSharedJapanAddressable* self);





#endif // BaadSharedJapanAddressable_H
