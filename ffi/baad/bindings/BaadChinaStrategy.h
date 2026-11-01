#ifndef BaadChinaStrategy_H
#define BaadChinaStrategy_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadSharedBundleCatalogCN.d.h"
#include "../../baad-shared/bindings/BaadSharedDownloads.d.h"
#include "../../baad-shared/bindings/BaadSharedMediaCatalogCN.d.h"
#include "../../baad-shared/bindings/BaadSharedPlatform.d.h"
#include "../../baad-shared/bindings/BaadSharedTableCatalogCN.d.h"

#include "BaadChinaStrategy.d.h"






BaadSharedDownloads* BaadChinaStrategy_build_asset_downloads(BaadSharedBundleCatalogCN* catalog, DiplomatStringView url, BaadSharedPlatform platform);

BaadSharedDownloads* BaadChinaStrategy_build_table_downloads(BaadSharedTableCatalogCN* catalog, DiplomatStringView url);

BaadSharedDownloads* BaadChinaStrategy_build_media_downloads(BaadSharedMediaCatalogCN* catalog, DiplomatStringView url);

void BaadChinaStrategy_destroy(BaadChinaStrategy* self);





#endif // BaadChinaStrategy_H
