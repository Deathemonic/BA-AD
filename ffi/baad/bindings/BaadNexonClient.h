#ifndef BaadNexonClient_H
#define BaadNexonClient_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedGlobalAddressable.d.h"
#include "../../baad-shared/bindings/BaadSharedGlobalCatalogData.d.h"
#include "../../baad-shared/bindings/BaadSharedMarketConfig.d.h"

#include "BaadNexonClient.d.h"






typedef struct BaadNexonClient_get_catalog_result {union {BaadSharedGlobalCatalogData* ok; BaadError* err;}; bool is_ok;} BaadNexonClient_get_catalog_result;
BaadNexonClient_get_catalog_result BaadNexonClient_get_catalog(DiplomatStringView url);

typedef struct BaadNexonClient_get_addressable_result {union {BaadSharedGlobalAddressable* ok; BaadError* err;}; bool is_ok;} BaadNexonClient_get_addressable_result;
BaadNexonClient_get_addressable_result BaadNexonClient_get_addressable(const BaadSharedMarketConfig* config, DiplomatStringView version, DiplomatStringView build_number);

void BaadNexonClient_destroy(BaadNexonClient* self);





#endif // BaadNexonClient_H
