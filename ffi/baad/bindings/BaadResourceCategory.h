#ifndef BaadResourceCategory_H
#define BaadResourceCategory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadResourceCategory.d.h"






uint8_t BaadResourceCategory_assets(void);

uint8_t BaadResourceCategory_tables(void);

uint8_t BaadResourceCategory_media(void);

uint8_t BaadResourceCategory_all(void);

void BaadResourceCategory_destroy(BaadResourceCategory* self);





#endif // BaadResourceCategory_H
