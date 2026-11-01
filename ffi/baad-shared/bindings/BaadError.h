#ifndef BaadError_H
#define BaadError_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadErrorKind.d.h"

#include "BaadError.d.h"






void BaadError_message(const BaadError* self, DiplomatWrite* write);

BaadErrorKind BaadError_kind(const BaadError* self);

void BaadError_destroy(BaadError* self);





#endif // BaadError_H
