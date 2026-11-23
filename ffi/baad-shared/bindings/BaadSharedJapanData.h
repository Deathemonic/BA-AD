#ifndef BaadSharedJapanData_H
#define BaadSharedJapanData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedJapanData.d.h"






DiplomatStringView BaadSharedJapanData_version(const BaadSharedJapanData* self);

DiplomatStringView BaadSharedJapanData_server_info_url(const BaadSharedJapanData* self);

DiplomatStringView BaadSharedJapanData_catalog_url(const BaadSharedJapanData* self);

DiplomatStringView BaadSharedJapanData_platform(const BaadSharedJapanData* self);

void BaadSharedJapanData_destroy(BaadSharedJapanData* self);





#endif // BaadSharedJapanData_H
