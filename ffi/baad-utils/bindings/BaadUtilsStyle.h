#ifndef BaadUtilsStyle_H
#define BaadUtilsStyle_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"

#include "BaadUtilsStyle.d.h"






typedef struct BaadUtilsStyle_apply_result {union { BaadError* err;}; bool is_ok;} BaadUtilsStyle_apply_result;
BaadUtilsStyle_apply_result BaadUtilsStyle_apply(const BaadUtilsStyle* self, DiplomatStringView text, DiplomatWrite* write);

void BaadUtilsStyle_destroy(BaadUtilsStyle* self);





#endif // BaadUtilsStyle_H
