#ifndef BaadCatalogUrl_H
#define BaadCatalogUrl_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadCatalogUrl.d.h"






DiplomatStringView BaadCatalogUrl_url(const BaadCatalogUrl* self);

bool BaadCatalogUrl_up_to_date(const BaadCatalogUrl* self);

void BaadCatalogUrl_destroy(BaadCatalogUrl* self);





#endif // BaadCatalogUrl_H
