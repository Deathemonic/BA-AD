#ifndef BaadResourcesAsset_H
#define BaadResourcesAsset_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadSharedGameFile.d.h"

#include "BaadResourcesAsset.d.h"






DiplomatStringView BaadResourcesAsset_url(const BaadResourcesAsset* self);

const BaadSharedGameFile* BaadResourcesAsset_file(const BaadResourcesAsset* self);

void BaadResourcesAsset_destroy(BaadResourcesAsset* self);





#endif // BaadResourcesAsset_H
