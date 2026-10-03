#ifndef BaadGlobalStrategy_H
#define BaadGlobalStrategy_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadSharedDownloads.d.h"
#include "../../baad-shared/bindings/BaadSharedGlobalCatalogData.d.h"

#include "BaadGlobalStrategy.d.h"






BaadSharedDownloads* BaadGlobalStrategy_build_downloads(BaadSharedGlobalCatalogData* catalog, DiplomatStringView base_url, uint8_t category);

void BaadGlobalStrategy_destroy(BaadGlobalStrategy* self);





#endif // BaadGlobalStrategy_H
