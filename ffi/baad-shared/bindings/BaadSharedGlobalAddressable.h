#ifndef BaadSharedGlobalAddressable_H
#define BaadSharedGlobalAddressable_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedGlobalPatch.d.h"

#include "BaadSharedGlobalAddressable.d.h"






DiplomatStringView BaadSharedGlobalAddressable_api_version(const BaadSharedGlobalAddressable* self);

DiplomatStringView BaadSharedGlobalAddressable_market_game_id(const BaadSharedGlobalAddressable* self);

DiplomatStringView BaadSharedGlobalAddressable_latest_build_version(const BaadSharedGlobalAddressable* self);

DiplomatStringView BaadSharedGlobalAddressable_latest_build_number(const BaadSharedGlobalAddressable* self);

DiplomatStringView BaadSharedGlobalAddressable_min_build_version(const BaadSharedGlobalAddressable* self);

DiplomatStringView BaadSharedGlobalAddressable_min_build_number(const BaadSharedGlobalAddressable* self);

const BaadSharedGlobalPatch* BaadSharedGlobalAddressable_patch(const BaadSharedGlobalAddressable* self);

void BaadSharedGlobalAddressable_destroy(BaadSharedGlobalAddressable* self);





#endif // BaadSharedGlobalAddressable_H
