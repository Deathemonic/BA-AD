#ifndef BaadSharedResource_H
#define BaadSharedResource_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedResource.d.h"






DiplomatStringView BaadSharedResource_group(const BaadSharedResource* self);

DiplomatStringView BaadSharedResource_resource_path(const BaadSharedResource* self);

int64_t BaadSharedResource_resource_size(const BaadSharedResource* self);

DiplomatStringView BaadSharedResource_resource_hash(const BaadSharedResource* self);

void BaadSharedResource_destroy(BaadSharedResource* self);





#endif // BaadSharedResource_H
