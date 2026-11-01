#ifndef BaadUtilsJson_H
#define BaadUtilsJson_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"

#include "BaadUtilsJson.d.h"






typedef struct BaadUtilsJson_parse_result {union {BaadUtilsJson* ok; BaadError* err;}; bool is_ok;} BaadUtilsJson_parse_result;
BaadUtilsJson_parse_result BaadUtilsJson_parse(DiplomatStringView json);

typedef struct BaadUtilsJson_replace_result {union { BaadError* err;}; bool is_ok;} BaadUtilsJson_replace_result;
BaadUtilsJson_replace_result BaadUtilsJson_replace(BaadUtilsJson* self, DiplomatStringView json);

typedef struct BaadUtilsJson_write_result {union { BaadError* err;}; bool is_ok;} BaadUtilsJson_write_result;
BaadUtilsJson_write_result BaadUtilsJson_write(const BaadUtilsJson* self, DiplomatWrite* write);

void BaadUtilsJson_destroy(BaadUtilsJson* self);





#endif // BaadUtilsJson_H
