#ifndef BaadSharedChinaData_H
#define BaadSharedChinaData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedChinaData.d.h"






DiplomatStringView BaadSharedChinaData_version(const BaadSharedChinaData* self);

DiplomatStringView BaadSharedChinaData_catalog_url(const BaadSharedChinaData* self);

DiplomatStringView BaadSharedChinaData_resource_version(const BaadSharedChinaData* self);

DiplomatStringView BaadSharedChinaData_table_version(const BaadSharedChinaData* self);

DiplomatStringView BaadSharedChinaData_media_version(const BaadSharedChinaData* self);

DiplomatStringView BaadSharedChinaData_platform(const BaadSharedChinaData* self);

void BaadSharedChinaData_destroy(BaadSharedChinaData* self);





#endif // BaadSharedChinaData_H
