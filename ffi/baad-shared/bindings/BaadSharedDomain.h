#ifndef BaadSharedDomain_H
#define BaadSharedDomain_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedDomain.d.h"






DiplomatStringView BaadSharedDomain_primary_cdn(const BaadSharedDomain* self);

DiplomatStringView BaadSharedDomain_back_up_cdn(const BaadSharedDomain* self);

void BaadSharedDomain_destroy(BaadSharedDomain* self);





#endif // BaadSharedDomain_H
