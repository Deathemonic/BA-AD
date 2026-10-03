#ifndef BaadUtilsBytes_H
#define BaadUtilsBytes_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadUtilsBytes.d.h"






DiplomatU8View BaadUtilsBytes_data(const BaadUtilsBytes* self);

void BaadUtilsBytes_destroy(BaadUtilsBytes* self);





#endif // BaadUtilsBytes_H
