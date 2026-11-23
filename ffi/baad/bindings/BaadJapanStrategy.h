#ifndef BaadJapanStrategy_H
#define BaadJapanStrategy_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadSharedBundlePatchPackInfo.d.h"
#include "../../baad-shared/bindings/BaadSharedDownloads.d.h"
#include "../../baad-shared/bindings/BaadSharedMediaCatalog.d.h"
#include "../../baad-shared/bindings/BaadSharedPlatform.d.h"
#include "../../baad-shared/bindings/BaadSharedTableCatalog.d.h"

#include "BaadJapanStrategy.d.h"






BaadSharedDownloads* BaadJapanStrategy_build_asset_downloads(BaadSharedBundlePatchPackInfo* catalog, DiplomatStringView url, BaadSharedPlatform platform);

BaadSharedDownloads* BaadJapanStrategy_build_table_downloads(BaadSharedTableCatalog* catalog, DiplomatStringView url);

BaadSharedDownloads* BaadJapanStrategy_build_media_downloads(BaadSharedMediaCatalog* catalog, DiplomatStringView url, BaadSharedPlatform platform);

void BaadJapanStrategy_destroy(BaadJapanStrategy* self);





#endif // BaadJapanStrategy_H
