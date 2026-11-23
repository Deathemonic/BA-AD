#ifndef BaadSharedGlobalData_H
#define BaadSharedGlobalData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedGlobalData.d.h"






DiplomatStringView BaadSharedGlobalData_version(const BaadSharedGlobalData* self);

DiplomatStringView BaadSharedGlobalData_catalog_url(const BaadSharedGlobalData* self);

DiplomatStringView BaadSharedGlobalData_platform(const BaadSharedGlobalData* self);

DiplomatStringView BaadSharedGlobalData_build_type(const BaadSharedGlobalData* self);

void BaadSharedGlobalData_destroy(BaadSharedGlobalData* self);





#endif // BaadSharedGlobalData_H
