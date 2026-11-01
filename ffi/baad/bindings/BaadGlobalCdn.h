#ifndef BaadGlobalCdn_H
#define BaadGlobalCdn_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedGlobalCatalogData.d.h"
#include "../../baad-shared/bindings/BaadSharedPlatform.d.h"

#include "BaadGlobalCdn.d.h"






BaadGlobalCdn* BaadGlobalCdn_new(DiplomatStringView url, BaadSharedPlatform platform);

typedef struct BaadGlobalCdn_fetch_result {union {BaadSharedGlobalCatalogData* ok; BaadError* err;}; bool is_ok;} BaadGlobalCdn_fetch_result;
BaadGlobalCdn_fetch_result BaadGlobalCdn_fetch(const BaadGlobalCdn* self);

typedef struct BaadGlobalCdn_derive_base_url_result {union {DiplomatStringView ok; }; bool is_ok;} BaadGlobalCdn_derive_base_url_result;
BaadGlobalCdn_derive_base_url_result BaadGlobalCdn_derive_base_url(DiplomatStringView path);

void BaadGlobalCdn_destroy(BaadGlobalCdn* self);





#endif // BaadGlobalCdn_H
