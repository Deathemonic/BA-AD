#ifndef BaadSharedOverrideConnectionGroup_H
#define BaadSharedOverrideConnectionGroup_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedOverrideConnectionGroup.d.h"






DiplomatStringView BaadSharedOverrideConnectionGroup_name(const BaadSharedOverrideConnectionGroup* self);

DiplomatStringView BaadSharedOverrideConnectionGroup_addressables_catalog_url_root(const BaadSharedOverrideConnectionGroup* self);

void BaadSharedOverrideConnectionGroup_destroy(BaadSharedOverrideConnectionGroup* self);





#endif // BaadSharedOverrideConnectionGroup_H
