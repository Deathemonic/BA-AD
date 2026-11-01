#ifndef BaadSharedApiData_H
#define BaadSharedApiData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedChinaData.d.h"
#include "BaadSharedGlobalData.d.h"
#include "BaadSharedJapanData.d.h"

#include "BaadSharedApiData.d.h"






const BaadSharedJapanData* BaadSharedApiData_japan(const BaadSharedApiData* self);

const BaadSharedGlobalData* BaadSharedApiData_global(const BaadSharedApiData* self);

const BaadSharedChinaData* BaadSharedApiData_china(const BaadSharedApiData* self);

void BaadSharedApiData_destroy(BaadSharedApiData* self);





#endif // BaadSharedApiData_H
