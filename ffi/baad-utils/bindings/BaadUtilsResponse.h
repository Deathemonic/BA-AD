#ifndef BaadUtilsResponse_H
#define BaadUtilsResponse_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "BaadUtilsBytes.d.h"

#include "BaadUtilsResponse.d.h"






uint64_t BaadUtilsResponse_content_length(const BaadUtilsResponse* self);

uint16_t BaadUtilsResponse_status(const BaadUtilsResponse* self);

typedef struct BaadUtilsResponse_body_result {union {BaadUtilsBytes* ok; BaadError* err;}; bool is_ok;} BaadUtilsResponse_body_result;
BaadUtilsResponse_body_result BaadUtilsResponse_body(BaadUtilsResponse* self);

void BaadUtilsResponse_destroy(BaadUtilsResponse* self);





#endif // BaadUtilsResponse_H
